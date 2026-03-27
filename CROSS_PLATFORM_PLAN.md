# Cross-Platform Compatibility Plan for Packet Loss Monitor

## Overview
This document outlines the plan to make the packet loss monitor application compatible with Linux, macOS, and Windows.

---

## 1. Ping Command Compatibility

### Current Implementation
- Uses `ping -c <count> <host>` (Linux/macOS syntax)

### Platform Differences
| Platform | Command Syntax |
|----------|---------------|
| Linux | `ping -c <count> <host>` |
| macOS | `ping -c <count> <host>` |
| Windows | `ping -n <count> <host>` |

### Implementation Plan
1. Create a `PingCommand` struct with platform-specific implementations
2. Use conditional compilation (`#[cfg(target_os = "...")]`)
3. Abstract the ping execution into a trait:
   ```rust
   trait PingExecutor {
       fn build_command(&self, host: &str, count: usize) -> ProcessCommand;
       fn parse_output(&self, stdout: &[u8]) -> f64;
   }
   ```
4. Implement for each platform:
   - `LinuxPingExecutor`
   - `MacOSPingExecutor`
   - `WindowsPingExecutor`

### Output Parsing
- Linux/macOS: `6 packets transmitted, 5 received, 16% packet loss`
- Windows: `Lost = 1 (25%), Loss = 25%`
- Need platform-specific parsers or a unified regex-based parser

---

## 2. Default Gateway Detection

### Current Implementation
- Reads `/proc/net/route` (Linux-only)

### Platform Differences
| Platform | Method | Command/File |
|----------|--------|--------------|
| Linux | Parse routing table | `/proc/net/route` |
| macOS | Route command | `route -n get default` |
| Windows | Route command | `route print` or PowerShell |

### Implementation Plan
1. Create a `GatewayDetector` trait:
   ```rust
   trait GatewayDetector {
       fn detect(&self) -> Result<String, String>;
   }
   ```

2. Platform-specific implementations:
   - **Linux**: Continue using `/proc/net/route` (already implemented)
   - **macOS**: Parse `route -n get default` output
     - Look for line containing "gateway:"
   - **Windows**: Parse `route print` output
     - Look for "0.0.0.0" or "0.0.0.0.0" row
     - Extract gateway from the first row

3. Factory pattern to select correct detector at runtime based on target OS

---

## 3. Network Interface Detection

### Current Implementation
- Interface is required as a command-line argument

### Enhancement Plan
1. Add optional `--list-interfaces` flag to show available interfaces
2. Create `InterfaceDetector` trait:
   ```rust
   trait InterfaceDetector {
       fn get_interfaces(&self) -> Result<Vec<NetworkInterface>, String>;
   }
   ```

3. Platform-specific implementations:
   - **Linux**: Parse `/proc/net/dev` or use `ip -o link`
   - **macOS**: Use `ifconfig` or `networksetup -listallhardwareports`
   - **Windows**: Use `GetAdaptersInfo` (WinAPI) or `ipconfig`

4. Consider using the `network-interface` crate for cross-platform support

---

## 4. Notification Alerts

### Current Implementation
- Uses `notify-send` (Linux D-Bus notifications)
- Falls back to `xmessage`

### Platform Differences
| Platform | Method | Command/Tool |
|----------|--------|--------------|
| Linux | D-Bus | `notify-send` |
| macOS | AppleScript | `osascript -e 'display notification'` |
| Windows | PowerShell | `Show-Notification` or Win32 API |

### Implementation Plan
1. Create `NotificationService` trait:
   ```rust
   trait NotificationService {
       fn send(&self, title: &str, message: &str) -> Result<(), String>;
   }
   ```

2. Platform-specific implementations:
   - **Linux**: Use `notify-send` (already implemented)
   - **macOS**: Use `osascript`:
     ```bash
     osascript -e 'display notification "Message" with title "Title"'
     ```
   - **Windows**: 
     - Option A: PowerShell with `[Windows.UI.Notifications...]`
     - Option B: Use `powershell -Command "Add-Type ..."`
     - Option C: Use a crate like `windows-notification`

