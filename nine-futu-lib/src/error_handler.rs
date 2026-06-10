use std::process::Command;

use crate::error::FutuError;
use crate::process::ProcessManager;

pub struct ErrorHandler {
    process_mgr: ProcessManager,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            process_mgr: ProcessManager::new(),
        }
    }

    /// Send notification via opencb
    pub fn notify(&self, message: &str) {
        let _ = Command::new("opencb")
            .arg("send")
            .arg(message)
            .output();
    }

    /// Handle connection error
    pub fn handle_connection_error(&self, code: &str) {
        self.notify("WARNING-NINE_FUT OpenD-Connection-Error");
        let _ = self.process_mgr.remove_pid_file(code);
    }

    /// Handle runtime error
    pub fn handle_runtime_error(&self, code: &str) {
        self.notify("WARNING-NINE_FUT Sub-Daemon-Error");
        let _ = self.process_mgr.remove_pid_file(code);
    }

    /// Try to connect with retries
    pub async fn connect_with_retries<F, Fut>(
        &self,
        code: &str,
        max_retries: u32,
        connect_fn: F,
    ) -> Result<(), FutuError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<(), FutuError>>,
    {
        for attempt in 1..=max_retries {
            match connect_fn().await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    eprintln!("Connection attempt {} failed: {}", attempt, e);
                    if attempt < max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        }

        self.handle_connection_error(code);
        Err(FutuError::ConnectionLost)
    }

    /// Run a closure with error handling
    pub fn with_error_handling<F, T>(&self, code: &str, f: F) -> Result<T, FutuError>
    where
        F: FnOnce() -> Result<T, FutuError>,
    {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
            Ok(result) => result,
            Err(_) => {
                self.handle_runtime_error(code);
                Err(FutuError::ConnectionLost)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_handler_creation() {
        let _handler = ErrorHandler::new();
    }
}
