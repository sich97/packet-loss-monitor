use assert_cmd::Command;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_valid_arguments() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--interface").arg("lo").arg("--count").arg("1").arg("--interval").arg("1");
    cmd.assert().success();
}

#[test]
fn test_default_arguments() {
    let mut cmd = Command::cargo_bin("packet_loss_monitor").unwrap();
    cmd.arg("--interface").arg("lo");
    cmd.assert().success();
}
