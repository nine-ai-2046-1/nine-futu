use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::error::FutuError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub account: AccountConfig,
    pub connection: ConnectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub account_id: String,
    pub password: String,
    pub real_trade_enabled: bool,
    pub default_trade_env: String,
    pub default_account_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            account: AccountConfig {
                account_id: String::new(),
                password: String::new(),
                real_trade_enabled: false,
                default_trade_env: "SIMULATE".to_string(),
                default_account_type: "CASH".to_string(),
            },
            connection: ConnectionConfig {
                host: "127.0.0.1".to_string(),
                port: 11111,
            },
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".opens")
            .join("nine-futu")
            .join("config.toml")
    }

    pub fn load() -> Result<Self, FutuError> {
        let path = Self::config_path();
        
        if !path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| FutuError::IoError(format!("Failed to read config: {}", e)))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| FutuError::IoError(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<(), FutuError> {
        let path = Self::config_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| FutuError::IoError(format!("Failed to create config dir: {}", e)))?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| FutuError::IoError(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&path, content)
            .map_err(|e| FutuError::IoError(format!("Failed to write config: {}", e)))?;
        
        Ok(())
    }

    pub fn is_real_trade_enabled(&self) -> bool {
        self.account.real_trade_enabled
    }

    pub fn get_trade_env(&self, flag: Option<&str>) -> String {
        match flag {
            Some("real") => "REAL".to_string(),
            Some("sim") => "SIMULATE".to_string(),
            Some(_) => self.account.default_trade_env.clone(),
            None => self.account.default_trade_env.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.is_real_trade_enabled());
        assert_eq!(config.get_trade_env(None), "SIMULATE");
        assert_eq!(config.get_trade_env(Some("sim")), "SIMULATE");
        assert_eq!(config.get_trade_env(Some("real")), "REAL");
    }
}
