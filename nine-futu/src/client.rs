use bytes::{Buf, BufMut, Bytes, BytesMut};
use prost::Message;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::Duration;

use crate::error::FutuError;
use crate::proto_layer::{FutuHeader, ProtoResponse, SerialManager, HEADER_SIZE, is_push_proto_id};

/// Futu OpenD API client
///
/// This client connects to a FutuOpenD gateway and provides methods
/// to query market data and manage subscriptions.
pub struct FutuClient {
    stream: TcpStream,
    serial_mgr: SerialManager,
    recv_buf: BytesMut,
    pending: std::collections::HashMap<u32, oneshot::Sender<ProtoResponse>>,
    push_tx: Option<mpsc::UnboundedSender<ProtoResponse>>,
    conn_id: u64,
    aes_key: String,
    last_recv_time: std::time::Instant,
    debug: bool,
}

impl From<prost::EncodeError> for FutuError {
    fn from(e: prost::EncodeError) -> Self {
        Self::ProtobufError(e.to_string())
    }
}

impl FutuClient {
    /// Connect to FutuOpenD gateway
    ///
    /// # Arguments
    /// * `host` - Gateway host (e.g., "127.0.0.1")
    /// * `port` - Gateway port (e.g., 11111)
    /// * `debug` - Enable debug output
    pub async fn connect(host: &str, port: u16, debug: bool) -> Result<Self, FutuError> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;

        let (push_tx, _push_rx) = mpsc::unbounded_channel();

