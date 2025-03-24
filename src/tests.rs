extern crate assert_cmd;
extern crate predicates;

use assert_cmd::{Command, assert::Assert, crate_name};
use predicates::{
    prelude::{PredicateBooleanExt as _, predicate},
    str::contains,
};

#[test]
fn _test_ok() {
    assert_eq!(2 + 2, 4);
}
#[test]
#[should_panic]
fn _test_error() {
    panic!("This is a test error")
}

#[test]
fn _test_build_ok() {
    Command::new("cargo").args(["build"]).assert().success();
}

trait Dbg {
    fn dbg(&self) -> &Self;
}

impl Dbg for Assert {
    fn dbg(&self) -> &Self {
        dbg!(self);
        self
    }
}

#[test]
fn calling_with_not_exists_file() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["wwww.shouldnotwork.com"])
        .assert()
        .failure()
        .stderr(contains(
            "The fortune file 'wwww.shouldnotwork.com' does not exist",
        ))
        .dbg();
}

#[test]
fn calling_with_help() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["-h"])
        .assert()
        .success()
        .stdout(contains(r#"The fortune cookie file path"#.trim()))
        .dbg();
}

#[test]
fn calling_with_read_dir_from_commandline() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["src"])
        .assert()
        .failure()
        .stderr(contains("'src' is a directory, not a file"))
        .dbg();
}
#[test]
fn calling_with_read_file_from_commandline() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["Cargo.toml"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_read_file_from_env() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["Cargo.toml"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_read_fortune_from_pipe() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .pipe_stdin("./Cargo.toml")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_print_shell_completions() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["completions", "-s", "zsh"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicates::str::starts_with("#compdef rs-fortune"))
        .dbg();
}

use std::{fs::File, io::Write};
#[test]
fn calling_with_empty_file() {
    let file = File::create("empty.txt").unwrap();
    drop(file);
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["empty.txt"])
        .assert()
        .success()
        .stdout("\n")
        .dbg();
    std::fs::remove_file("empty.txt").unwrap();
}

#[test]
fn calling_with_invalid_encoding() {
    let mut file = File::create("invalid.txt").unwrap();
    file.write_all(b"\xFF").unwrap();
    drop(file);
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["invalid.txt"])
        .assert()
        .failure()
        .stderr(contains(
            r#"Error: Error { kind: InvalidData, message: "stream did not contain valid UTF-8"#,
        ))
        .dbg();
    std::fs::remove_file("invalid.txt").unwrap();
}

#[test]
fn calling_with_non_existent_shell() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(["completions", "-s", "invalid"])
        .assert()
        .failure()
        .stderr(contains(
            "error: invalid value 'invalid' for '--shell <SHELL>'",
        ))
        .stderr(contains(
            "[possible values: bash, elvish, fish, powershell, zsh]",
        ));
}