3. Consider using the `notify-rust` crate for cross-platform notifications

---

## 5. File System & Paths

### Current Implementation
- Hardcoded `/proc/net/route` (Linux-only)

### Platform Differences
| Platform | Path Separator | Config Directory |
|----------|---------------|------------------|
| Linux | `/` | `~/.config/` |
| macOS | `/` | `~/Library/Application Support/` |
| Windows | `\` | `%APPDATA%` |

### Implementation Plan
1. Use `std::path::Path` for all file operations
2. Use `std::env::var()` for environment variables:
   - Linux/macOS: `HOME`
   - Windows: `USERPROFILE`
3. Consider adding config file support:
   - `~/.packet-loss-monitor/config.toml` or `~/.config/packet-loss-monitor/config.toml`
4. Use the `dirs` crate for platform-specific directories

---

## 6. Signal Handling

### Current Implementation
- Reads from stdin to detect Ctrl+C

### Platform Differences
| Platform | Signal Handling |
|----------|----------------|
| Linux/macOS | Unix signals (SIGINT) |
| Windows | Console control handlers |

### Implementation Plan
1. Use the `ctrlc` crate for cross-platform Ctrl+C handling:
   ```rust
   use ctrlc;
   
   ctrlc::set_handler(move || {
       // cleanup code
   }).expect("Error setting Ctrl-C handler");
   ```

2. Or use `tokio::signal` if migrating to async runtime

---

## 7. Dependencies

### Current Dependencies
- `clap`: CLI parsing (cross-platform ✓)
- `tokio`: Async runtime (not currently used)
- `ping`: Network ping (not currently used - uses std::process::Command)

### Recommended Additions
1. **`network-interface`**: Cross-platform network interface detection
2. **`notify-rust`**: Cross-platform notifications
3. **`ctrlc`**: Cross-platform signal handling
4. **`dirs`**: Platform-specific directories
5. **`regex`**: For parsing ping output (more robust)

### Conditional Dependencies
Use Cargo features for platform-specific dependencies:
```toml
[dependencies]
clap = "4.0"
regex = "1.0"

[target.'cfg(target_os = "linux")'.dependencies]
notify-rust = "4.0"

[target.'cfg(target_os = "macos")'.dependencies]
notify-rust = "4.0"

[target.'cfg(target_os = "windows")'.dependencies]
windows-notification = "0.1"
```

---

## 8. Code Structure Changes

### Proposed Module Structure
```
src/
├── main.rs           # Entry point
├── monitor.rs        # Main monitoring logic
├── platform/
│   ├── mod.rs        # Platform module
│   ├── linux.rs      # Linux-specific implementations
│   ├── macos.rs      # macOS-specific implementations
│   └── windows.rs    # Windows-specific implementations
├── ping/
│   ├── mod.rs        # Ping abstraction
│   └── executor.rs   # Ping command executor
├── gateway/
│   ├── mod.rs        # Gateway detection abstraction
│   └── detector.rs   # Gateway detector
├── notification/
│   ├── mod.rs        # Notification abstraction
│   └── service.rs    # Notification service
└── interface/
    ├── mod.rs        # Interface detection abstraction
    └── detector.rs   # Interface detector
```

### Alternative: Simpler Approach
If the full module structure is too complex, use conditional compilation in a single file:

```rust
#[cfg(target_os = "linux")]
mod platform_impl {
    include!("platform/linux.rs");
}

#[cfg(target_os = "macos")]
mod platform_impl {
    include!("platform/macos.rs");
}

