use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use crate::proto_layer::ProtoResponse;
use crate::types::*;

pub const UPDATE_BASIC_QOT_PROTO_ID: u32 = 3005;
pub const UPDATE_KL_PROTO_ID: u32 = 3007;
pub const UPDATE_ORDER_BOOK_PROTO_ID: u32 = 3013;
pub const UPDATE_TICKER_PROTO_ID: u32 = 3011;

pub enum PushData {
    Quote(StockQuote),
    Kline(KlineBar),
    OrderBook(OrderBook),
    Ticker(Ticker),
}

pub trait PushHandler: Send + Sync {
    fn on_quote(&self, data: StockQuote);
    fn on_kline(&self, data: KlineBar);
    fn on_order_book(&self, data: OrderBook);
    fn on_ticker(&self, data: Ticker);
}

pub struct PushDataHandler {
    rx: mpsc::UnboundedReceiver<ProtoResponse>,
    handlers: Vec<Box<dyn PushHandler>>,
}

impl PushDataHandler {
    pub fn new(rx: mpsc::UnboundedReceiver<ProtoResponse>) -> Self {
        Self {
            rx,
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Box<dyn PushHandler>) {
        self.handlers.push(handler);
    }

    pub async fn run(&mut self) {
        while let Some(response) = self.rx.recv().await {
            let proto_id = response.header.proto_id;

            match proto_id {
                UPDATE_BASIC_QOT_PROTO_ID => {
                    // TODO: Parse actual quote data
                    let data = StockQuote {
                        code: String::new(),
                        last_done: 0.0,
                        prev_close_price: 0.0,
                        open_price: 0.0,
                        high_price: 0.0,
                        low_price: 0.0,
                        volume: 0,
                        turnover: 0.0,
                        change_val: 0.0,
                        change_rate: 0.0,
                        amplitude: 0.0,
                        pe_ratio: 0.0,
                        yield_rate: 0.0,
                    };

                    for handler in &self.handlers {
                        handler.on_quote(data.clone());
                    }
                }
                UPDATE_KL_PROTO_ID => {
                    let data = KlineBar {
                        code: String::new(),
                        time_key: String::new(),
                        open: 0.0,
                        close: 0.0,
                        high: 0.0,
                        low: 0.0,
                        volume: 0,
                        turnover: 0.0,
                        change_rate: 0.0,
                    };

                    for handler in &self.handlers {
                        handler.on_kline(data.clone());
                    }
                }
                UPDATE_ORDER_BOOK_PROTO_ID => {
                    let data = OrderBook {
                        code: String::new(),
                        bid: vec![],
                        ask: vec![],
                    };

                    for handler in &self.handlers {
                        handler.on_order_book(data.clone());
                    }
                }
                UPDATE_TICKER_PROTO_ID => {
                    let data = Ticker {
                        code: String::new(),
                        time: String::new(),
                        price: 0.0,
                        volume: 0,
                        turnover: 0.0,
                        ticker_direction: 0,
                        sequence: 0,
                    };

                    for handler in &self.handlers {
                        handler.on_ticker(data.clone());
                    }
                }
                _ => {
                    // Unknown push type
                }
            }
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_data_handler_creation() {
        let (_tx, rx) = mpsc::unbounded_channel();
        let _handler = PushDataHandler::new(rx);
    }
}
