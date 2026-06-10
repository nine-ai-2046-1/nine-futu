use chrono::Local;
use prost::Message;
use tokio::sync::mpsc;

use crate::data_parser::DataParser;
use crate::proto::qot_update_basic_qot::Response as QuoteResponse;
use crate::proto::qot_update_kl::Response as KlineResponse;
use crate::proto::qot_update_order_book::Response as OrderBookResponse;
use crate::proto::qot_update_ticker::Response as TickerResponse;
use crate::proto_layer::ProtoResponse;
use crate::storage::LiveStorage;
use crate::types::*;

pub const UPDATE_BASIC_QOT_PROTO_ID: u32 = 3005;
pub const UPDATE_KL_PROTO_ID: u32 = 3007;
pub const UPDATE_ORDER_BOOK_PROTO_ID: u32 = 3013;
pub const UPDATE_TICKER_PROTO_ID: u32 = 3011;
pub const UPDATE_RT_PROTO_ID: u32 = 3009;
pub const UPDATE_BROKER_PROTO_ID: u32 = 3015;

#[derive(Debug, Clone)]
pub enum PushData {
    Quote(StockQuote),
    Kline(KlineBar),
    OrderBook(OrderBook),
    Ticker(Ticker),
    RtData(RtData),
    Broker(BrokerQueue),
}

pub trait PushHandler: Send + Sync {
    fn on_quote(&self, data: StockQuote);
    fn on_kline(&self, data: KlineBar);
    fn on_order_book(&self, data: OrderBook);
    fn on_ticker(&self, data: Ticker);
    fn on_rt_data(&self, data: RtData);
    fn on_broker(&self, data: BrokerQueue);
}

pub struct PushDataHandler {
    rx: mpsc::UnboundedReceiver<ProtoResponse>,
    handlers: Vec<Box<dyn PushHandler>>,
    storage: LiveStorage,
    timeframe: String,
}

