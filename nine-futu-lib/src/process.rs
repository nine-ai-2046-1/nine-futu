use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process;

use crate::error::FutuError;

pub struct ProcessManager {
    pid_dir: PathBuf,
}

impl ProcessManager {
    pub fn new() -> Self {
        let pid_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".opens")
            .join("nine-futu")
            .join("pid");

        Self { pid_dir }
    }

    /// Get the PID file path for a code
    pub fn pid_file_path(&self, code: &str) -> PathBuf {
        self.pid_dir.join(format!("{}.pid", code))
    }

    /// Create PID file with format: {pid}\n{timeframe}\n{start_time}
    pub fn create_pid_file(&self, code: &str, timeframe: &str) -> Result<(), FutuError> {
        fs::create_dir_all(&self.pid_dir)?;

        let pid = process::id();
        let start_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let content = format!("{}\n{}\n{}", pid, timeframe, start_time);

        let mut file = File::create(self.pid_file_path(code))?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Remove PID file
    pub fn remove_pid_file(&self, code: &str) -> Result<(), FutuError> {
        let path = self.pid_file_path(code);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Check if a process is running for a code
    /// Returns PID if running, -1 if not
    pub fn check_process(&self, code: &str) -> i32 {
        let path = self.pid_file_path(code);

        if !path.exists() {
            return -1;
        }

        if let Ok(content) = fs::read_to_string(&path) {
            if let Some(pid_str) = content.lines().next() {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    // Check if process is still running
                    if self.is_process_running(pid) {
                        return pid as i32;
                    } else {
                        // Stale PID file, clean it up
                        let _ = fs::remove_file(&path);
                    }
                }
            }
        }

        -1
    }

    /// Check if a process is running
    fn is_process_running(&self, pid: u32) -> bool {
        // On Unix, we can check if a process exists by sending signal 0
        #[cfg(unix)]
        {
            unsafe {
                libc::kill(pid as i32, 0) == 0
            }
        }
        #[cfg(not(unix))]
        {
            // On non-Unix, just check if PID file exists
            true
        }
    }

    /// List all running daemons
    pub fn list_daemons(&self) -> Result<Vec<ProcessInfo>, FutuError> {
        let mut daemons = Vec::new();

        if !self.pid_dir.exists() {
            return Ok(daemons);
        }

        for entry in fs::read_dir(&self.pid_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "pid") {
                if let Some(code) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let lines: Vec<&str> = content.lines().collect();
                        if lines.len() >= 3 {
                            if let Ok(pid) = lines[0].parse::<u32>() {
                                let timeframe = lines[1].to_string();
                                let start_time = lines[2].to_string();

                                if self.is_process_running(pid) {
                                    daemons.push(ProcessInfo {
                                        pid,
                                        code: code.to_string(),
                                        timeframe,
                                        start_time,
                                    });
                                } else {
                                    // Stale PID file
                                    let _ = fs::remove_file(&path);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(daemons)
    }

    /// Stop a daemon by PID
    pub fn stop_daemon(&self, pid: u32) -> Result<(), FutuError> {
        #[cfg(unix)]
        {
            unsafe {
                if libc::kill(pid as i32, libc::SIGTERM) == 0 {
                    // Wait a bit for process to exit
                    std::thread::sleep(std::time::Duration::from_millis(100));

                    // If still running, force kill
                    if self.is_process_running(pid) {
                        let _ = libc::kill(pid as i32, libc::SIGKILL);
                    }
                }
            }
        }

        // Clean up any PID file with this PID
        for entry in fs::read_dir(&self.pid_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "pid") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(pid_str) = content.lines().next() {
                        if let Ok(file_pid) = pid_str.parse::<u32>() {
                            if file_pid == pid {
                                let _ = fs::remove_file(&path);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub code: String,
    pub timeframe: String,
    pub start_time: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_file_path() {
        let mgr = ProcessManager::new();
        let path = mgr.pid_file_path("HK.00700");
        assert!(path.to_string_lossy().contains("HK.00700.pid"));
    }
}
