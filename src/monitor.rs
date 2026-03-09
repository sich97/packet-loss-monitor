use clap::{ArgMatches, Command};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Args {
    interface: String,
    interval: u64,
    target: String,
    count: usize,
}

impl Args {
    fn parse() -> Self {
        let matches = Command::version("packet_loss_monitor 0.1")
            .about("Lightweight packet loss monitoring tool")
            .arg(
                clap::Arg::new("interface")
                    .required(true)
                    .help("Network interface to monitor"),
            )
            .arg(
                clap::Arg::new("target")
                    .long("target")
                    .default("1.1.1.1")
                    .help("Target IP for ping tests"),
            )
            .arg(
                clap::Arg::new("interval")
                    .long("interval")
                    .default(5)
                    .value_name("seconds")
                    .help("Monitoring interval in seconds"),
            )
            .arg(
                clap::Arg::new("count")
                    .long("count")
                    .default(10)
                    .value_name("packets")
                    .help("Packets to send per interval"),
            )
            .get_matches();

        Args {
            interface: matches.get::<String>("interface").cloned().unwrap(),
            interval: matches.get::<u64>("interval").unwrap(),
            target: matches.get::<String>("target").cloned().unwrap(),
            count: matches.get::<usize>("count").unwrap(),
        }
    }
}

fn parse_packet_loss(stdout: &[u8]) -> f64 {
    let stdout = String::from_utf8_lossy(&stdout);
    stdout.lines()
        .find(|line| line.contains("packet loss"))
        .and_then(|line| line.split("packet loss").nth(1))
        .and_then(|s| s.split("%").next())
        .and_then(|s| s.trim().parse::<f64>().ok())
        .unwrap_or(0.0)
}

fn main() {
    let args = Args::parse();
    
    println!(
        "Packet Loss Monitor - Monitoring {} for packet loss\n",
        args.interface
    );
    println!(
        "Target: {}, Interval: {}s, Packets per interval: {}",
        args.target, args.interval, args.count
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
        let count = args.count;
        
        thread::spawn(move || {
            let interval = Duration::from_secs(interval);
            
            loop {
                if *stop.lock().unwrap() {
                    break;
                }
                
                let output = Command::new("ping")
                    .arg("-c")
                    .arg(count.to_string())
                    .arg(target.clone())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::null())
                    .output();
                
                if let Ok(output) = output {
                    let packet_loss = parse_packet_loss(&output.stdout);
                    results.lock().unwrap().push(packet_loss);
                    
                    let total_packets = count;
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