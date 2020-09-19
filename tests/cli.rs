use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn subcommand_pzf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("numbers")?;

    cmd.arg("pfz")
        .arg("40")
        .assert()
        .success()
        .stdout(predicate::str::contains("pfz(40) = [2, 2, 2, 5]"));

    Ok(())
}

#[test]
fn subcommand_ggt() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("numbers")?;

    cmd.arg("ggt")
        .arg("8")
        .arg("12")
        .assert()
        .success()
        .stdout(predicate::str::contains("ggT(8, 12) = 4"));

    Ok(())
}
