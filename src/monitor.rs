use clap::{Arg, ArgAction, Command};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use ctrlc;


use crate::platform::PlatformImpl;

#[derive(Debug, Clone)]
struct Args {
    interface: String,
    interval: u64,
    target: String,
    packets: usize,
    popup: bool,
    test: bool,
    list_interfaces: bool,
}

impl Args {
    fn parse() -> Self {
        let matches = Command::new("packet_loss_monitor")
            .version("0.8.0")
            .about("Cross-platform packet loss monitoring tool")
            .arg(
                Arg::new("interface")
                    .long("interface")
                    .required(false)
                    .help("Network interface to monitor (required unless --list-interfaces is used)"),
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
            .arg(
                Arg::new("popup")
                    .long("popup")
                    .action(ArgAction::SetTrue)
                    .help("Show popup alert when packet loss is detected"),
            )
            .arg(
                Arg::new("test")
                    .long("test")
                    .action(ArgAction::SetTrue)
                    .help("Simulate packet loss (1 in 10 packets lost) for testing"),
            )
            .arg(
                Arg::new("list_interfaces")
                    .long("list-interfaces")
                    .action(ArgAction::SetTrue)
                    .help("List all available network interfaces and exit"),
            )
            .get_matches();

        // Handle --list-interfaces flag
        if matches.get_one::<bool>("list_interfaces").copied().unwrap_or(false) {
            let platform = PlatformImpl::new();
            match platform.get_network_interfaces() {
                Ok(interfaces) => {
                    println!("Available network interfaces:");
                    for iface in &interfaces {
                        println!("  - {}", iface);
                    }
                    println!("\nTotal: {} interfaces", interfaces.len());
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error listing interfaces: {}", e);
                    std::process::exit(1);
                }
            }
        }

        let interface = matches
            .get_one::<String>("interface")
            .map(|s| s.clone())
            .unwrap_or_else(|| {
                eprintln!("Error: --interface is required (or use --list-interfaces to see available interfaces)");
                std::process::exit(1);
            });

        let target = matches
            .get_one::<String>("target")
            .map(|s| s.clone())
            .unwrap_or_else(|| {
                PlatformImpl::new().get_default_gateway().unwrap_or_else(|_| "1.1.1.1".to_string())
            });

        Args {
            interface,
            interval: matches.get_one::<u64>("interval").unwrap().clone(),
            target,
            packets: matches.get_one::<usize>("packets").unwrap().clone(),
            popup: matches.get_one::<bool>("popup").copied().unwrap_or(false),
            test: matches.get_one::<bool>("test").copied().unwrap_or(false),
            list_interfaces: matches.get_one::<bool>("list_interfaces").copied().unwrap_or(false),
        }
    }
}

/// Shows a popup alert using platform-specific notification system
fn show_popup_alert(platform: &PlatformImpl, message: &str) {
    eprintln!("[ALERT] Showing notification: {}", message);
    platform.show_notification("Packet Loss Alert", message);
}

fn parse_packet_loss(platform: &PlatformImpl, stdout: &[u8]) -> f64 {
    platform.parse_ping_output(stdout)
}

pub fn main() {
    let args = Args::parse();
    
    let platform = PlatformImpl::new();
    
    eprintln!("[DEBUG] Interface: {}", args.interface);
    eprintln!("[DEBUG] Target: {}", args.target);
    eprintln!("[DEBUG] Interval: {}s", args.interval);
    eprintln!("[DEBUG] Packets per interval: {}", args.packets);
    eprintln!("[DEBUG] Popup alerts: {}", args.popup);
    eprintln!("[DEBUG] Test mode: {}", args.test);
    println!();
    
    println!(
        "Packet Loss Monitor - Monitoring {} for packet loss\n",
        args.interface
    );
    println!(
        "Target: {}, Interval: {}s, Packets per interval: {}",
        args.target, args.interval, args.packets
    );
    if args.popup {
        println!("Popup alerts: ENABLED");
    }
    if args.test {
        println!("Test mode: ENABLED (simulating 10% packet loss)");
    }
    println!("Press Ctrl+C to stop...\n");

    let results = Arc::new(Mutex::new(Vec::<f64>::new()));
    let stop = Arc::new(Mutex::new(false));

    // Set up signal handling for Ctrl+C
    let stop_clone = stop.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        *stop_clone.lock().unwrap() = true;
    }) {
        eprintln!("Error setting Ctrl-C handler: {}", e);
        std::process::exit(1);
    }

    let monitoring_thread = {
        let stop = stop.clone();
        let results = results.clone();
        let interface = args.interface.clone();
        let target = args.target.clone();
        let interval = args.interval;
        let packets = args.packets;
        let popup = args.popup;
        let test = args.test;
        let platform = platform.clone();
        
        thread::spawn(move || {
            let interval = Duration::from_secs(interval);
            
            loop {
                if *stop.lock().unwrap() {
                    break;
                }
                
                let output = platform.execute_ping(&target, packets);
                
                if let Ok(output) = output {
                    let packet_loss = if test {
                        // Simulate 10% packet loss in test mode
                        10.0
                    } else {
                        parse_packet_loss(&platform, &output.stdout)
                    };
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
                        
                        if popup {
                            show_popup_alert(&platform, &format!(
                                "Packet Loss Alert!\nInterface: {}\nLoss: {:.2}%",
                                interface, packet_loss
                            ));
                        }
                    }
                    println!();
                }
                
                thread::sleep(interval);
            }
        })
    };

    // Wait for monitoring thread to finish
    monitoring_thread.join().unwrap();

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
    fn test_args_parse_with_defaults() {
        // Test that Args can be created with default values
        let args = Args {
            interface: "eth0".to_string(),
            interval: 5u64,
            target: "1.1.1.1".to_string(),
            packets: 10usize,
            popup: false,
            test: false,
            list_interfaces: false,
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
            popup: true,
            test: true,
            list_interfaces: false,
        };
        
        assert_eq!(args.interface, "wlan0");
        assert_eq!(args.interval, 10);
        assert_eq!(args.target, "8.8.8.8");
        assert_eq!(args.packets, 20);
    }
}
