use assert_cmd::Command;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_list_interfaces() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--list-interfaces");
    cmd.assert().success();
}

#[test]
fn test_missing_interface_error() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--test");
    cmd.assert().failure();
}
