use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn bin_errors_out() {
    let mut cmd = Command::cargo_bin("bin").unwrap();
    let output = cmd.output().unwrap();

    assert!(!output.status.success());
}
