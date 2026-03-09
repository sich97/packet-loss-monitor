use assert_cmd::prelude::*;

#[test]
fn test_help_output() {
    let mut cmd = assert_cmd::Command::cargo_bin("packet_loss_monitor")
        .unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_valid_arguments() {
    let mut cmd = assert_cmd::Command::cargo_bin("packet_loss_monitor")
        .unwrap();
    cmd.arg("eth0").arg("2").arg("10");
    cmd.assert().success();
}

#[test]
fn test_default_arguments() {
    let mut cmd = assert_cmd::Command::cargo_bin("packet_loss_monitor")
        .unwrap();
    cmd.arg("eth0");
    cmd.assert().success();
}
