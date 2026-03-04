use assert_cmd::Command;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("dev-dashboard").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("dev-dashboard").unwrap();
    cmd.arg("--help").assert().success();
}
