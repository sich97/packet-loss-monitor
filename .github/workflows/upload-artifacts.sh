#!/bin/bash
set -e
set +f  # Disable glob expansion to prevent errors on non-matching globs

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Upload Linux artifact
gh release upload "v$VERSION" 'artifacts/linux/packet_loss_monitor-x86_64-unknown-linux-gnu' 'packet_loss_monitor-linux' --clobber

# Upload macOS artifact
gh release upload "v$VERSION" 'artifacts/macos/packet_loss_monitor-x86_64-apple-darwin' 'packet_loss_monitor-macos' --clobber

# Upload Windows artifact
gh release upload "v$VERSION" 'artifacts/windows/packet_loss_monitor-x86_64-pc-windows-gnu.exe' 'packet_loss_monitor-windows.exe' --clobber
