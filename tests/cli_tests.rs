use assert_cmd::{Command, assert::Assert, cargo::cargo_bin_cmd};
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
    cargo_bin_cmd!()
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
    cargo_bin_cmd!()
        .args(["-h"])
        .assert()
        .success()
        .stdout(contains(r#"The fortune cookie file path"#.trim()))
        .dbg();
}

#[test]
fn calling_with_read_dir_from_commandline() {
    cargo_bin_cmd!()
        .args(["src"])
        .assert()
        .failure()
        .stderr(contains("'src' is a directory, not a file"))
        .dbg();
}

#[test]
fn calling_with_read_file_from_commandline() {
    cargo_bin_cmd!()
        .args(["Cargo.toml"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_read_file_from_env() {
    cargo_bin_cmd!()
        .args(["Cargo.toml"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_read_fortune_from_pipe() {
    cargo_bin_cmd!()
        .pipe_stdin("./Cargo.toml")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not())
        .dbg();
}

#[test]
fn calling_with_print_shell_completions() {
    cargo_bin_cmd!()
        .args(["completions", "-s", "zsh"])
        .env("FORTUNE_FILE", "Cargo.toml")
        .assert()
        .success()
        .stdout(predicates::str::starts_with("#compdef rs-fortune"))
        .dbg();
}

use std::io::Write;
#[test]
fn calling_with_empty_file() {
    let file = tempfile::NamedTempFile::new().unwrap();
    cargo_bin_cmd!()
        .args([file.path()])
        .assert()
        .success()
        .stdout("")
        .dbg();
}

#[test]
fn calling_with_actual_content() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(file, "fortune1\n%\nfortune2\n%\n").unwrap();

    cargo_bin_cmd!()
        .args([file.path()])
        .assert()
        .success()
        .stdout(predicates::str::is_match("fortune1|fortune2").unwrap())
        .dbg();
}

#[test]
fn calling_with_actual_content2() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(file, "fortune1\n%\nfortune2\n").unwrap();

    cargo_bin_cmd!()
        .args([file.path()])
        .assert()
        .success()
        .stdout(predicates::str::is_match("fortune1|fortune2").unwrap())
        .dbg();
}

#[test]
fn calling_with_invalid_encoding() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(b"\xFF").unwrap();
    cargo_bin_cmd!()
        .args([file.path()])
        .assert()
        .failure()
        .stderr(contains(
            r#"Error: Error { kind: InvalidData, message: "stream did not contain valid UTF-8"#,
        ))
        .dbg();
}

#[test]
fn calling_with_non_existent_shell() {
    cargo_bin_cmd!()
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
