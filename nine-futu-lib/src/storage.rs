use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::FutuError;

pub struct LiveStorage {
    base_dir: PathBuf,
}

impl LiveStorage {
    pub fn new() -> Self {
        let base_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".opens")
            .join("nine-futu")
            .join("data")
            .join("live");

        Self { base_dir }
    }

    pub fn with_base_dir(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Get the path for a specific data type
    ///
    /// # Arguments
    /// * `code` - Stock code (e.g., "HK.00700")
    /// * `date` - Date string (e.g., "2026-06-01")
    /// * `data_type` - Data type (e.g., "quote", "orderbook", "ticker", "broker", "rt_data")
    pub fn get_data_path(&self, code: &str, date: &str, data_type: &str) -> PathBuf {
        self.base_dir
            .join(code)
            .join(date)
            .join(format!("{}.txt", data_type))
    }

    /// Get the path for K-line data with timeframe
    ///
    /// # Arguments
    /// * `code` - Stock code (e.g., "HK.00700")
    /// * `date` - Date string (e.g., "2026-06-01")
    /// * `timeframe` - Timeframe (e.g., "5m", "15m", "1d")
    pub fn get_kline_path(&self, code: &str, date: &str, timeframe: &str) -> PathBuf {
        self.base_dir
            .join(code)
            .join(date)
            .join(timeframe)
            .join("kline.txt")
    }

    /// Create directory structure if it doesn't exist
    pub fn ensure_dir(&self, path: &Path) -> Result<(), FutuError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    /// Append a line to a file (create if not exists)
    pub fn append_line(&self, path: &Path, line: &str) -> Result<(), FutuError> {
        self.ensure_dir(path)?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        writeln!(file, "{}", line)?;

        Ok(())
    }

    /// Get the base directory
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    /// List all code directories
    pub fn list_codes(&self) -> Result<Vec<String>, FutuError> {
        let mut codes = Vec::new();

        if self.base_dir.exists() {
            for entry in fs::read_dir(&self.base_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        codes.push(name.to_string());
                    }
                }
            }
        }

        Ok(codes)
    }

    /// List all date directories for a code
    pub fn list_dates(&self, code: &str) -> Result<Vec<String>, FutuError> {
        let mut dates = Vec::new();
        let code_dir = self.base_dir.join(code);

        if code_dir.exists() {
            for entry in fs::read_dir(&code_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        dates.push(name.to_string());
                    }
                }
            }
        }

        Ok(dates)
    }

    /// Check if a date folder is older than N days
    pub fn isOLDER(&self, date_str: &str, days: u32) -> Result<bool, FutuError> {
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|e| FutuError::ParamErr(format!("Invalid date: {}", e)))?;

        let today = chrono::Local::now().date_naive();
        let cutoff = today - chrono::Duration::days(days as i64);

        Ok(date < cutoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_paths() {
        let storage = LiveStorage::new();

        let quote_path = storage.get_data_path("HK.00700", "2026-06-01", "quote");
        assert!(quote_path.to_string_lossy().contains("HK.00700"));
        assert!(quote_path.to_string_lossy().contains("2026-06-01"));
        assert!(quote_path.to_string_lossy().contains("quote.txt"));

        let kline_path = storage.get_kline_path("HK.00700", "2026-06-01", "5m");
        assert!(kline_path.to_string_lossy().contains("5m"));
        assert!(kline_path.to_string_lossy().contains("kline.txt"));
    }
}
