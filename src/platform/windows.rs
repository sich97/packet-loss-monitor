//! Windows-specific platform implementation

use std::process::Command as ProcessCommand;

#[derive(Clone)]
pub struct PlatformImpl;

impl PlatformImpl {
    pub fn new() -> Self {
        Self
    }

    /// Execute ping command on Windows
    pub fn execute_ping(&self, target: &str, count: usize) -> std::io::Result<std::process::Output> {
        ProcessCommand::new("ping")
            .arg("-n")
            .arg(count.to_string())
            .arg(target)
            .output()
    }

    /// Parse ping output to extract packet loss percentage
    pub fn parse_ping_output(&self, stdout: &[u8]) -> f64 {
        let stdout = String::from_utf8_lossy(&stdout);
        
        // Windows format: "Lost = 1 (25%), Loss = 25%"
        // or "Packets: Sent = 4, Received = 4, Lost = 0 (0% loss)"
        let re = regex::Regex::new(r"(?:Lost\s*=\s*\d+\s*\((\d+)%\)|Loss\s*=\s*(\d+)%|\d+%\s+loss)").ok();
        
        if let Some(re) = re {
            if let Some(caps) = re.captures(&stdout) {
                if let Some(match_) = caps.get(1) {
                    return match_.as_str().parse::<f64>().unwrap_or(0.0);
                }
                if let Some(match_) = caps.get(2) {
                    return match_.as_str().parse::<f64>().unwrap_or(0.0);
                }
            }
        }

        0.0
    }

    /// Get default gateway using route command
    pub fn get_default_gateway(&self) -> Result<String, String> {
        let output = ProcessCommand::new("cmd")
            .args(&["/c", "route", "print", "-4"])
            .output()
            .map_err(|e| format!("Failed to execute route command: {}", e))?;

        if !output.status.success() {
            return Err("Route command failed".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Look for the Default Gateway line under each interface
        // Format:
        //   0.0.0.0          0.0.0.0          192.168.1.1
        for line in stdout.lines() {
            let line = line.trim();
            // Look for lines starting with 0.0.0.0 (default route)
            if line.starts_with("0.0.0.0") && line.contains("0.0.0.0") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let gateway = parts[2];
                    if !gateway.is_empty() && gateway != "0.0.0.0" {
                        return Ok(gateway.to_string());
                    }
                }
            }
        }

        Err("No default gateway found".to_string())
    }

    /// Show notification alert using PowerShell
    pub fn show_notification(&self, title: &str, message: &str) {
        // Use PowerShell to show a toast notification
        let script = format!(
            r#"Add-Type -AssemblyName System.Windows.Forms; 
            Add-Type -AssemblyName System.Drawing; 
            $notification = [System.Windows.Forms.ToastNotification]::new(); 
            $notification.Title = "{}"; 
            $notification.Content = "{}"; 
            $notification.Show()"#,
            title, message
        );

        let _ = ProcessCommand::new("powershell")
            .args(&["-Command", &script])
            .output();
    }

    /// Get list of network interfaces
    pub fn get_network_interfaces(&self) -> Result<Vec<String>, String> {
        let output = ProcessCommand::new("cmd")
            .args(&["/c", "ipconfig", "/all"])
            .output()
            .map_err(|e| format!("Failed to execute ipconfig: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut interfaces = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            // Look for "Ethernet adapter" or "Wireless LAN adapter" lines
            if line.starts_with("Ethernet adapter") || line.starts_with("Wireless LAN adapter") {
                // Extract the adapter name (everything after the colon)
                if let Some(pos) = line.find(':') {
                    let name = line[pos + 1..].trim();
                    if !name.is_empty() {
                        interfaces.push(name.to_string());
                    }
                }
            }
        }

        Ok(interfaces)
    }
}
