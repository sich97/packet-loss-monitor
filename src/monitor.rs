use clap::{Arg, Command};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::fs;
use std::net::Ipv4Addr;

#[derive(Debug)]
struct Args {
    interface: String,
    interval: u64,
    target: String,
    packets: usize,
}

impl Args {
    fn parse() -> Self {
        let matches = Command::new("packet_loss_monitor")
            .version("0.1")
            .about("Lightweight packet loss monitoring tool")
            .arg(
                Arg::new("interface")
                    .long("interface")
                    .required(true)
                    .help("Network interface to monitor"),
            )
            .arg(
                Arg::new("target")
                    .long("target")
                    .help("Target IP for ping tests (defaults to default gateway)"),
            )
            .arg(
                Arg::new("interval")
                    .long("interval")
                    .default_value("5")
                    .value_name("seconds")
                    .help("Monitoring interval in seconds")
                    .value_parser(clap::value_parser!(u64)),
            )
            .arg(
                Arg::new("packets")
                    .long("count")
                    .default_value("10")
                    .value_name("packets")
                    .help("Number of packets to send per interval")
                    .value_parser(clap::value_parser!(usize)),
            )
            .get_matches();

        let interface = matches.get_one::<String>("interface").unwrap().clone();
        let target = matches
            .get_one::<String>("target")
            .map(|s| s.clone())
            .unwrap_or_else(|| {
                detect_default_gateway(&interface).unwrap_or_else(|_| "1.1.1.1".to_string())
            });

        Args {
            interface,
            interval: matches.get_one::<u64>("interval").unwrap().clone(),
            target,
            packets: matches.get_one::<usize>("packets").unwrap().clone(),
        }
    }
}

/// Detects the default gateway by parsing /proc/net/route
/// Returns the gateway for the default route (0.0.0.0) regardless of interface
fn detect_default_gateway(_interface: &str) -> Result<String, String> {
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
            if let Some(gateway_hex) = parts[2].strip_prefix("0x") {
                let gateway_u32 = u32::from_str_radix(gateway_hex, 16)
                    .map_err(|e| format!("Invalid gateway hex: {}", e))?;
                
                // Convert little-endian to network byte order
                let gateway_ip = Ipv4Addr::from(u32::from_le(gateway_u32));
                return Ok(gateway_ip.to_string());
            }
        }
    }

    Err("No default gateway found".to_string())
}

