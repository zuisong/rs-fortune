extern crate assert_cmd;
extern crate predicates;

use assert_cmd::{crate_name, prelude::*};
use std::process::Command;

#[test]
fn test_ok() {
    assert!(true)
}

#[test]
fn calling_with_not_exists_file() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["wwww.shouldnotwork.com"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            r#"The forunte file 'wwww.shouldnotwork.com' does not exists"#,
        ));
}

#[test]
fn calling_with_help() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["-h"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"The fortune cookie file path"#.trim(),
        ));
}

#[test]
fn calling_with_read_file_from_commandline() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["Cargo.toml"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"
name = "rs-fortune"
"#
            .trim(),
        ));
}

#[test]
fn calling_with_read_dir_from_commandline() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["src"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            r#"The forunte file 'src' is a directory"#.trim(),
        ));
}

#[test]
fn calling_with_read_file_from_env() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["Cargo.toml"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"
name = "rs-fortune"
"#
            .trim(),
        ));
}

#[test]
fn calling_with_print_shell_completions() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["--completion", "zsh"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicates::str::contains("rs-fortune"));
}
