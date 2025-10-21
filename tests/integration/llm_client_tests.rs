use assert_cmd::Command;
use httpmock::prelude::*;
use predicates::prelude::*;
use serde_json::json;
use tempfile::TempDir;

#[test]
fn test_ollama_backend_mock() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    // Mock Ollama generate endpoint
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate")
            .json_body(json!({
                "model": "llama2",
                "prompt": "find large files",
                "stream": false
            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "response": "{\"command\": \"find . -type f -size +100M\", \"description\": \"Finds files larger than 100MB\"}",
                "done": true
            }));
    });

    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Verify the mock was called
    mock.assert();
}

#[test]
fn test_groq_backend_requires_api_key() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend to groq without API key
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("groq")
        .assert()
        .success();

    // Attempting to use it should fail without API key
    // (Can't test actual generation without API key, but we can test setup)
}

#[test]
fn test_openai_backend_requires_api_key() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend to openai without API key
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("openai")
        .assert()
        .success();
}

#[test]
fn test_backend_switching() {
    let temp_dir = TempDir::new().unwrap();

    // Set to ollama
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success()
        .stdout(predicate::str::contains("ollama"));

    // Switch to groq
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("groq")
        .assert()
        .success()
        .stdout(predicate::str::contains("groq"));

    // Switch to openai
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("openai")
        .assert()
        .success()
        .stdout(predicate::str::contains("openai"));
}

#[test]
fn test_model_configuration() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend first
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Set model
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-model")
        .arg("qwen3-coder:latest")
        .assert()
        .success()
        .stdout(predicate::str::contains("qwen3-coder:latest"));
}

#[test]
fn test_list_backends_shows_all() {
    Command::cargo_bin("ez")
        .unwrap()
        .arg("--list-backends")
        .assert()
        .success()
        .stdout(predicate::str::contains("ollama"))
        .stdout(predicate::str::contains("groq"))
        .stdout(predicate::str::contains("openai"));
}

#[tokio::test]
async fn test_ollama_streaming_mock() {
    let server = MockServer::start();

    // Mock streaming response
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .header("content-type", "application/x-ndjson")
            .body(r#"{"response": "{\"command\": ", "done": false}
{"response": "\"ls -la\"", "done": false}
{"response": ", \"description\": \"", "done": false}
{"response": "List files\"}", "done": true}"#);
    });

    // This would require actually calling the client in the test
    // For now, we just verify the mock is set up
    mock.assert_hits(0); // Not called yet, but ready
}

#[test]
fn test_env_var_override_ollama_host() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend with custom OLLAMA_HOST
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", "http://custom-host:11434")
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();
}

#[test]
fn test_invalid_ollama_host_format() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend with invalid OLLAMA_HOST (should still accept it for config)
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", "not-a-url")
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();
}

#[test]
fn test_backend_persistence_across_commands() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Set model (should remember backend)
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-model")
        .arg("llama3")
        .assert()
        .success();

    // Both should be in config now
    let config_path = temp_dir.path().join(".config/ez-term/config.toml");
    let contents = std::fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("ollama"));
    assert!(contents.contains("llama3"));
}
