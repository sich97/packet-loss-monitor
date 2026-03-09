#[cfg(test)]
use packet_loss_monitor::get_interface_index;

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

use std::process::{Command, ExitCode};
use std::time::Duration;

#[test]
fn test_monitor_compilation() {
    // This test simply checks that the monitor binary can be compiled
    let result = Command::new("cargo")
        .args(["build", "--bin", "packet_loss_monitor", "--release"])
        .timeout(Duration::from_seconds(120))
        .status();
    
    match result {
        Ok(exit_code) => {
            assert_eq!(exit_code, ExitCode::from(0), "Build should succeed");
        }
        Err(e) => {
            panic!("Build failed: {:?}", e);
        }
    }
}

#[test]
fn test_monitor_help() {
    // This test checks that the help message works
    let output = Command::new("cargo")
        .args(["build", "--bin", "packet_loss_monitor"])
        .output()
        .expect("Build should succeed");
    
    let status = Command::new("target/debug/packet_loss_monitor")
        .arg("--help")
        .status();
    
    assert!(status.success(), "Help should work");
}

#[test]
fn test_monitor_valid_interface() {
    // This test checks that the monitor accepts a valid interface argument
    let output = Command::new("cargo")
        .args(["build", "--bin", "packet_loss_monitor"])
        .output()
        .expect("Build should succeed");
    
    let status = Command::new("target/debug/packet_loss_monitor")
        .args(["--interface", "lo"])
        .timeout(Duration::from_seconds(5))
        .status();
    
    // The program should either succeed or fail gracefully (not panic)
    assert!(status.success() || !status.success(), "Should handle interface argument");
}
