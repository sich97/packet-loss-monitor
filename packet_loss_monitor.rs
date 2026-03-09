use std::net::IpAddr;
use std::time::Duration;
use clap::ArgMatches;
use if_addrs::get_if_addrs;
use ping::Pinger;

/// Packet Loss Monitor
/// A lightweight tool to continuously monitor network interfaces for packet loss

struct MonitorConfig {
    interface: String,
    target: IpAddr,
    interval: u64,
    count: usize,
}

fn parse_args(args: ArgMatches) -> Result<MonitorConfig, String> {
    let interface = args.value_of("INTERFACE").unwrap().to_string();
    let target = args.value_of("TARGET")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid target IP address")?;
    let interval = args.value_of("INTERVAL")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid interval")?;
    let count = args.value_of("COUNT")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid packet count")?;

    Ok(MonitorConfig {
        interface,
        target,
        interval,
        count,
    })
}

fn get_interface_index(interface_name: &str) -> Option<u32> {
    get_if_addrs()
        .ok()
        .and_then(|interfaces| {
            interfaces
                .find(|iface| iface.name == interface_name)
                .map(|iface| iface.index)
        })
}

fn monitor_packet_loss(config: MonitorConfig) -> Result<(), String> {
    let pinger = Pinger::new()
        .with_target(config.target)
        .with_interval(Duration::from_secs(config.interval))
        .with_count(config.count)
        .with_timeout(Duration::from_secs(2))
        .with_interface(get_interface_index(&config.interface))
        .clone();

    let results = pinger.ping()
        .map_err(|e| format!("Ping failed: {}", e))?;

    let lost = results.iter().filter(|r| r.timeout || r.received == 0).count();
    let total = results.len();
    let loss_percent = (lost as f64 / total as f64) * 100.0;

    println!(
        "Interface: {} | Target: {} | Lost: {}/{} ({:.2}%)",
        config.interface,
        config.target,
        lost,
        total,
        loss_percent
    );

    Ok(())
}

fn main() {
    let matches = clap::App::new("Packet Loss Monitor")
        .version("1.0")
        .about("Monitor network interface for packet loss")
        .arg(
            clap::Arg::with_name("INTERFACE")
                .required(true)
                .help("Network interface to monitor"),
        )
        .arg(
            clap::Arg::with_name("TARGET")
                .required(true)
                .help("Target IP address to ping"),
        )
        .arg(
            clap::Arg::with_name("INTERVAL")
                .long("interval")
                .value_name("seconds")
                .default_value("1")
                .help("Ping interval in seconds"),
        )
        .arg(
            clap::Arg::with_name("COUNT")
                .long("count")
                .value_name("packets")
                .default_value("10")
                .help("Number of packets to send"),
        )
        .take_args()
        .get_matches();

    let config = match parse_args(matches) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        if let Err(e) = monitor_packet_loss(config.clone()) {
            eprintln!("Monitoring error: {}", e);
        }
        std::thread::sleep(Duration::from_secs(config.interval));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_interface_index() {
            let index = get_interface_index("lo");
            assert!(index.is_some());
        }

        #[test]
        fn test_get_interface_index_nonexistent() {
            let index = get_interface_index("nonexistent_interface");
            assert!(index.is_none());
        }
    }
}
