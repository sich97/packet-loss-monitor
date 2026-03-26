# AGENTS.md - Packet Loss Monitor Development Guide

## Project Overview
**Repository**: `sich97/packet-loss-monitor`  
**Language**: Rust  
**Purpose**: A CLI tool for monitoring network packet loss

---

## Development Environment Setup

### Prerequisites
- Rust (stable toolchain)
- Git
- GitHub CLI (`gh`) for release management

### Local Development
```bash
cd packet-loss-monitor
cargo build          # Debug build
cargo build --release  # Release build
cargo test           # Run tests
```

---

## GitHub Actions Workflow (`rust-ci.yml`)

### Workflow Structure
The workflow has two jobs:
1. **test**: Runs on every push/PR, executes `cargo test`
2. **release**: Runs only on pushes to `main` with commit messages starting with "Release"

### Key Configuration
```yaml
permissions:
  contents: write  # Required for creating releases with GITHUB_TOKEN
```

### Release Trigger Conditions
The release job only runs when:
- Event is a push to `main` branch
- Commit message starts with "Release" (e.g., "Release v0.1.0")

### GitHub CLI Installation
The workflow installs GitHub CLI using:
```bash
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh -y
```

**Important**: All apt commands must use `sudo` due to permission requirements.

---

## Release Process

### Creating a Release
1. Commit changes (if any)
2. Create a release commit:
   ```bash
   git commit --allow-empty -m "Release v0.1.0"
   git push origin main
   ```
3. Wait for workflow to complete (~2-3 minutes)
4. Verify release at: `https://github.com/sich97/packet-loss-monitor/releases`

### Release Command
The workflow uses `gh release create` with these flags:
```bash
gh release create v0.1.0 \
  --title "Release v0.1.0" \
  --notes "Automated release" \
  --draft=false \
  --prerelease=false \
  --repo sich97/packet-loss-monitor \
  target/release/packet_loss_monitor
```

**Note**: The `--repo` flag is required for proper asset attachment.

---

## Common Issues & Solutions

### Issue: HTTP 403 when creating releases
**Cause**: Missing `contents: write` permission in workflow  
**Solution**: Add `permissions: contents: write` at workflow level

### Issue: "tee: Permission denied" during CLI installation
**Cause**: Missing `sudo` for apt commands  
**Solution**: Add `sudo` to all apt-related commands

### Issue: "release with same tag already exists"
**Cause**: Tag already exists from previous release  
**Solution**: Delete old release via API or use new version number

### Issue: Release job skipped
**Cause**: Commit message doesn't start with "Release"  
**Solution**: Ensure commit message follows pattern "Release vX.Y.Z"

---

## Code Structure

### Main Source Files
- `src/monitor.rs`: Core monitoring logic and CLI argument parsing
- `src/main.rs`: Entry point
- `integration_tests.rust`: Integration tests

### CLI Arguments
The `packets` argument uses `clap::value_parser!(usize)` for proper type validation.

---

## Version Control

### Branch Strategy
- `main`: Production-ready code
- Feature branches: Created for new features/fixes

### Commit Message Convention
- Feature/fix: "feat: description" or "fix: description"
- Release: "Release vX.Y.Z" (triggers automated release)

---

## Testing

### Running Tests
```bash
cargo test --workspace
```

### Integration Tests
Located in `integration_tests.rust`

---

## Dependencies

### Rust Dependencies
- `clap`: CLI argument parsing
- `tokio`: Async runtime
- `ping`: Network ping functionality

### GitHub Actions Dependencies
- `dtolnay/rust-toolchain@stable`: Rust toolchain (replaced deprecated `actions-rs/setup-rust`)
- `actions/checkout@v4`: Code checkout
- `softprops/action-gh-release@v1`: **DEPRECATED** - Use `gh release create` instead

---

## API Usage

### GitHub API for Release Management
```bash
# Get latest release
curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/releases/latest"

# Delete release
curl -s -X DELETE -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/releases/<RELEASE_ID>"

# Check workflow runs
curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/actions/runs?per_page=1"
```

---

## Debugging Workflows

### Getting Workflow Logs
```bash
# Get run ID
curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/actions/runs?per_page=1" | \
  python3 -c "import sys, json; data = json.load(sys.stdin); print(data['workflow_runs'][0]['id'])"

# Get job logs
curl -sL -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/actions/jobs/<JOB_ID>/logs" | \
  tail -50
```

### Checking Job Status
```bash
curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/actions/runs/<RUN_ID>/jobs" | \
  python3 -c "import sys, json; data = json.load(sys.stdin); [print(f\"{j['name']}: {j['conclusion']}\") for j in data['jobs']]"
```

---

## Best Practices

1. **Always use `sudo`** for system package installation in GitHub Actions
2. **Add `permissions: contents: write`** when creating releases
3. **Use `--repo` flag** with `gh release create` for proper asset attachment
4. **Test locally** before pushing to ensure builds succeed
5. **Use descriptive commit messages** starting with "Release" for automated releases
6. **Verify releases** via GitHub API after workflow completion

---

## Future Improvements

1. Consider using dynamic versioning (e.g., based on git tags)
2. Add release notes generation from changelog
3. Implement multi-platform builds (Linux, macOS, Windows)
4. Add artifact retention policies
5. Consider using `softprops/action-gh-release@v2` if v1 continues to have issues

---

## Quick Reference

### Trigger Release
```bash
git commit --allow-empty -m "Release vX.Y.Z" && git push origin main
```

### Check Release Status
```bash
curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/releases/latest"
```

### View Workflow Runs
```bash
curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "https://api.github.com/repos/sich97/packet-loss-monitor/actions/runs?per_page=5"
```
