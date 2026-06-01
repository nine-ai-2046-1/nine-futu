use crate::error::FutuError;
use super::MarketHandler;

pub struct HkMarket;

impl MarketHandler for HkMarket {
    fn prefix(&self) -> &str {
        "HK"
    }

    fn parse_code(&self, input: &str) -> Result<String, FutuError> {
        // Remove any existing prefix if accidentally passed
        let code = if let Some((_, c)) = input.split_once('.') {
            c
        } else {
            input
        };

        // Check if numeric
        if !code.chars().all(|c| c.is_ascii_digit()) {
            return Err(FutuError::ParamErr(format!("HK stock code must be numeric: {}", code)));
        }

        // Pad to 5 digits
        let padded = format!("{:0>5}", code);
        Ok(padded)
    }

    fn market_id(&self) -> i32 {
        1 // HK market ID in Futu
    }

    fn name(&self) -> &str {
        "Hong Kong"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hk_code_parsing() {
        let hk = HkMarket;
        assert_eq!(hk.parse_code("700").unwrap(), "00700");
        assert_eq!(hk.parse_code("00700").unwrap(), "00700");
        assert_eq!(hk.parse_code("0700").unwrap(), "00700");
        assert_eq!(hk.parse_code("1234").unwrap(), "01234");
        assert_eq!(hk.parse_code("9988").unwrap(), "09988");
    }
}
