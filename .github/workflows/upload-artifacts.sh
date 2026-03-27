#!/bin/bash
set -e
shopt -u failglob  # Disable failglob to prevent errors on non-matching globs

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Upload Linux artifact
gh release upload "v$VERSION" artifacts/linux/packet_loss_monitor packet_loss_monitor-linux --clobber

# Upload macOS artifact
gh release upload "v$VERSION" artifacts/macos/packet_loss_monitor packet_loss_monitor-macos --clobber

# Upload Windows artifact
gh release upload "v$VERSION" artifacts/windows/packet_loss_monitor.exe packet_loss_monitor-windows.exe --clobber
