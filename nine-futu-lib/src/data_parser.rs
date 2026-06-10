use crate::types::*;
use crate::push_handler::PushData;

pub struct DataParser;

impl DataParser {
    /// Convert PushData to JSON string
    pub fn to_json(data: &PushData) -> Result<String, serde_json::Error> {
        match data {
            PushData::Quote(d) => serde_json::to_string(d),
            PushData::Kline(d) => serde_json::to_string(d),
            PushData::OrderBook(d) => serde_json::to_string(d),
            PushData::Ticker(d) => serde_json::to_string(d),
            PushData::RtData(d) => serde_json::to_string(d),
            PushData::Broker(d) => serde_json::to_string(d),
        }
    }

    /// Get the file extension for a push data type
    pub fn file_extension(data_type: &str) -> &str {
        match data_type {
            "quote" => "txt",
            "orderbook" => "txt",
            "ticker" => "txt",
            "broker" => "txt",
            "rt_data" => "txt",
            "kline" => "txt",
            _ => "txt",
        }
    }

    /// Get the data type name from PushData
    pub fn data_type_name(data: &PushData) -> &str {
        match data {
            PushData::Quote(_) => "quote",
            PushData::Kline(_) => "kline",
            PushData::OrderBook(_) => "orderbook",
            PushData::Ticker(_) => "ticker",
            PushData::RtData(_) => "rt_data",
            PushData::Broker(_) => "broker",
        }
    }

    /// Extract code from PushData
    pub fn extract_code(data: &PushData) -> &str {
        match data {
            PushData::Quote(d) => &d.code,
            PushData::Kline(d) => &d.code,
            PushData::OrderBook(d) => &d.code,
            PushData::Ticker(d) => &d.code,
            PushData::RtData(d) => &d.code,
            PushData::Broker(d) => &d.code,
        }
    }

    /// Extract timeframe from Kline data (if applicable)
    pub fn extract_timeframe(data: &PushData) -> Option<&str> {
        // Timeframe is determined by subscription type, not the data itself
        // This will be passed separately when storing
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_names() {
        let quote = PushData::Quote(StockQuote {
            code: "HK.00700".to_string(),
            last_done: 100.0,
            prev_close_price: 99.0,
            open_price: 99.5,
            high_price: 101.0,
            low_price: 98.5,
            volume: 1000000,
            turnover: 100000000.0,
            change_val: 1.0,
            change_rate: 1.0,
            amplitude: 2.5,
            pe_ratio: 20.0,
            yield_rate: 2.0,
        });

        assert_eq!(DataParser::data_type_name(&quote), "quote");
    }

    #[test]
    fn test_to_json() {
        let quote = PushData::Quote(StockQuote {
            code: "HK.00700".to_string(),
            last_done: 100.0,
            prev_close_price: 99.0,
            open_price: 99.5,
            high_price: 101.0,
            low_price: 98.5,
            volume: 1000000,
            turnover: 100000000.0,
            change_val: 1.0,
            change_rate: 1.0,
            amplitude: 2.5,
            pe_ratio: 20.0,
            yield_rate: 2.0,
        });

        let json = DataParser::to_json(&quote).unwrap();
        assert!(json.contains("HK.00700"));
        assert!(json.contains("100.0"));
    }
}
