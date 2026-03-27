#!/bin/bash
set -e

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

# Upload Linux artifact
eval "gh release upload \"v$VERSION\" artifacts/packet_loss_monitor-x86_64-unknown-linux-gnu/packet_loss_monitor packet_loss_monitor-linux --clobber"

# Upload macOS artifact
eval "gh release upload \"v$VERSION\" artifacts/packet_loss_monitor-x86_64-apple-darwin/packet_loss_monitor packet_loss_monitor-macos --clobber"

# Upload Windows artifact
eval "gh release upload \"v$VERSION\" artifacts/packet_loss_monitor-x86_64-pc-windows-gnu/packet_loss_monitor.exe packet_loss_monitor-windows.exe --clobber"
