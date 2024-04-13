use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
//Running a basic test to call the program with no arguments.
fn run_with_defaults() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow"));
}

#[test]
//Running a test to pass in a file that does not exist to confirm that the program fails.
fn fail_on_no_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no_file_here.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Dude!!! Fix the file location!!"));
    Ok(())
}