impl PushDataHandler {
    pub fn new(rx: mpsc::UnboundedReceiver<ProtoResponse>, timeframe: &str) -> Self {
        Self {
            rx,
            handlers: Vec::new(),
            storage: LiveStorage::new(),
            timeframe: timeframe.to_string(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn PushHandler>) {
        self.handlers.push(handler);
    }

    pub async fn run(&mut self) {
        let today = Local::now().format("%Y-%m-%d").to_string();

        while let Some(response) = self.rx.recv().await {
            let proto_id = response.header.proto_id;

            if let Some(push_data) = self.parse_push_data(proto_id, response) {
                // Get code and determine storage path
                let code = DataParser::extract_code(&push_data).to_string();
                let data_type = DataParser::data_type_name(&push_data).to_string();

                // Determine file path based on data type
                let path = if data_type == "kline" {
                    self.storage.get_kline_path(&code, &today, &self.timeframe)
                } else {
                    self.storage.get_data_path(&code, &today, &data_type)
                };

                // Convert to JSON and store
                if let Ok(json) = DataParser::to_json(&push_data) {
                    if let Err(e) = self.storage.append_line(&path, &json) {
                        eprintln!("Failed to write data: {}", e);
                    }
                }

                // Call handlers
                match &push_data {
                    PushData::Quote(d) => {
                        for handler in &self.handlers {
                            handler.on_quote(d.clone());
                        }
                    }
                    PushData::Kline(d) => {
                        for handler in &self.handlers {
                            handler.on_kline(d.clone());
                        }
                    }
                    PushData::OrderBook(d) => {
                        for handler in &self.handlers {
                            handler.on_order_book(d.clone());
                        }
                    }
                    PushData::Ticker(d) => {
                        for handler in &self.handlers {
                            handler.on_ticker(d.clone());
                        }
                    }
                    PushData::RtData(d) => {
                        for handler in &self.handlers {
                            handler.on_rt_data(d.clone());
                        }
                    }
                    PushData::Broker(d) => {
                        for handler in &self.handlers {
                            handler.on_broker(d.clone());
                        }
                    }
                }
            }
        }
    }

    fn parse_push_data(&self, proto_id: u32, response: ProtoResponse) -> Option<PushData> {
        match proto_id {
            UPDATE_BASIC_QOT_PROTO_ID => {
                if let Ok(rsp) = QuoteResponse::decode(response.body) {
                    if let Some(s2c) = rsp.s2c {
                        for basic_qot in s2c.basic_qot_list {
                            let security = basic_qot.security;
                            let market_str = match security.market {
                                1 => "HK",
                                2 => "US",
                                3 => "SH",
                                4 => "SZ",
                                5 => "SG",
                                6 => "JP",
                                19 => "CC",
                                _ => "??",
                            };
                            let code = format!("{}.{}", market_str, security.code);

                            return Some(PushData::Quote(StockQuote {
                                code,
                                last_done: basic_qot.cur_price,
                                prev_close_price: basic_qot.last_close_price,
                                open_price: basic_qot.open_price,
                                high_price: basic_qot.high_price,
                                low_price: basic_qot.low_price,
                                volume: basic_qot.volume,
                                turnover: basic_qot.turnover,
                                change_val: 0.0,
                                change_rate: 0.0,
                                amplitude: basic_qot.amplitude,
                                pe_ratio: 0.0,
                                yield_rate: 0.0,
                            }));
                        }
                    }
                }
            }
            UPDATE_KL_PROTO_ID => {
                if let Ok(rsp) = KlineResponse::decode(response.body) {
                    if let Some(s2c) = rsp.s2c {
                        let security = s2c.security;
                        let market_str = match security.market {
                            1 => "HK",
                            2 => "US",
                            3 => "SH",
                            4 => "SZ",
                            5 => "SG",
                            6 => "JP",
                            19 => "CC",
                            _ => "??",
                        };
                        let code = format!("{}.{}", market_str, security.code);

                        for kl in s2c.kl_list {
                            return Some(PushData::Kline(KlineBar {
                                code: code.clone(),
                                time_key: kl.time.clone(),
                                open: kl.open_price.unwrap_or(0.0),
                                close: kl.close_price.unwrap_or(0.0),
                                high: kl.high_price.unwrap_or(0.0),
                                low: kl.low_price.unwrap_or(0.0),
                                volume: kl.volume.unwrap_or(0),
                                turnover: kl.turnover.unwrap_or(0.0),
                                change_rate: 0.0,
                            }));
                        }
                    }
                }
            }
            UPDATE_ORDER_BOOK_PROTO_ID => {
                if let Ok(rsp) = OrderBookResponse::decode(response.body) {
                    if let Some(s2c) = rsp.s2c {
                        let security = s2c.security;
                        let market_str = match security.market {
                            1 => "HK",
                            2 => "US",
                            3 => "SH",
                            4 => "SZ",
                            5 => "SG",
                            6 => "JP",
                            19 => "CC",
                            _ => "??",
                        };
                        let code = format!("{}.{}", market_str, security.code);

                        let mut bid = Vec::new();
                        let mut ask = Vec::new();

                        for order in &s2c.order_book_bid_list {
                            bid.push(OrderBookEntry {
                                price: order.price,
                                volume: order.volume,
                            });
                        }

                        for order in &s2c.order_book_ask_list {
                            ask.push(OrderBookEntry {
                                price: order.price,
                                volume: order.volume,
                            });
                        }

                        return Some(PushData::OrderBook(OrderBook { code, bid, ask }));
                    }
                }
            }
            UPDATE_TICKER_PROTO_ID => {
                if let Ok(rsp) = TickerResponse::decode(response.body) {
                    if let Some(s2c) = rsp.s2c {
                        let security = s2c.security;
                        let market_str = match security.market {
                            1 => "HK",
                            2 => "US",
                            3 => "SH",
                            4 => "SZ",
                            5 => "SG",
                            6 => "JP",
                            19 => "CC",
                            _ => "??",
                        };
                        let code = format!("{}.{}", market_str, security.code);

                        for ticker in &s2c.ticker_list {
                            return Some(PushData::Ticker(Ticker {
                                code: code.clone(),
                                time: ticker.time.clone(),
                                price: ticker.price,
                                volume: ticker.volume,
                                turnover: ticker.turnover,
                                ticker_direction: ticker.dir,
                                sequence: ticker.sequence,
                            }));
                        }
                    }
                }
            }
            UPDATE_RT_PROTO_ID => {
                // RT data - parse if needed
            }
            UPDATE_BROKER_PROTO_ID => {
                // Broker queue - parse if needed
            }
            _ => {}
        }
        None
    }
}

pub struct DefaultPushHandler;

impl PushHandler for DefaultPushHandler {
    fn on_quote(&self, data: StockQuote) {
        println!("Quote: {} - {}", data.code, data.last_done);
    }

    fn on_kline(&self, data: KlineBar) {
        println!("Kline: {} - {}", data.code, data.time_key);
    }

    fn on_order_book(&self, data: OrderBook) {
        println!("OrderBook: {}", data.code);
    }

    fn on_ticker(&self, data: Ticker) {
        println!("Ticker: {} - {}", data.code, data.price);
    }

    fn on_rt_data(&self, data: RtData) {
        println!("RtData: {}", data.code);
    }

    fn on_broker(&self, data: BrokerQueue) {
        println!("Broker: {}", data.code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_data_handler_creation() {
        let (_tx, rx) = mpsc::unbounded_channel();
        let _handler = PushDataHandler::new(rx, "5m");
    }
}
