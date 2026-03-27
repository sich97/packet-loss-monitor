# Packet Loss Monitor

A cross-platform Rust tool to continuously monitor network interfaces for packet loss.

## Features

- **Cross-platform support**: Works on Linux, macOS, and Windows
- **Automatic gateway detection**: Automatically detects the default gateway
- **Real-time statistics display**: Shows packet loss percentage and statistics
- **Configurable monitoring**: Adjustable interval and packet count
- **Cross-platform notifications**: Desktop notifications when packet loss is detected
- **Interface listing**: List all available network interfaces

## Usage

```bash
# Build the tool
cargo build --release

# List available network interfaces
./target/release/packet_loss_monitor --list-interfaces

# Monitor packet loss on an interface (uses default gateway)
./target/release/packet_loss_monitor --interface eth0

# Monitor with custom settings
./target/release/packet_loss_monitor --interface eth0 --target 8.8.8.8 --interval 5 --count 10

# Enable popup notifications
./target/release/packet_loss_monitor --interface eth0 --popup

# Test mode (simulates 10% packet loss)
./target/release/packet_loss_monitor --interface eth0 --test
```

## Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--interface <iface>` | Network interface to monitor (required) | - |
| `--target <ip>` | Target IP for ping tests (defaults to gateway) | Auto-detect |
| `--interval <seconds>` | Monitoring interval in seconds | 5 |
| `--count <packets>` | Number of packets per interval | 10 |
| `--popup` | Show popup alert on packet loss | Disabled |
| `--test` | Simulate packet loss for testing | Disabled |
| `--list-interfaces` | List all available interfaces | - |

## Platform-Specific Notes

### Linux

- **Notifications**: Uses `notify-send` (D-Bus)
- **Gateway detection**: Reads from `/proc/net/route`
- **Interfaces**: Parses `/proc/net/dev`
- **Permissions**: May require `sudo` for some network operations

### macOS

- **Notifications**: Uses `osascript` (AppleScript)
- **Gateway detection**: Uses `route -n get default`
- **Interfaces**: Parses `ifconfig` output
- **Permissions**: May need to grant accessibility permissions for notifications

### Windows

- **Notifications**: Uses PowerShell toast notifications
- **Gateway detection**: Parses `route print -4` output
- **Interfaces**: Parses `ipconfig /all` output
- **Permissions**: May require Administrator privileges for some operations

## Requirements

- **Rust**: Stable toolchain (tested with Rust 1.70+)
- **Platform-specific tools**:
  - Linux: `ping`, `notify-send` (optional)
  - macOS: `ping`, `route`, `ifconfig`
  - Windows: `ping`, `route`, `ipconfig`

## Installation

```bash
# Clone repository
git clone <repository_url>
cd packet-loss-monitor

# Build
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Building for Different Platforms

### Cross-compilation

```bash
# Build for Linux (x86_64)
cargo build --release --target x86_64-unknown-linux-gnu

# Build for macOS (x86_64)
cargo build --release --target x86_64-apple-darwin

# Build for Windows (x86_64)
cargo build --release --target x86_64-pc-windows-gnu
```

## Example Output

```
Packet Loss Monitor - Monitoring eth0 for packet loss

Target: 192.168.1.1, Interval: 5s, Packets per interval: 10
Popup alerts: ENABLED
Press Ctrl+C to stop...

Loss: 0.00% (0 lost/10 sent)

Loss: 10.00% (1 lost/10 sent)
  ⚠ Warning: Packet loss detected on interface eth0

Loss: 0.00% (0 lost/10 sent)

Monitoring complete.
Average packet loss: 3.33%
Min: 0.00%, Max: 10.00%
```

## Troubleshooting

### Gateway Detection Fails

- **Linux**: Check `/proc/net/route` exists and is readable
- **macOS**: Run `route -n get default` manually to debug
- **Windows**: Run `route print -4` manually to debug

### Notifications Not Showing

- **Linux**: Install `libnotify-bin` package (`sudo apt install libnotify-bin`)
- **macOS**: Ensure Terminal has accessibility permissions in System Preferences
- **Windows**: Run as Administrator or check Windows notification settings

### Permission Errors

- **Linux**: Use `sudo` or add user to appropriate network groups
- **macOS**: May need to grant Terminal accessibility permissions
- **Windows**: Run as Administrator

## Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_test
cargo test --test integration_tests
```

## License

MIT
