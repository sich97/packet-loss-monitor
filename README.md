# Packet Loss Monitor

A lightweight Rust tool to continuously monitor a given network interface for packet loss.

## Features

- Monitors packet loss using ICMP ping
- Real-time statistics display
- Configurable packet interval
- Linux compatible

## Usage

```bash
# Build the tool
cargo build --release

# Monitor packet loss on an interface
sudo target/release/packet_loss_monitor <interface_name>

# Example:
sudo target/release/packet_loss_monitor eth0

# Monitor with custom packet interval (milliseconds)
sudo target/release/packet_loss_monitor eth0 500
```

## Example Output

```
Monitoring packet loss on eth0 (interval: 200ms)
Lost: 0/100 packets (0.00%)
Lost: 2/120 packets (1.67%)
Lost: 2/140 packets (1.43%)
Lost: 3/160 packets (1.88%)
```

## Requirements

- Linux host
- Rust compiler
- Administrative privileges (sudo) for network access

## Installation

```bash
# Clone repository
git clone <repository_url>
cd packet_loss_monitor

# Build
cargo build --release
```

## License

MIT
# Release Notes
