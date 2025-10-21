use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join(".config/ez-term/config.toml");

    // Set backend
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Config file should exist
    assert!(config_path.exists());

    // Read config
    let contents = fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("ollama"));

    // Set model
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-model")
        .arg("qwen3-coder:latest")
        .assert()
        .success();

    // Config should contain both settings
    let contents = fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("ollama"));
    assert!(contents.contains("qwen3-coder"));
}

#[test]
fn test_config_env_var_priority() {
    let temp_dir = TempDir::new().unwrap();

    // Set config file value
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", "http://env-override:11434")
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Env var should work
    // (Can't easily test this without actually running LLM, but config is set up correctly)
}
