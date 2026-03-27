//! Linux-specific platform implementation

use std::process::Command as ProcessCommand;

#[derive(Clone)]
pub struct PlatformImpl;

impl PlatformImpl {
    pub fn new() -> Self {
        Self
    }

    /// Execute ping command on Linux
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
        
        // Linux format: "6 packets transmitted, 5 received, 16% packet loss, time 5006ms"
        regex::Regex::new(r"(\d+)%\s*packet\s+loss")
            .ok()
            .and_then(|re| re.captures(&stdout))
            .and_then(|caps| caps.get(1))
            .and_then(|match_| match_.as_str().parse::<f64>().ok())
            .unwrap_or(0.0)
    }

    /// Get default gateway from /proc/net/route
    pub fn get_default_gateway(&self) -> Result<String, String> {
        use std::fs;
        use std::net::Ipv4Addr;

        let route_content = fs::read_to_string("/proc/net/route")
            .map_err(|e| format!("Failed to read /proc/net/route: {}", e))?;

        for line in route_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            // Skip header line
            if parts.is_empty() || parts[0] == "Iface" {
                continue;
            }

            // Check if this is the default route (destination == 00000000)
            if parts[1] == "00000000" {
                // Gateway is in hex format, convert to IP
                let gateway_hex = if parts[2].starts_with("0x") {
                    &parts[2][2..]
                } else {
                    &parts[2]
                };
                
                match u32::from_str_radix(gateway_hex, 16) {
                    Ok(gateway_u32) => {
                        // /proc/net/route stores IPs in network byte order (big-endian)
                        let gateway_ip = Ipv4Addr::from(u32::from_be(gateway_u32));
                        return Ok(gateway_ip.to_string());
                    }
                    Err(e) => {
                        return Err(format!("Invalid gateway hex '{}': {}", gateway_hex, e));
                    }
                }
            }
        }

        Err("No default gateway found".to_string())
    }

    /// Show notification alert using notify-send
    pub fn show_notification(&self, title: &str, message: &str) {
        let _ = ProcessCommand::new("notify-send")
            .args(&[
                "-u", "normal",
                "-t", "5000",
                title,
                message
            ])
            .output();
    }

    /// Get list of network interfaces
    pub fn get_network_interfaces(&self) -> Result<Vec<String>, String> {
        use std::fs;
        
        let dev_content = fs::read_to_string("/proc/net/dev")
            .map_err(|e| format!("Failed to read /proc/net/dev: {}", e))?;

        let mut interfaces = Vec::new();
        
        for line in dev_content.lines() {
            let line = line.trim();
            if line.contains(":") && !line.starts_with("Inter-") && !line.is_empty() {
                let parts: Vec<&str> = line.split(":").collect();
                let iface = parts[0].trim();
                if !iface.is_empty() && iface != "lo" {
                    interfaces.push(iface.to_string());
                }
            }
        }

        Ok(interfaces)
    }
}
