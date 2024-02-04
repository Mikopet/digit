use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_no_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("digit")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("no argument given"));

    Ok(())
}

#[test]
fn test_wrong_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("digit")?;

    cmd.arg("NaN");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn test_good_arg_fail() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("digit")?;

    cmd.arg("1");
    cmd.assert().failure().code(predicate::eq(1));

    Ok(())
}

#[test]
fn test_good_arg_success() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("digit")?;

    cmd.arg("zero");
    cmd.assert().success().code(predicate::eq(0));

    Ok(())
}
