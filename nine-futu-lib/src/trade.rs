use bytes::Bytes;
use prost::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::client::FutuClient;
use crate::config::Config;
use crate::error::FutuError;
use crate::types::*;

pub struct TradeClient {
    client: FutuClient,
    config: Config,
    trade_env: String,
    account_type: String,
    acc_id: u64,
}

impl TradeClient {
    pub async fn new(host: &str, port: u16, config: Config) -> Result<Self, FutuError> {
        let mut client = FutuClient::connect(host, port, false).await?;
        
        client.init_connect().await?;
        
        let trade_env = config.get_trade_env(None);
        let account_type = config.account.default_account_type.clone();
        
        Ok(Self {
            client,
            config,
            trade_env,
            account_type,
            acc_id: 0,
        })
    }

    pub fn set_trade_env(&mut self, env: &str) {
        self.trade_env = Self::parse_trade_env(env);
    }

    pub fn get_trade_env(&self) -> &str {
        &self.trade_env
    }

    pub fn is_real_trade(&self) -> bool {
        self.trade_env == "REAL"
    }

    pub fn set_account_type(&mut self, acc_type: &str) {
        self.account_type = Self::parse_account_type(acc_type);
    }

    pub fn get_account_type(&self) -> &str {
        &self.account_type
    }

    pub fn set_acc_id(&mut self, acc_id: u64) {
        self.acc_id = acc_id;
    }

    pub fn get_acc_id(&self) -> u64 {
        self.acc_id
    }

    /// Parse trade environment from string
    pub fn parse_trade_env(env: &str) -> String {
        match env.to_lowercase().as_str() {
            "sim" | "simulate" | "simulation" => "SIMULATE".to_string(),
            "real" | "live" => "REAL".to_string(),
            _ => env.to_uppercase(),
        }
    }

    /// Parse account type from string
    pub fn parse_account_type(acc_type: &str) -> String {
        match acc_type.to_lowercase().as_str() {
            "cash" | "c" => "CASH".to_string(),
            "margin" | "m" => "MARGIN".to_string(),
            _ => acc_type.to_uppercase(),
        }
    }

    /// Get TrdEnv numeric value
    pub fn get_trd_env_value(&self) -> i32 {
        match self.trade_env.as_str() {
            "SIMULATE" => 0,
            "REAL" => 1,
            _ => 0,
        }
    }

    /// Get TrdAccType numeric value
    pub fn get_trd_acc_type_value(&self) -> i32 {
        match self.account_type.as_str() {
            "CASH" => 1,
            "MARGIN" => 2,
            _ => 1,
        }
    }

    /// Auto-select account ID based on environment and account type
    pub async fn auto_select_account(&mut self) -> Result<(), FutuError> {
        let accounts = self.get_acc_list().await?;
        
        let trd_env = self.get_trd_env_value();
        let acc_type = self.get_trd_acc_type_value();
        
        // Find matching account
        for acc in &accounts {
            if acc.trd_env == trd_env && acc.acc_type == Some(acc_type) {
                self.acc_id = acc.acc_id;
                return Ok(());
            }
        }
        
        // Fallback: try any account with matching environment
        for acc in &accounts {
            if acc.trd_env == trd_env {
                self.acc_id = acc.acc_id;
                return Ok(());
            }
        }
        
        Err(FutuError::ParamErr(format!(
            "No account found for environment {} and type {}",
            self.trade_env, self.account_type
        )))
    }

    pub fn check_real_trade_enabled(&self) -> Result<(), FutuError> {
        if self.is_real_trade() && !self.config.is_real_trade_enabled() {
            return Err(FutuError::ParamErr(
                "Real trading is disabled in config. Set real_trade_enabled = true in config.toml".to_string()
            ));
        }
        Ok(())
    }

