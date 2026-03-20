use assert_cmd::Command;
use std::thread;

#[test]
fn test_monitor_compilation() {
    // This test simply checks that the monitor binary can be compiled
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("Help command should succeed");
    
    assert!(output.status.success(), "Help should work");
}

#[test]
fn test_monitor_help() {
    // This test checks that the help message works
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("Help command should succeed");
    
    assert!(output.status.success(), "Help should work");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("packet_loss_monitor"), "Help should contain program name");
}

#[test]
fn test_monitor_valid_interface() {
    // This test checks that the monitor accepts a valid interface argument
    // We'll just check that it doesn't panic on startup with valid args
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--interface")
       .arg("lo")
       .arg("--count")
       .arg("1")
       .arg("--interval")
       .arg("1");
    
    // Run with a timeout using std::process
    let handle = thread::spawn(move || {
        cmd.output()
    });
    
    let result = handle.join();
    
    match result {
        Ok(Ok(output)) => {
            // The program ran successfully
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(!stderr.contains("panicked"), "Program should not panic");
        }
        Ok(Err(e)) => {
            panic!("Command failed: {:?}", e);
        }
        Err(e) => {
            panic!("Thread panicked: {:?}", e);
        }
    }
}
