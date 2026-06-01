use crate::error::FutuError;
use super::MarketHandler;

pub struct UsMarket;

impl MarketHandler for UsMarket {
    fn prefix(&self) -> &str {
        "US"
    }

    fn parse_code(&self, input: &str) -> Result<String, FutuError> {
        // Remove any existing prefix if accidentally passed
        let code = if let Some((_, c)) = input.split_once('.') {
            c
        } else {
            input
        };

        // US codes are alphabetic, just uppercase
        Ok(code.to_uppercase())
    }

    fn market_id(&self) -> i32 {
        2 // US market ID in Futu
    }

    fn name(&self) -> &str {
        "United States"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_code_parsing() {
        let us = UsMarket;
        assert_eq!(us.parse_code("AAPL").unwrap(), "AAPL");
        assert_eq!(us.parse_code("aapl").unwrap(), "AAPL");
        assert_eq!(us.parse_code("FUTU").unwrap(), "FUTU");
    }
}
