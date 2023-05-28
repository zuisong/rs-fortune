extern crate assert_cmd;
extern crate predicates;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_ok() {
    assert!(true)
}

#[test]
fn calling_with_not_exists_file() {
    let mut cmd = Command::cargo_bin("rs-fortune").unwrap();

    cmd.args(&["wwww.shouldnotwork.com"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"The forunte file 'wwww.shouldnotwork.com' does not exists"#,
        ));
}

#[test]
fn calling_with_help() {
    let mut cmd = Command::cargo_bin("rs-fortune").unwrap();
    cmd.args(&["-h"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"
If the fortune cookie file path is omitted, the contents of environment
variable FORTUNE_FILE will be used. If neither is available, fortune will abort.
"#
            .trim(),
        ));
}

#[test]
fn calling_with_read_file_from_commandline() {
    Command::cargo_bin("rs-fortune")
        .unwrap()
        .args(&["Cargo.toml"])
        .assert()
        .stdout(predicates::str::contains(
            r#"
[package]
edition = "2021"
name = "rs-fortune"
"#
            .trim(),
        ));
}

#[test]
fn calling_with_read_file_from_env() {
    Command::cargo_bin("rs-fortune")
        .unwrap()
        .args(&["Cargo.toml"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"
[package]
edition = "2021"
name = "rs-fortune"
"#
            .trim(),
        ));
}
