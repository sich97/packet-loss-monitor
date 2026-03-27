#!/bin/bash
set -e

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Upload Linux artifact
gh release upload "v$VERSION" artifacts/linux/packet_loss_monitor "$(printf '%s' 'packet_loss_monitor-linux')" --clobber

# Upload macOS artifact
gh release upload "v$VERSION" artifacts/macos/packet_loss_monitor "$(printf '%s' 'packet_loss_monitor-macos')" --clobber

# Upload Windows artifact
gh release upload "v$VERSION" artifacts/windows/packet_loss_monitor.exe "$(printf '%s' 'packet_loss_monitor-windows.exe')" --clobber
