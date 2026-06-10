use crate::types::*;
use crate::push_handler::PushHandler;

pub struct CliCallbackHandler {
    session_id: String,
    code: String,
    ktype: String,
}

impl CliCallbackHandler {
    pub fn new(session_id: &str, code: &str, ktype: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            code: code.to_string(),
            ktype: ktype.to_string(),
        }
    }
}

impl PushHandler for CliCallbackHandler {
    fn on_quote(&self, _data: StockQuote) {
        // Not used for CLI callback
    }

    fn on_kline(&self, data: KlineBar) {
        let json = match serde_json::to_string(&data) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("[CLI callback error] Failed to serialize kline: {}", e);
                return;
            }
        };

        if let Err(e) = crate::call_cli(&self.session_id, &self.code, &self.ktype, &json) {
            eprintln!("[CLI callback error] {}", e);
        }
    }

    fn on_order_book(&self, _data: OrderBook) {
        // Not used for CLI callback
    }

    fn on_ticker(&self, _data: Ticker) {
        // Not used for CLI callback
    }

    fn on_rt_data(&self, _data: RtData) {
        // Not used for CLI callback
    }

    fn on_broker(&self, _data: BrokerQueue) {
        // Not used for CLI callback
    }
}
