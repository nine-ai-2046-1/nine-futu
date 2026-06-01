pub mod hk;
pub mod us;

use crate::error::FutuError;

pub trait MarketHandler: Send + Sync {
    fn prefix(&self) -> &str;
    fn parse_code(&self, input: &str) -> Result<String, FutuError>;
    fn market_id(&self) -> i32;
    fn name(&self) -> &str;
}

pub struct MarketRegistry {
    handlers: Vec<Box<dyn MarketHandler>>,
}

impl MarketRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: Vec::new(),
        };
        registry.register(Box::new(hk::HkMarket));
        registry.register(Box::new(us::UsMarket));
        registry
    }

    pub fn register(&mut self, handler: Box<dyn MarketHandler>) {
        self.handlers.push(handler);
    }

    pub fn parse_code(&self, input: &str) -> Result<(String, i32), FutuError> {
        // Check if already has prefix
        if let Some((prefix, code)) = input.split_once('.') {
            let upper_prefix = prefix.to_uppercase();
            for handler in &self.handlers {
                if handler.prefix() == upper_prefix {
                    let formatted = handler.parse_code(code)?;
                    return Ok((format!("{}.{}", handler.prefix(), formatted), handler.market_id()));
                }
            }
            return Err(FutuError::ParamErr(format!("Unknown market prefix: {}", prefix)));
        }

        // Check if numeric only
        let is_numeric = input.chars().all(|c| c.is_ascii_digit());
        if is_numeric {
            // HK market
            let hk = &self.handlers[0]; // HK is first
            let formatted = hk.parse_code(input)?;
            return Ok((format!("{}.{}", hk.prefix(), formatted), hk.market_id()));
        }

        // Non-numeric → US market
        let us = &self.handlers[1]; // US is second
        let formatted = us.parse_code(input)?;
        Ok((format!("{}.{}", us.prefix(), formatted), us.market_id()))
    }

    pub fn get_handler_by_prefix(&self, prefix: &str) -> Option<&dyn MarketHandler> {
        self.handlers.iter().find(|h| h.prefix() == prefix).map(|h| h.as_ref())
    }
}

impl Default for MarketRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric_code() {
        let registry = MarketRegistry::new();
        assert_eq!(registry.parse_code("700").unwrap().0, "HK.00700");
        assert_eq!(registry.parse_code("00700").unwrap().0, "HK.00700");
        assert_eq!(registry.parse_code("0700").unwrap().0, "HK.00700");
        assert_eq!(registry.parse_code("1234").unwrap().0, "HK.01234");
    }

    #[test]
    fn test_parse_alpha_code() {
        let registry = MarketRegistry::new();
        assert_eq!(registry.parse_code("AAPL").unwrap().0, "US.AAPL");
        assert_eq!(registry.parse_code("FUTU").unwrap().0, "US.FUTU");
        assert_eq!(registry.parse_code("aapl").unwrap().0, "US.AAPL");
    }

    #[test]
    fn test_parse_full_code() {
        let registry = MarketRegistry::new();
        assert_eq!(registry.parse_code("HK.00700").unwrap().0, "HK.00700");
        assert_eq!(registry.parse_code("US.AAPL").unwrap().0, "US.AAPL");
        assert_eq!(registry.parse_code("hk.00700").unwrap().0, "HK.00700");
        assert_eq!(registry.parse_code("us.aapl").unwrap().0, "US.AAPL");
    }

    #[test]
    fn test_market_id() {
        let registry = MarketRegistry::new();
        assert_eq!(registry.parse_code("700").unwrap().1, 1); // HK
        assert_eq!(registry.parse_code("AAPL").unwrap().1, 2); // US
    }

    #[test]
    fn test_unknown_market_prefix() {
        let registry = MarketRegistry::new();
        let result = registry.parse_code("XX.1234");
        assert!(result.is_err());
    }

    #[test]
    fn test_hk_market_padding() {
        let hk = hk::HkMarket;
        assert_eq!(hk.parse_code("1").unwrap(), "00001");
        assert_eq!(hk.parse_code("12").unwrap(), "00012");
        assert_eq!(hk.parse_code("123").unwrap(), "00123");
        assert_eq!(hk.parse_code("1234").unwrap(), "01234");
        assert_eq!(hk.parse_code("12345").unwrap(), "12345");
    }

    #[test]
    fn test_us_market_uppercase() {
        let us = us::UsMarket;
        assert_eq!(us.parse_code("aapl").unwrap(), "AAPL");
        assert_eq!(us.parse_code("Tsla").unwrap(), "TSLA");
        assert_eq!(us.parse_code("MSFT").unwrap(), "MSFT");
    }

    #[test]
    fn test_market_registry_handler_lookup() {
        let registry = MarketRegistry::new();
        assert!(registry.get_handler_by_prefix("HK").is_some());
        assert!(registry.get_handler_by_prefix("US").is_some());
        assert!(registry.get_handler_by_prefix("XX").is_none());
    }
}