    // Account APIs
    pub async fn get_acc_list(&mut self) -> Result<Vec<AccountInfo>, FutuError> {
        use crate::proto::trd_get_acc_list::{Request, C2s};
        
        let c2s = C2s {
            user_id: 0, // Deprecated, fill with 0
            trd_category: None,
            need_general_sec_account: Some(true),
        };
        
        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;
        
        let rx = self.client.send_request(2001, body).await?;
        let response = self.client.wait_response(rx).await?;
        
        use crate::proto::trd_get_acc_list::Response;
        let rsp = Response::decode(response.body)?;
        
        if rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: rsp.ret_type,
                msg: rsp.ret_msg.unwrap_or_default(),
            });
        }
        
        let s2c = rsp.s2c.ok_or(FutuError::PacketDataErr)?;
        
        let accounts = s2c.acc_list.iter().map(|acc| AccountInfo {
            acc_id: acc.acc_id,
            trd_env: acc.trd_env,
            acc_type: acc.acc_type,
            card_num: acc.card_num.clone().unwrap_or_default(),
            security_firm: format!("{:?}", acc.security_firm()),
        }).collect();
        
        Ok(accounts)
    }

    pub async fn get_funds(&mut self) -> Result<AccountFunds, FutuError> {
        // Auto-select account if not set
        if self.acc_id == 0 {
            self.auto_select_account().await?;
        }
        
        use crate::proto::trd_get_funds::{Request, C2s};
        use crate::proto::trd_common::TrdHeader;
        
        let c2s = C2s {
            header: TrdHeader {
                trd_env: self.get_trd_env_value(),
                acc_id: self.acc_id,
                trd_market: 1, // HK
                jp_acc_type: None,
            },
            ..Default::default()
        };
        
        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;
        
        let rx = self.client.send_request(2101, body).await?;
        let response = self.client.wait_response(rx).await?;
        
        use crate::proto::trd_get_funds::Response;
        let rsp = Response::decode(response.body)?;
        
        if rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: rsp.ret_type,
                msg: rsp.ret_msg.unwrap_or_default(),
            });
        }
        
        let s2c = rsp.s2c.ok_or(FutuError::PacketDataErr)?;
        let funds = s2c.funds.ok_or(FutuError::PacketDataErr)?;
        
        Ok(AccountFunds {
            total_assets: funds.total_assets,
            cash: funds.cash,
            market_val: funds.market_val,
            frozen_cash: funds.frozen_cash,
            power: funds.power,
        })
    }

    // Order APIs
    pub async fn place_order(
        &mut self,
        code: &str,
        side: &str,
        qty: i32,
        price: f64,
        order_type: &str,
    ) -> Result<OrderResult, FutuError> {
        self.check_real_trade_enabled()?;
        
        use crate::proto::trd_place_order::{Request, C2s};
        use crate::proto::trd_common::TrdHeader;
        use crate::proto::common::PacketId;
        
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
        
        let trd_side = match side {
            "BUY" => 1,
            "SELL" => 2,
            _ => return Err(FutuError::ParamErr(format!("Invalid side: {}", side))),
        };
        
        let order_type_val = match order_type {
            "NORMAL" => 1,
            "MARKET" => 2,
            _ => return Err(FutuError::ParamErr(format!("Invalid order type: {}", order_type))),
        };
        
        let c2s = C2s {
            packet_id: PacketId {
                conn_id: self.client.conn_id(),
                serial_no: self.client.serial_mgr.next(),
            },
            header: TrdHeader {
                trd_env: 1, // SIMULATE
                acc_id: 0,
                trd_market: market,
                jp_acc_type: None,
            },
            trd_side,
            order_type: order_type_val,
            code: parts[1].to_string(),
            qty: qty as f64,
            price: Some(price),
            ..Default::default()
        };
        
        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;
        
        let rx = self.client.send_request(2202, body).await?;
        let response = self.client.wait_response(rx).await?;
        
        use crate::proto::trd_place_order::Response;
        let rsp = Response::decode(response.body)?;
        
        if rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: rsp.ret_type,
                msg: rsp.ret_msg.unwrap_or_default(),
            });
        }
        
        let s2c = rsp.s2c.ok_or(FutuError::PacketDataErr)?;
        
        Ok(OrderResult {
            order_id: s2c.order_id.unwrap_or(0),
            status: "SUBMITTED".to_string(),
        })
    }

    pub async fn modify_order(
        &mut self,
        order_id: u64,
        price: Option<f64>,
        qty: Option<i32>,
    ) -> Result<OrderResult, FutuError> {
        self.check_real_trade_enabled()?;
        
        // TODO: Implement actual proto encoding
        Ok(OrderResult {
            order_id,
            status: "MODIFIED".to_string(),
        })
    }

    pub async fn cancel_order(&mut self, order_id: u64) -> Result<OrderResult, FutuError> {
        self.check_real_trade_enabled()?;
        
        // TODO: Implement actual proto encoding
        Ok(OrderResult {
            order_id,
            status: "CANCELLED".to_string(),
        })
    }

    pub async fn get_orders(&mut self, history: bool) -> Result<Vec<OrderInfo>, FutuError> {
        // TODO: Implement actual proto encoding
        Ok(vec![])
    }

    // Position APIs
    pub async fn get_positions(&mut self) -> Result<Vec<PositionInfo>, FutuError> {
        // Auto-select account if not set
        if self.acc_id == 0 {
            self.auto_select_account().await?;
        }
        
        use crate::proto::trd_get_position_list::{Request, C2s};
        use crate::proto::trd_common::TrdHeader;
        
        let c2s = C2s {
            header: TrdHeader {
                trd_env: self.get_trd_env_value(),
                acc_id: self.acc_id,
                trd_market: 1, // HK
                jp_acc_type: None,
            },
            ..Default::default()
        };
        
        let request = Request { c2s };
        let mut body = Vec::new();
        request.encode(&mut body)?;
        
        let rx = self.client.send_request(2102, body).await?;
        let response = self.client.wait_response(rx).await?;
        
        use crate::proto::trd_get_position_list::Response;
        let rsp = Response::decode(response.body)?;
        
        if rsp.ret_type != 0 {
            return Err(FutuError::ProtoError {
                ret_type: rsp.ret_type,
                msg: rsp.ret_msg.unwrap_or_default(),
            });
        }
        
        let s2c = rsp.s2c.ok_or(FutuError::PacketDataErr)?;
        
        let positions = s2c.position_list.iter().map(|pos| {
            PositionInfo {
                code: pos.code.clone(),
                qty: pos.qty as i32,
                cost_price: pos.cost_price.unwrap_or(0.0),
                market_val: pos.val,
                pl_val: pos.pl_val,
                pl_ratio: pos.pl_ratio.unwrap_or(0.0),
            }
        }).collect();
        
        Ok(positions)
    }

    // Trade History APIs
    pub async fn get_trades(&mut self, history: bool) -> Result<Vec<TradeInfo>, FutuError> {
        // TODO: Implement actual proto encoding
        Ok(vec![])
    }

    pub async fn get_cashflow(&mut self) -> Result<Vec<CashflowInfo>, FutuError> {
        // TODO: Implement actual proto encoding
        Ok(vec![])
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountInfo {
    pub acc_id: u64,
    pub trd_env: i32,
    pub acc_type: Option<i32>,
    pub card_num: String,
    pub security_firm: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountFunds {
    pub total_assets: f64,
    pub cash: f64,
    pub market_val: f64,
    pub frozen_cash: f64,
    pub power: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderResult {
    pub order_id: u64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderInfo {
    pub order_id: u64,
    pub code: String,
    pub side: String,
    pub qty: i32,
    pub price: f64,
    pub status: String,
    pub create_time: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PositionInfo {
    pub code: String,
    pub qty: i32,
    pub cost_price: f64,
    pub market_val: f64,
    pub pl_val: f64,
    pub pl_ratio: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeInfo {
    pub order_id: u64,
    pub code: String,
    pub side: String,
    pub qty: i32,
    pub price: f64,
    pub deal_time: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CashflowInfo {
    pub code: String,
    pub cash_flow: f64,
    pub balance: f64,
    pub create_time: String,
}

use serde::Serialize;

/// Convert order status code to human-readable description
pub fn order_status_desc(status: i32) -> &'static str {
    match status {
        -1 => "Unknown",
        1 => "Waiting Submit",
        2 => "Submitting",
        5 => "Submitted",
        10 => "Partially Filled",
        11 => "Filled",
        14 => "Partially Cancelled",
        15 => "Cancelled",
        21 => "Failed",
        22 => "Disabled",
        23 => "Deleted",
        24 => "Fill Cancelled",
        _ => "Unknown",
    }
}

/// Convert trade environment string to numeric value
pub fn trade_env_to_value(env: &str) -> i32 {
    match env.to_uppercase().as_str() {
        "SIMULATE" => 0,
        "REAL" => 1,
        _ => 0,
    }
}

/// Convert account type string to numeric value
pub fn account_type_to_value(acc_type: &str) -> i32 {
    match acc_type.to_uppercase().as_str() {
        "CASH" => 1,
        "MARGIN" => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_env() {
        assert_eq!(TradeClient::parse_trade_env("sim"), "SIMULATE");
        assert_eq!(TradeClient::parse_trade_env("real"), "REAL");
        assert_eq!(TradeClient::parse_trade_env("SIMULATE"), "SIMULATE");
        assert_eq!(TradeClient::parse_trade_env("REAL"), "REAL");
    }

    #[test]
    fn test_account_type() {
        assert_eq!(TradeClient::parse_account_type("cash"), "CASH");
        assert_eq!(TradeClient::parse_account_type("margin"), "MARGIN");
        assert_eq!(TradeClient::parse_account_type("CASH"), "CASH");
        assert_eq!(TradeClient::parse_account_type("MARGIN"), "MARGIN");
    }

    #[test]
    fn test_order_status_desc() {
        assert_eq!(order_status_desc(5), "Submitted");
        assert_eq!(order_status_desc(11), "Filled");
        assert_eq!(order_status_desc(15), "Cancelled");
        assert_eq!(order_status_desc(21), "Failed");
    }

    #[test]
    fn test_trade_env_to_value() {
        assert_eq!(trade_env_to_value("SIMULATE"), 0);
        assert_eq!(trade_env_to_value("REAL"), 1);
    }

    #[test]
    fn test_account_type_to_value() {
        assert_eq!(account_type_to_value("CASH"), 1);
        assert_eq!(account_type_to_value("MARGIN"), 2);
    }
}