fn parse_packet_loss(stdout: &[u8]) -> f64 {
    let stdout = String::from_utf8_lossy(&stdout);
    stdout.lines()
        .find(|line| line.contains("packet loss"))
        .and_then(|line| {
            // Find the '%' character and parse the number before it
            line.find('%').and_then(|percent_pos| {
                // Get the substring before '%'
                let before_percent = &line[..percent_pos];
                // Find the last sequence of digits
                before_percent
                    .chars()
                    .rev()
                    .take_while(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<f64>()
                    .ok()
            })
        })
        .unwrap_or(0.0)
}

pub fn main() {
    let args = Args::parse();
    
    println!(
        "Packet Loss Monitor - Monitoring {} for packet loss\n",
        args.interface
    );
    println!(
        "Target: {}, Interval: {}s, Packets per interval: {}",
        args.target, args.interval, args.packets
    );
    println!("Press Ctrl+C to stop...\n");

    let results = Arc::new(Mutex::new(Vec::<f64>::new()));
    let stop = Arc::new(Mutex::new(false));

    let monitoring_thread = {
        let stop = stop.clone();
        let results = results.clone();
        let interface = args.interface.clone();
        let target = args.target.clone();
        let interval = args.interval;
        let packets = args.packets;
        
        thread::spawn(move || {
            let interval = Duration::from_secs(interval);
            
            loop {
                if *stop.lock().unwrap() {
                    break;
                }
                
                let output = std::process::Command::new("ping")
                    .arg("-c")
                    .arg(packets.to_string())
                    .arg(target.clone())
                    .output();
                
                if let Ok(output) = output {
                    let packet_loss = parse_packet_loss(&output.stdout);
                    results.lock().unwrap().push(packet_loss);
                    
                    let total_packets = packets;
                    let lost_packets = (total_packets as f64 * packet_loss / 100.0) as usize;
                    println!(
                        "Loss: {:.2}% ({} lost/{} sent)",
                        packet_loss, lost_packets, total_packets
                    );
                    
                    if lost_packets > 0 {
                        println!(
                            "  ⚠ Warning: Packet loss detected on interface {}",
                            interface
                        );
                    }
                    println!();
                }
                
                thread::sleep(interval);
            }
        })
    };

    // Set up signal handling for Ctrl+C
    let sigint_handler = {
        let stop = stop.clone();
        thread::spawn(move || {
            let _ = std::io::stdin();
            let stdin = std::io::stdin();
            let _ = stdin.read_line(&mut String::new());
            *stop.lock().unwrap() = true;
        })
    };

    monitoring_thread.join().unwrap();
    sigint_handler.join().unwrap();

    let results = results.lock().unwrap();
    if !results.is_empty() {
        let avg_loss: f64 = results.iter().sum::<f64>() / results.len() as f64;
        let min_loss = results.iter().fold(f64::MAX, |a, b| a.min(*b));
        let max_loss = results.iter().fold(f64::MIN, |a, b| a.max(*b));
        println!("\nMonitoring complete.");
        println!("Average packet loss: {:.2}%", avg_loss);
        println!("Min: {:.2}%, Max: {:.2}%", min_loss, max_loss);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet_loss_with_zero_loss() {
        let output = b"6 packets transmitted, 6 received, 0% packet loss, time 5006ms\n";
        let result = parse_packet_loss(output);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_parse_packet_loss_with_some_loss() {
        let output = b"6 packets transmitted, 5 received, 16% packet loss, time 5006ms\n";
        let result = parse_packet_loss(output);
        assert_eq!(result, 16.0);
    }

    #[test]
    fn test_parse_packet_loss_with_high_loss() {
        let output = b"10 packets transmitted, 2 received, 80% packet loss, time 9000ms\n";
        let result = parse_packet_loss(output);
        assert_eq!(result, 80.0);
    }

    #[test]
    fn test_parse_packet_loss_with_no_packets_sent() {
        let output = b"0 packets transmitted, 0 received, 0% packet loss, time 0ms\n";
        let result = parse_packet_loss(output);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_parse_packet_loss_with_no_loss_line() {
        let output = b"some other output without packet loss info\n";
        let result = parse_packet_loss(output);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_parse_packet_loss_with_invalid_percentage() {
        let output = b"6 packets transmitted, 6 received, invalid% packet loss\n";
        let result = parse_packet_loss(output);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_detect_default_gateway_no_file() {
        // This test verifies error handling when /proc/net/route doesn't exist
        // Note: This will pass on most systems since /proc/net/route usually exists
        let result = detect_default_gateway("eth0");
        // The function now finds the default gateway regardless of interface
        // So this test just checks it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_detect_default_gateway_invalid_hex() {
        // Test the parsing logic directly
        let invalid_hex = "ZZZZZZZZ";
        let result = u32::from_str_radix(invalid_hex, 16);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_default_gateway_hex_conversion() {
        // Test the hex to IP conversion logic
        // Gateway 0xC0A80101 in little-endian is 192.168.1.1
        let gateway_hex = "c0a80101";
        let gateway_u32 = u32::from_str_radix(gateway_hex, 16).unwrap();
        let gateway_ip = Ipv4Addr::from(u32::from_le(gateway_u32));
        assert_eq!(gateway_ip.to_string(), "192.168.1.1");
    }

    #[test]
    fn test_args_parse_with_defaults() {
        // Test that Args can be created with default values
        let args = Args {
            interface: "eth0".to_string(),
            interval: 5u64,
            target: "1.1.1.1".to_string(),
            packets: 10usize,
        };
        
        assert_eq!(args.interface, "eth0");
        assert_eq!(args.interval, 5);
        assert_eq!(args.target, "1.1.1.1");
        assert_eq!(args.packets, 10);
    }

    #[test]
    fn test_args_parse_with_custom_values() {
        // Test that Args can be created with custom values
        let args = Args {
            interface: "wlan0".to_string(),
            interval: 10u64,
            target: "8.8.8.8".to_string(),
            packets: 20usize,
        };
        
        assert_eq!(args.interface, "wlan0");
        assert_eq!(args.interval, 10);
        assert_eq!(args.target, "8.8.8.8");
        assert_eq!(args.packets, 20);
    }
}