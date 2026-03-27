use assert_cmd::Command;

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
fn test_list_interfaces_flag() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--list-interfaces");
    let output = cmd.output().expect("List interfaces should succeed");
    
    assert!(output.status.success(), "List interfaces should work");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Available network interfaces"), "Output should list interfaces");
}
