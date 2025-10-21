use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ez"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("--update"));
}

#[test]
fn test_list_backends() {
    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.arg("--list-backends")
        .assert()
        .success()
        .stdout(predicate::str::contains("ollama"))
        .stdout(predicate::str::contains("groq"))
        .stdout(predicate::str::contains("openai"));
}

#[test]
fn test_set_backend() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success()
        .stdout(predicate::str::contains("Default backend set to: ollama"));
}

#[test]
fn test_set_model() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.env("HOME", temp_dir.path())
        .arg("--set-model")
        .arg("qwen3-coder:latest")
        .assert()
        .success()
        .stdout(predicate::str::contains("Default model set to: qwen3-coder:latest"));
}

#[test]
fn test_invalid_backend() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("invalid_backend")
        .assert()
        .success(); // Should still succeed in setting
}

#[test]
fn test_multiple_flags_error() {
    let mut cmd = Command::cargo_bin("ez").unwrap();
    cmd.arg("--help")
        .arg("--version")
        .assert()
        .success(); // Should process first flag
}
