extern crate assert_cmd;
extern crate predicates;

use assert_cmd::{assert::Assert, crate_name, Command};
use predicates::prelude::{predicate, PredicateBooleanExt};

#[test]
fn _test_ok() {
    assert!(true)
}

#[test]
fn _test_build_ok() {
    Command::new("cargo").args(&["build"]).assert().success();
}

trait DBG {
    fn dbg(&self) -> &Self;
}

impl DBG for Assert {
    fn dbg(&self) -> &Self {
        dbg!(self);
        return self;
    }
}

#[test]
fn calling_with_not_exists_file() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["wwww.shouldnotwork.com"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "The fortune file 'wwww.shouldnotwork.com' does not exist",
        ))
        .dbg();
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
        ))
        .dbg();
}

#[test]
fn calling_with_read_dir_from_commandline() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["src"])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "'src' is a directory, not a file",
        ))
        .dbg();
}
#[test]
fn calling_with_read_file_from_commandline() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["Cargo.toml"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_read_file_from_env() {
    Command::cargo_bin(crate_name!())
        .unwrap()
        .args(&["Cargo.toml"])
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
        .args(&["--completion", "zsh"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicates::str::starts_with("#compdef rs-fortune"))
        .dbg();
}
