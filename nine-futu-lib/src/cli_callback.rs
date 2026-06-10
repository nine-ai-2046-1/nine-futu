use std::process::Command;

/// Call external CLI tool for each K-line bar
///
/// # Arguments
/// * `session_id` - Session ID for the external tool
/// * `code` - Stock code (e.g., "HK.00700")
/// * `ktype` - K-line type (e.g., "5m")
/// * `data_json` - JSON string of K-line data
pub fn call_cli(session_id: &str, code: &str, ktype: &str, data_json: &str) -> Result<(), String> {
    let output = Command::new("nine-stock")
        .arg("--session").arg(session_id)
        .arg("--code").arg(code)
        .arg("--ktype").arg(ktype)
        .arg("--data").arg(data_json)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("[nine-stock error] {}", stderr);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("[nine-stock error] Failed to execute: {}", e);
            Ok(()) // Don't fail the main process
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_callback_exists() {
        // Test that the module compiles
    }
}
