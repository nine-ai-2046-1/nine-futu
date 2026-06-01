use bytes::{Bytes, BytesMut};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

use crate::error::FutuError;
use crate::proto_layer::{
    FutuHeader, ProtoRequest, ProtoResponse, SerialManager, HEADER_SIZE,
    ALL_PUSH_IDS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    Start,
    Connecting,
    Connected,
    Ready,
    Closing,
    Closed,
}

pub struct FutuConnection {
    pub stream: Option<TcpStream>,
    pub state: ConnState,
    pub host: String,
    pub port: u16,
    pub serial_mgr: Arc<SerialManager>,
    pub recv_buf: BytesMut,
    pub push_tx: Option<mpsc::UnboundedSender<ProtoResponse>>,
    pub last_recv_time: std::time::Instant,
    pub keep_alive_interval: Duration,
    pub conn_timeout: Duration,
}

impl FutuConnection {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            stream: None,
            state: ConnState::Start,
            host: host.to_string(),
            port,
            serial_mgr: Arc::new(SerialManager::new()),
            recv_buf: BytesMut::with_capacity(1024 * 1024),
            push_tx: None,
            last_recv_time: std::time::Instant::now(),
            keep_alive_interval: Duration::from_secs(8),
            conn_timeout: Duration::from_secs(33),
        }
    }

    pub async fn connect(&mut self) -> Result<(), FutuError> {
        self.state = ConnState::Connecting;

        let addr = format!("{}:{}", self.host, self.port);
        let stream = TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;

        self.stream = Some(stream);
        self.state = ConnState::Connected;
        self.last_recv_time = std::time::Instant::now();

        Ok(())
    }

    pub fn set_push_channel(&mut self, tx: mpsc::UnboundedSender<ProtoResponse>) {
        self.push_tx = Some(tx);
    }

    pub fn set_state(&mut self, state: ConnState) {
        self.state = state;
    }

    pub fn state(&self) -> ConnState {
        self.state
    }

    pub fn is_connected(&self) -> bool {
        matches!(
            self.state,
            ConnState::Connected | ConnState::Ready
        )
    }

    pub async fn send(&mut self, proto_id: u32, body: Bytes) -> Result<u32, FutuError> {
        if !self.is_connected() {
            return Err(FutuError::NotConnected);
        }

        let serial_no = self.serial_mgr.next();
        let request = ProtoRequest::new(proto_id, serial_no, body);
        let data = request.to_bytes();

        if let Some(stream) = &mut self.stream {
            stream.write_all(&data).await?;
            Ok(serial_no)
        } else {
            Err(FutuError::NotConnected)
        }
    }

    pub async fn recv_loop(&mut self) -> Result<(), FutuError> {
        if let Some(stream) = &mut self.stream {
            let mut buf = [0u8; 1024 * 1024];

            loop {
                let n = stream.read(&mut buf).await?;
                if n == 0 {
                    self.state = ConnState::Closed;
                    return Err(FutuError::ConnectionLost);
                }

                self.recv_buf.extend_from_slice(&buf[..n]);
                self.last_recv_time = std::time::Instant::now();

                while self.recv_buf.len() >= HEADER_SIZE {
                    let header = FutuHeader::parse(&mut self.recv_buf)?;

                    if self.recv_buf.len() < header.body_len as usize {
                        break;
                    }

                    let body = self.recv_buf
                        .split_to(header.body_len as usize)
                        .freeze();

                    let response = ProtoResponse { header, body };

                    if is_push_proto_id(response.header.proto_id) {
                        if let Some(tx) = &self.push_tx {
                            let _ = tx.send(response);
                        }
                    } else {
                        // Request-response - handled elsewhere
                    }
                }
            }
        } else {
            Err(FutuError::NotConnected)
        }
    }

    pub async fn keep_alive_loop(&mut self) -> Result<(), FutuError> {
        let mut ticker = interval(self.keep_alive_interval);

        loop {
            ticker.tick().await;

            if !self.is_connected() {
                break;
            }

            if self.last_recv_time.elapsed() > self.conn_timeout {
                self.state = ConnState::Closed;
                return Err(FutuError::Timeout);
            }

            let keep_alive_proto_id = 1004;
            let body = Bytes::new();
            self.send(keep_alive_proto_id, body).await?;
        }

        Ok(())
    }

    pub async fn close(&mut self) {
        self.state = ConnState::Closing;

        if let Some(mut stream) = self.stream.take() {
            let _ = stream.shutdown().await;
        }

        self.state = ConnState::Closed;
    }
}

fn is_push_proto_id(proto_id: u32) -> bool {
    ALL_PUSH_IDS.contains(&proto_id)
}

pub struct ConnectionManager {
    connections: Vec<FutuConnection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    pub fn add(&mut self, conn: FutuConnection) {
        self.connections.push(conn);
    }

    pub fn get(&self, index: usize) -> Option<&FutuConnection> {
        self.connections.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut FutuConnection> {
        self.connections.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.connections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.connections.is_empty()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_manager() {
        let mut mgr = ConnectionManager::new();
        assert!(mgr.is_empty());

        let conn = FutuConnection::new("127.0.0.1", 11111);
        mgr.add(conn);
        assert_eq!(mgr.len(), 1);
    }
}