#[cfg(target_os = "windows")]
mod platform_impl {
    include!("platform/windows.rs");
}
```

---

## 9. Testing Strategy

### Unit Tests
- Keep existing tests (they should still work)
- Add tests for each platform-specific module
- Use conditional compilation for platform-specific tests

### Integration Tests
1. Create platform-specific integration tests
2. Use environment variables to detect test platform
3. Consider using GitHub Actions for cross-platform testing:
   - Ubuntu (Linux)
   - macOS
   - Windows

### Manual Testing Checklist
- [ ] Linux (Ubuntu, Debian, Fedora)
- [ ] macOS (latest 2 versions)
- [ ] Windows (10, 11)
- [ ] Test with different network interfaces
- [ ] Test notification system on each platform
- [ ] Test gateway detection on each platform

---

## 10. Implementation Phases

### Phase 1: Core Abstractions (Week 1)
- [ ] Create trait abstractions for ping, gateway, notifications
- [ ] Implement Linux versions (already mostly done)
- [ ] Add `ctrlc` crate for signal handling
- [ ] Update Cargo.toml with new dependencies

### Phase 2: macOS Support (Week 2)
- [ ] Implement macOS ping executor
- [ ] Implement macOS gateway detector
- [ ] Implement macOS notification service
- [ ] Test on macOS

### Phase 3: Windows Support (Week 3)
- [ ] Implement Windows ping executor
- [ ] Implement Windows gateway detector
- [ ] Implement Windows notification service
- [ ] Test on Windows

### Phase 4: Polish & Documentation (Week 4)
- [ ] Add `--list-interfaces` flag
- [ ] Add config file support
- [ ] Update README with cross-platform instructions
- [ ] Create platform-specific documentation
- [ ] Add cross-platform CI/CD pipeline

---

## 11. Potential Challenges & Solutions

### Challenge 1: Ping Output Format Differences
**Solution**: Use regex-based parsing that handles multiple formats

### Challenge 2: Windows Administrator Privileges
**Solution**: 
- Detect if running with sufficient privileges
- Show helpful error message if admin rights needed
- Consider using Windows Network List Service API

### Challenge 3: macOS Security Permissions
**Solution**:
- Add instructions for granting accessibility permissions
- Use `osascript` which doesn't require special permissions

### Challenge 4: Cross-Platform Build Issues
**Solution**:
- Use GitHub Actions with matrix builds
- Test on all platforms before release
- Consider using `cross` for cross-compilation

---

## 12. Success Criteria

The application will be considered cross-platform compatible when:
1. ✓ Builds successfully on Linux, macOS, and Windows
2. ✓ Detects default gateway correctly on all platforms
3. ✓ Sends notifications on all platforms
4. ✓ Parses ping output correctly on all platforms
5. ✓ Handles Ctrl+C gracefully on all platforms
6. ✓ All existing tests pass on all platforms
7. ✓ Documentation includes platform-specific instructions

---

## 13. Timeline Estimate

| Phase | Duration | Deliverables |
|-------|----------|--------------|
| Phase 1 | 1 week | Core abstractions, Linux improvements |
| Phase 2 | 1 week | Full macOS support |
| Phase 3 | 1 week | Full Windows support |
| Phase 4 | 1 week | Documentation, CI/CD, polish |
| **Total** | **4 weeks** | **Cross-platform release** |

---

## 14. Next Steps

1. **Review and approve this plan**
2. **Set up development environment** for all target platforms
3. **Start with Phase 1**: Create trait abstractions
4. **Implement incrementally**: Test each platform before moving to the next
5. **Update AGENTS.md** with cross-platform development notes

---

## 15. References

- Rust cross-platform development: https://doc.rust-lang.org/book/ch14-03-generating-cross-platform-code.html
- `network-interface` crate: https://crates.io/crates/network-interface
- `notify-rust` crate: https://crates.io/crates/notify-rust
- Windows notifications: https://docs.microsoft.com/en-us/windows/uwp/design/apps/notifications
- macOS notifications: https://developer.apple.com/documentation/appkit/nsusernotification
