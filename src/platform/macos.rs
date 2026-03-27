//! macOS-specific platform implementation

use std::process::Command as ProcessCommand;

#[derive(Clone)]
pub struct PlatformImpl;

impl PlatformImpl {
    pub fn new() -> Self {
        Self
    }

    /// Execute ping command on macOS
    pub fn execute_ping(&self, target: &str, count: usize) -> std::io::Result<std::process::Output> {
        ProcessCommand::new("ping")
            .arg("-c")
            .arg(count.to_string())
            .arg(target)
            .output()
    }

    /// Parse ping output to extract packet loss percentage
    pub fn parse_ping_output(&self, stdout: &[u8]) -> f64 {
        let stdout = String::from_utf8_lossy(&stdout);
        
        // macOS format: "6 packets transmitted, 5 received, 16.6667% packet loss, time 5007ms"
        regex::Regex::new(r"(\d+\.?\d*)\s*%\s*packet\s+loss")
            .ok()
            .and_then(|re| re.captures(&stdout))
            .and_then(|caps| caps.get(1))
            .and_then(|match_| match_.as_str().parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// Get default gateway using route command
    pub fn get_default_gateway(&self) -> Result<String, String> {
        let output = ProcessCommand::new("route")
            .args(&["-n", "get", "default"])
            .output()
            .map_err(|e| format!("Failed to execute route command: {}", e))?;

        if !output.status.success() {
            return Err("Route command failed".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Look for "gateway: x.x.x.x" in the output
        for line in stdout.lines() {
            if line.contains("gateway:") {
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() > 1 {
                    let gateway = parts[1].trim();
                    // Remove any trailing parentheses or comments
                    let gateway = gateway.split(')').next().unwrap_or(gateway).trim();
                    if !gateway.is_empty() {
                        return Ok(gateway.to_string());
                    }
                }
            }
        }

        Err("No default gateway found".to_string())
    }

    /// Show notification alert using osascript
    pub fn show_notification(&self, title: &str, message: &str) {
        let _ = ProcessCommand::new("osascript")
            .args(&[
                "-e",
                &format!(r#"display notification "{}" with title "{}""#, message, title)
            ])
            .output();
    }

    /// Get list of network interfaces
    pub fn get_network_interfaces(&self) -> Result<Vec<String>, String> {
        let output = ProcessCommand::new("ifconfig")
            .output()
            .map_err(|e| format!("Failed to execute ifconfig: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut interfaces = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            // Interface names start at the beginning of a line and end with ':'
            if line.ends_with(":") && !line.is_empty() {
                let iface = line.trim_end_matches(':');
                if !iface.is_empty() && iface != "lo0" {
                    interfaces.push(iface.to_string());
                }
            }
        }

        Ok(interfaces)
    }
}