        Ok(Self {
            stream,
            serial_mgr: SerialManager::new(),
            recv_buf: BytesMut::with_capacity(1024 * 1024),
            pending: std::collections::HashMap::new(),
            push_tx: Some(push_tx),
            conn_id: 0,
            aes_key: String::new(),
            last_recv_time: std::time::Instant::now(),
            debug,
        })
    }

    pub async fn init_connect(&mut self) -> Result<(), FutuError> {
        use crate::proto::init_connect::{Request, C2s};

        // Build InitConnect request
        let c2s = C2s {
            client_ver: 10406408, // 104.06408
            client_id: "nine-futu-rust".to_string(),
            recv_notify: Some(true),
            packet_enc_algo: Some(-1), // PacketEncAlgo_None
            push_proto_fmt: Some(0),   // Protobuf
            programming_language: Some("Rust".to_string()),
        };

        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;

        // Send request
        let serial_no = self.send_request(1001, body).await?;

        // Wait for response
        let response = self.wait_response(serial_no).await?;

        // Parse InitConnect response
        use crate::proto::init_connect::Response;
        let init_rsp = Response::decode(response.body)?;

        if init_rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: init_rsp.ret_type,
                msg: init_rsp.ret_msg.unwrap_or_default(),
            });
        }

        let s2c = init_rsp.s2c.ok_or(FutuError::PacketDataErr)?;
        self.conn_id = s2c.conn_id;
        self.aes_key = s2c.conn_aes_key;

        if self.debug {
            eprintln!("[DEBUG] Connected! conn_id={}, server_ver={}", s2c.conn_id, s2c.server_ver);
            eprintln!("[DEBUG] AES key: {}...", &self.aes_key[..8.min(self.aes_key.len())]);
        }

        Ok(())
    }

    /// Get market snapshot for one or more stocks
    ///
    /// # Arguments
    /// * `codes` - List of stock codes (e.g., ["HK.00700", "US.AAPL"])
    pub async fn get_market_snapshot(&mut self, codes: Vec<String>) -> Result<Vec<crate::types::SnapshotData>, FutuError> {
        use crate::proto::qot_get_security_snapshot::{Request, C2s};
        use crate::proto::qot_common::Security;

        // Parse codes into Security structs
        let mut security_list = Vec::new();
        for code in &codes {
            let parts: Vec<&str> = code.splitn(2, '.').collect();
            if parts.len() != 2 {
                return Err(FutuError::ParamErr(format!("Invalid code: {}", code)));
            }
            let market = match parts[0] {
                "HK" => 1,
                "US" => 2,
                "SH" => 3,
                "SZ" => 4,
                "SG" => 5,
                "JP" => 6,
                "CC" => 19,
                _ => return Err(FutuError::ParamErr(format!("Unknown market: {}", parts[0]))),
            };
            security_list.push(Security {
                market,
                code: parts[1].to_string(),
            });
        }

        let c2s = C2s {
            security_list,
        };

        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;

        // Send request (proto_id = 3203)
        let serial_no = self.send_request(3203, body).await?;

        // Wait for response
        let response = self.wait_response(serial_no).await?;

        // Parse response
        use crate::proto::qot_get_security_snapshot::Response;
        let snap_rsp = Response::decode(response.body)?;

        if snap_rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: snap_rsp.ret_type,
                msg: snap_rsp.ret_msg.unwrap_or_default(),
            });
        }

        let s2c = snap_rsp.s2c.ok_or(FutuError::PacketDataErr)?;

        // Convert to our types
        let mut results = Vec::new();
        for snap in s2c.snapshot_list {
            let basic = snap.basic;
            let security = basic.security;

            let market_prefix = match security.market {
                1 => "HK",
                2 => "US",
                3 => "SH",
                4 => "SZ",
                5 => "SG",
                6 => "JP",
                19 => "CC",
                _ => "??",
            };

            results.push(crate::types::SnapshotData {
                code: format!("{}.{}", market_prefix, security.code),
                name: basic.name.unwrap_or_default(),
                last_done: basic.cur_price,
                prev_close_price: basic.last_close_price,
                open_price: basic.open_price,
                high_price: basic.high_price,
                low_price: basic.low_price,
                volume: basic.volume,
                turnover: basic.turnover,
                market_cap: 0.0,
                pe_ratio: 0.0,
                pb_ratio: 0.0,
                yield_rate: 0.0,
            });
        }

        Ok(results)
    }

    /// Send a request to FutuOpenD
    ///
    /// # Arguments
    /// * `proto_id` - Protocol ID (e.g., 3203 for GetSecuritySnapshot)
    /// * `body` - Request body (protobuf encoded)
    ///
    /// # Returns
    /// Serial number for matching response
    async fn send_request(&mut self, proto_id: u32, body: Vec<u8>) -> Result<u32, FutuError> {
        let serial_no = self.serial_mgr.next();

        // Build header with SHA1 hash
        let header = FutuHeader::new(proto_id, serial_no, body.len() as u32, &body);

        // Build packet
        let mut packet = BytesMut::with_capacity(HEADER_SIZE + body.len());
        header.serialize(&mut packet);
        packet.put_slice(&body);

        // Send
        self.stream.write_all(&packet).await?;

        Ok(serial_no)
    }

    /// Wait for a response with matching serial number
    ///
    /// # Arguments
    /// * `serial_no` - Serial number to match
    ///
    /// # Returns
    /// Response from FutuOpenD
    async fn wait_response(&mut self, serial_no: u32) -> Result<ProtoResponse, FutuError> {
        let (tx, rx) = oneshot::channel();
        self.pending.insert(serial_no, tx);

        // Read responses until we get the one we want
        loop {
            // Read more data if buffer is small
            if self.recv_buf.len() < HEADER_SIZE {
                let mut buf = [0u8; 65536];
                let read_result = tokio::time::timeout(
                    Duration::from_secs(12),
                    self.stream.read(&mut buf),
                ).await;

                let n = match read_result {
                    Ok(Ok(n)) => n,
                    Ok(Err(_)) => return Err(FutuError::Timeout),
                    Err(_) => return Err(FutuError::Timeout),
                };

                if n == 0 {
                    return Err(FutuError::ConnectionLost);
                }

                self.recv_buf.extend_from_slice(&buf[..n]);
                self.last_recv_time = std::time::Instant::now();
            }

            // Try to parse header
            if self.recv_buf.len() >= HEADER_SIZE {
                let header = FutuHeader::parse(&mut self.recv_buf)?;

                // Check if we have enough body
                if self.recv_buf.len() < header.body_len as usize {
                    // Need more data
                    let mut body_buf = vec![0u8; header.body_len as usize - self.recv_buf.len()];
                    self.stream.read_exact(&mut body_buf).await?;
                    self.recv_buf.extend_from_slice(&body_buf);
                }

                let body = self.recv_buf.split_to(header.body_len as usize).freeze();

                let response = ProtoResponse { header: header.clone(), body };

                // Check if this is the response we're waiting for
                if header.serial_no == serial_no {
                    return Ok(response);
                }

                // Check if it's a push message
                if is_push_proto_id(header.proto_id) {
                    if let Some(tx) = &self.push_tx {
                        let _ = tx.send(response);
                    }
                }
            }
        }
    }

    pub fn conn_id(&self) -> u64 {
        self.conn_id
    }

    pub async fn get_cur_kline(&mut self, code: &str, ktype: &str, num: u32) -> Result<Vec<crate::types::KlineBar>, FutuError> {
        // First subscribe to the stock with the correct K-line type
        let sub_type = match ktype {
            "1m" => "K_1M",
            "3m" => "K_3M",
            "5m" => "K_5M",
            "15m" => "K_15M",
            "30m" => "K_30M",
            "60m" => "K_60M",
            "1d" => "K_DAY",
            "1w" => "K_WEEK",
            "1M" => "K_MON",
            _ => "K_DAY",
        };
        self.subscribe(code, sub_type).await?;

        // Get K-line data
        use crate::proto::qot_get_kl::{Request, C2s};
        use crate::proto::qot_common::Security;

        let parts: Vec<&str> = code.splitn(2, '.').collect();
        if parts.len() != 2 {
            return Err(FutuError::ParamErr(format!("Invalid code: {}", code)));
        }

        let market = match parts[0] {
            "HK" => 1,
            "US" => 2,
            "SH" => 3,
            "SZ" => 4,
            "SG" => 5,
            "JP" => 6,
            "CC" => 19,
            _ => return Err(FutuError::ParamErr(format!("Unknown market: {}", parts[0]))),
        };

        let kl_type = match ktype {
            "1m" => 1,
            "1d" => 2,
            "1w" => 3,
            "1M" => 4,
            "1Y" => 5,
            "5m" => 6,
            "15m" => 7,
            "30m" => 8,
            "60m" => 9,
            _ => return Err(FutuError::ParamErr(format!("Invalid ktype: {}", ktype))),
        };

        let c2s = C2s {
            rehab_type: 1, // QFQ (forward adjustment)
            kl_type,
            security: Security {
                market,
                code: parts[1].to_string(),
            },
            req_num: num as i32,
        };

        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;

        // Send request (proto_id = 3006 for GetKL)
        let serial_no = self.send_request(3006, body).await?;
        let response = self.wait_response(serial_no).await?;

        // Parse response
        use crate::proto::qot_get_kl::Response;
        let kl_rsp = Response::decode(response.body)?;

        if kl_rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: kl_rsp.ret_type,
                msg: kl_rsp.ret_msg.unwrap_or_default(),
            });
        }

        let s2c = kl_rsp.s2c.ok_or(FutuError::PacketDataErr)?;

        // Convert to our types
        let mut results = Vec::new();
        for kl in s2c.kl_list {
            results.push(crate::types::KlineBar {
                code: code.to_string(),
                time_key: kl.time,
                open: kl.open_price.unwrap_or(0.0),
                close: kl.close_price.unwrap_or(0.0),
                high: kl.high_price.unwrap_or(0.0),
                low: kl.low_price.unwrap_or(0.0),
                volume: kl.volume.unwrap_or(0),
                turnover: kl.turnover.unwrap_or(0.0),
                change_rate: 0.0,
            });
        }

        Ok(results)
    }

    /// Get historical K-line data
    ///
    /// # Arguments
    /// * `code` - Stock code (e.g., "HK.00700")
    /// * `ktype` - K-line type (e.g., "1d", "5m", "1m")
    /// * `start_date` - Start date (e.g., "2026-05-28")
    /// * `start_time` - Start time for minute klines (e.g., "09:30")
    /// * `end_date` - End date (e.g., "2026-05-28")
    /// * `end_time` - End time for minute klines (e.g., "16:00")
    pub async fn get_history_kline(
        &mut self,
        code: &str,
        ktype: &str,
        start_date: Option<&str>,
        start_time: Option<&str>,
        end_date: Option<&str>,
        end_time: Option<&str>,
    ) -> Result<Vec<crate::types::KlineBar>, FutuError> {
        use crate::proto::qot_request_history_kl::{Request, C2s};
        use crate::proto::qot_common::Security;

        let parts: Vec<&str> = code.splitn(2, '.').collect();
        if parts.len() != 2 {
            return Err(FutuError::ParamErr(format!("Invalid code: {}", code)));
        }

        let market = match parts[0] {
            "HK" => 1,
            "US" => 2,
            "SH" => 3,
            "SZ" => 4,
            "SG" => 5,
            "JP" => 6,
            "CC" => 19,
            _ => return Err(FutuError::ParamErr(format!("Unknown market: {}", parts[0]))),
        };

        let kl_type = match ktype {
            "1m" => 1,
            "1d" => 2,
            "1w" => 3,
            "1M" => 4,
            "1Y" => 5,
            "5m" => 6,
            "15m" => 7,
            "30m" => 8,
            "60m" => 9,
            _ => return Err(FutuError::ParamErr(format!("Invalid ktype: {}", ktype))),
        };

        // Build start and end strings
        let begin_time = match (start_date, start_time) {
            (Some(date), Some(time)) => format!("{} {}", date, time),
            (Some(date), None) => date.to_string(),
            _ => String::new(),
        };

        let end_time_str = match (end_date, end_time) {
            (Some(date), Some(time)) => format!("{} {}", date, time),
            (Some(date), None) => date.to_string(),
            _ => String::new(),
        };

        let c2s = C2s {
            rehab_type: 1, // QFQ
            kl_type,
            security: Security {
                market,
                code: parts[1].to_string(),
            },
            begin_time,
            end_time: end_time_str,
            max_ack_kl_num: Some(1000),
            ..Default::default()
        };

        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;

        // Send request (proto_id = 3103 for RequestHistoryKL)
        let serial_no = self.send_request(3103, body).await?;
        let response = self.wait_response(serial_no).await?;

        // Parse response
        use crate::proto::qot_request_history_kl::Response;
        let kl_rsp = Response::decode(response.body)?;

        if kl_rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: kl_rsp.ret_type,
                msg: kl_rsp.ret_msg.unwrap_or_default(),
            });
        }

        let s2c = kl_rsp.s2c.ok_or(FutuError::PacketDataErr)?;

        // Convert to our types
        let mut results = Vec::new();
        for kl in s2c.kl_list {
            results.push(crate::types::KlineBar {
                code: code.to_string(),
                time_key: kl.time,
                open: kl.open_price.unwrap_or(0.0),
                close: kl.close_price.unwrap_or(0.0),
                high: kl.high_price.unwrap_or(0.0),
                low: kl.low_price.unwrap_or(0.0),
                volume: kl.volume.unwrap_or(0),
                turnover: kl.turnover.unwrap_or(0.0),
                change_rate: 0.0,
            });
        }

        Ok(results)
    }

    /// Get historical K-line data with automatic pagination
    ///
    /// This method automatically handles pagination to fetch all data
    /// within the specified date range.
    ///
    /// # Arguments
    /// * `code` - Stock code (e.g., "HK.00700")
    /// * `ktype` - K-line type (e.g., "1d", "5m", "1m")
    /// * `start` - Start datetime (e.g., "2026-05-28" or "2026-05-28 09:30")
    /// * `end` - End datetime (e.g., "2026-05-28" or "2026-05-28 16:00")
    pub async fn get_history_kline_all(
        &mut self,
        code: &str,
        ktype: &str,
        start: &str,
        end: &str,
    ) -> Result<Vec<crate::types::KlineBar>, FutuError> {
        let mut all_data = Vec::new();
        let mut page_key: Option<Vec<u8>> = None;
        let mut page_count = 0;
        let max_pages = 50; // Safety limit

        loop {
            if page_count >= max_pages {
                if self.debug {
                    eprintln!("[DEBUG] Reached max pages: {}", max_pages);
                }
                break;
            }

            use crate::proto::qot_request_history_kl::{Request, C2s};
            use crate::proto::qot_common::Security;

            let parts: Vec<&str> = code.splitn(2, '.').collect();
            if parts.len() != 2 {
                return Err(FutuError::ParamErr(format!("Invalid code: {}", code)));
            }

            let market = match parts[0] {
                "HK" => 1,
                "US" => 2,
                "SH" => 3,
                "SZ" => 4,
                "SG" => 5,
                "JP" => 6,
                "CC" => 19,
                _ => return Err(FutuError::ParamErr(format!("Unknown market: {}", parts[0]))),
            };

            let kl_type = match ktype {
                "1m" => 1,
                "1d" => 2,
                "1w" => 3,
                "1M" => 4,
                "1Y" => 5,
                "5m" => 6,
                "15m" => 7,
                "30m" => 8,
                "60m" => 9,
                _ => return Err(FutuError::ParamErr(format!("Invalid ktype: {}", ktype))),
            };

            let c2s = C2s {
                rehab_type: 1, // QFQ
                kl_type,
                security: Security {
                    market,
                    code: parts[1].to_string(),
                },
                begin_time: start.to_string(),
                end_time: end.to_string(),
                max_ack_kl_num: Some(1000),
                next_req_key: page_key.clone(),
                ..Default::default()
            };

            let request = Request { c2s };
            let mut body = Vec::new();
            request.encode(&mut body)?;

            let serial_no = self.send_request(3103, body).await?;
            let response = self.wait_response(serial_no).await?;

            use crate::proto::qot_request_history_kl::Response;
            let kl_rsp = Response::decode(response.body)?;

            if kl_rsp.ret_type != 0 {
                return Err(FutuError::ProtoError {
                    ret_type: kl_rsp.ret_type,
                    msg: kl_rsp.ret_msg.unwrap_or_default(),
                });
            }

            let s2c = kl_rsp.s2c.ok_or(FutuError::PacketDataErr)?;

            if self.debug {
                eprintln!("[DEBUG] Response: security={}, name={:?}", 
                    s2c.security.code, s2c.name);
                eprintln!("[DEBUG] KL count: {}", s2c.kl_list.len());
                if let Some(ref key) = s2c.next_req_key {
                    eprintln!("[DEBUG] Has next page: true (key len={})", key.len());
                } else {
                    eprintln!("[DEBUG] Has next page: false");
                }
            }

            let mut count = 0;
            for kl in &s2c.kl_list {
                all_data.push(crate::types::KlineBar {
                    code: code.to_string(),
                    time_key: kl.time.clone(),
                    open: kl.open_price.unwrap_or(0.0),
                    close: kl.close_price.unwrap_or(0.0),
                    high: kl.high_price.unwrap_or(0.0),
                    low: kl.low_price.unwrap_or(0.0),
                    volume: kl.volume.unwrap_or(0),
                    turnover: kl.turnover.unwrap_or(0.0),
                    change_rate: 0.0,
                });
                count += 1;
            }

            if self.debug {
                eprintln!("[DEBUG] Page {}: got {} bars (total: {})", page_count + 1, count, all_data.len());
            }

            // Check if there's a next page
            page_key = s2c.next_req_key;
            if page_key.is_none() || count == 0 {
                break;
            }

            page_count += 1;
        }

        Ok(all_data)
    }

    /// Subscribe to real-time data for a stock
    ///
    /// # Arguments
    /// * `code` - Stock code (e.g., "HK.00700")
    /// * `sub_type` - Subscription type (e.g., "QUOTE", "ORDER_BOOK", "K_5M")
    async fn subscribe(&mut self, code: &str, sub_type: &str) -> Result<(), FutuError> {
        use crate::proto::qot_sub::{Request, C2s};
        use crate::proto::qot_common::Security;

        let parts: Vec<&str> = code.splitn(2, '.').collect();
        if parts.len() != 2 {
            return Err(FutuError::ParamErr(format!("Invalid code: {}", code)));
        }

        let market = match parts[0] {
            "HK" => 1,
            "US" => 2,
            "SH" => 3,
            "SZ" => 4,
            "SG" => 5,
            "JP" => 6,
            "CC" => 19,
            _ => return Err(FutuError::ParamErr(format!("Unknown market: {}", parts[0]))),
        };

        let sub_type_id = match sub_type {
            "QUOTE" => 1,
            "ORDER_BOOK" => 2,
            "TICKER" => 4,
            "RT_DATA" => 5,
            "K_DAY" => 6,
            "K_5M" => 7,
            "K_15M" => 8,
            "K_30M" => 9,
            "K_60M" => 10,
            "K_1M" => 11,
            "K_WEEK" => 12,
            "K_MON" => 13,
            _ => return Err(FutuError::ParamErr(format!("Invalid sub_type: {}", sub_type))),
        };

        let c2s = C2s {
            security_list: vec![Security {
                market,
                code: parts[1].to_string(),
            }],
            sub_type_list: vec![sub_type_id],
            is_sub_or_un_sub: true,
            is_first_push: Some(true),
            ..Default::default()
        };

        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;

        // Send request (proto_id = 3001 for Sub)
        let serial_no = self.send_request(3001, body).await?;
        let response = self.wait_response(serial_no).await?;

        // Parse response
        use crate::proto::qot_sub::Response;
        let sub_rsp = Response::decode(response.body)?;

        if sub_rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: sub_rsp.ret_type,
                msg: sub_rsp.ret_msg.unwrap_or_default(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running on localhost:11111
    async fn test_connect_to_opend() {
        let result = FutuClient::connect("127.0.0.1", 11111, false).await;
        assert!(result.is_ok(), "Failed to connect: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running
    async fn test_init_connect() {
        let mut client = FutuClient::connect("127.0.0.1", 11111, false).await.unwrap();
        let result = client.init_connect().await;
        assert!(result.is_ok(), "InitConnect failed: {:?}", result.err());
        assert!(client.conn_id > 0);
        assert!(!client.aes_key.is_empty());
    }

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running
    async fn test_get_market_snapshot() {
        let mut client = FutuClient::connect("127.0.0.1", 11111, false).await.unwrap();
        client.init_connect().await.unwrap();
        
        let result = client.get_market_snapshot(vec!["HK.00700".to_string()]).await;
        assert!(result.is_ok(), "get_market_snapshot failed: {:?}", result.err());
        
        let snapshots = result.unwrap();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].code, "HK.00700");
        assert!(snapshots[0].last_done > 0.0);
    }

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running
    async fn test_get_market_snapshot_multiple() {
        let mut client = FutuClient::connect("127.0.0.1", 11111, false).await.unwrap();
        client.init_connect().await.unwrap();
        
        let result = client.get_market_snapshot(vec![
            "HK.00700".to_string(),
            "HK.09988".to_string(),
        ]).await;
        assert!(result.is_ok(), "get_market_snapshot failed: {:?}", result.err());
        
        let snapshots = result.unwrap();
        assert_eq!(snapshots.len(), 2);
    }

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running
    async fn test_get_history_kline() {
        let mut client = FutuClient::connect("127.0.0.1", 11111, false).await.unwrap();
        client.init_connect().await.unwrap();
        
        let result = client.get_history_kline(
            "HK.00700",
            "1d",
            Some("2026-05-28"),
            None,
            Some("2026-05-29"),
            None,
        ).await;
        assert!(result.is_ok(), "get_history_kline failed: {:?}", result.err());
        
        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert_eq!(klines[0].code, "HK.00700");
    }

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running
    async fn test_subscribe() {
        let mut client = FutuClient::connect("127.0.0.1", 11111, false).await.unwrap();
        client.init_connect().await.unwrap();
        
        let result = client.subscribe("HK.00700", "QUOTE").await;
        assert!(result.is_ok(), "subscribe failed: {:?}", result.err());
    }

    #[tokio::test]
    #[ignore] // Requires FutuOpenD running
    async fn test_get_cur_kline() {
        let mut client = FutuClient::connect("127.0.0.1", 11111, false).await.unwrap();
        client.init_connect().await.unwrap();
        
        let result = client.get_cur_kline("HK.00700", "1d", 5).await;
        assert!(result.is_ok(), "get_cur_kline failed: {:?}", result.err());
        
        let klines = result.unwrap();
        assert!(!klines.is_empty());
        assert!(klines.len() <= 5);
    }
}
