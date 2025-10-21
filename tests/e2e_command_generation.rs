use assert_cmd::Command;
use httpmock::prelude::*;
use predicates::prelude::*;
use serde_json::json;
use tempfile::TempDir;

/// E2E test for complete command generation flow
#[test]
fn test_e2e_command_generation_with_mock_ollama() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    // Mock Ollama generate endpoint
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "response": "{\"command\": \"find . -type f -size +100M -exec ls -lh {} \\\\;\", \"description\": \"Finds files larger than 100MB in current directory and shows their size in human-readable format.\"}",
                "done": true
            }));
    });

    // Set up backend
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Run command generation
    let output = Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("find large files")
        .assert()
        .success();

    // Verify mock was called
    mock.assert();

    // Verify output contains command
    output.stdout(predicate::str::contains("find . -type f -size +100M"));
}

#[test]
fn test_e2e_list_files_command() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .json_body(json!({
                "response": "{\"command\": \"ls -la\", \"description\": \"Lists all files in the current directory with detailed information.\"}",
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

    let output = Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("list all files")
        .assert()
        .success();

    mock.assert();
    output.stdout(predicate::str::contains("ls -la"));
}

#[test]
fn test_e2e_disk_usage_command() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .json_body(json!({
                "response": "{\"command\": \"du -h --max-depth=1 | sort -hr\", \"description\": \"Shows disk usage in human-readable format sorted by size.\"}",
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

    let output = Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("show disk usage")
        .assert()
        .success();

    mock.assert();
    output.stdout(predicate::str::contains("du -h"));
}

#[test]
fn test_e2e_json_parsing_with_description() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .json_body(json!({
                "response": "{\"command\": \"tar -czf archive.tar.gz .\", \"description\": \"Creates a gzip-compressed tar archive of the current directory.\"}",
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

    let output = Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("compress directory")
        .assert()
        .success();

    mock.assert();

    // Should contain both description and command
    output.stdout(predicate::str::contains("💡"))
        .stdout(predicate::str::contains("tar -czf archive.tar.gz"));
}

#[test]
fn test_e2e_fallback_parsing_non_json_response() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    // Mock returning non-JSON response (fallback scenario)
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .json_body(json!({
                "response": "ls -la",
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

    let output = Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("list files")
        .assert()
        .success();

    mock.assert();
    output.stdout(predicate::str::contains("ls -la"));
}

#[test]
fn test_e2e_system_context_included() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate")
            .matches(|req| {
                // Verify system context is included in prompt
                let body: serde_json::Value = serde_json::from_slice(&req.body.as_ref().unwrap()).unwrap();
                let prompt = body["prompt"].as_str().unwrap_or("");
                prompt.contains("Operating System:") &&
                (prompt.contains("LINUX") || prompt.contains("MACOS") || prompt.contains("WINDOWS"))
            });
        then.status(200)
            .json_body(json!({
                "response": "{\"command\": \"ls\", \"description\": \"List files\"}",
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

    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("list files")
        .assert()
        .success();

    mock.assert();
}

#[test]
fn test_e2e_os_specific_guidance_in_prompt() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate")
            .matches(|req| {
                // Verify OS-specific guidance is in prompt
                let body: serde_json::Value = serde_json::from_slice(&req.body.as_ref().unwrap()).unwrap();
                let prompt = body["prompt"].as_str().unwrap_or("");
                prompt.contains("OS Commands:") &&
                prompt.contains("MATCH the user's OS")
            });
        then.status(200)
            .json_body(json!({
                "response": "{\"command\": \"ps aux\", \"description\": \"List processes\"}",
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

    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("show processes")
        .assert()
        .success();

    mock.assert();
}

#[test]
fn test_e2e_error_handling_network_failure() {
    let temp_dir = TempDir::new().unwrap();

    // Set backend to a non-existent server
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", "http://localhost:99999")
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Attempt command generation should fail gracefully
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", "http://localhost:99999")
        .arg("list files")
        .assert()
        .failure();
}

#[test]
fn test_e2e_empty_query() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Empty query should show help or error
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_e2e_multi_backend_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    // Start with Ollama
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success();

    // Switch to Groq
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("groq")
        .assert()
        .success();

    // Switch to OpenAI
    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("openai")
        .assert()
        .success();

    // Verify final state is OpenAI
    let config_path = temp_dir.path().join(".config/ez-term/config.toml");
    let contents = std::fs::read_to_string(&config_path).unwrap();
    assert!(contents.contains("openai"));
}

#[test]
fn test_e2e_command_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/generate");
        then.status(200)
            .json_body(json!({
                "response": "{\"command\": \"grep -r 'TODO:' . | wc -l\", \"description\": \"Counts TODO comments in codebase\"}",
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

    let output = Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .env("OLLAMA_HOST", server.base_url())
        .arg("count todos in my code")
        .assert()
        .success();

    mock.assert();
    output.stdout(predicate::str::contains("grep"));
}
