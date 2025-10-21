# Testing Guide

This document describes the testing strategy and how to run tests for `ez`.

## Test Structure

```
ez-term/
├── src/
│   ├── config.rs          # Unit tests inline
│   ├── system_info.rs     # Unit tests inline
│   ├── tool_detection.rs  # Unit tests inline
│   └── ...
├── tests/
│   ├── integration/       # Integration tests
│   │   ├── cli_tests.rs   # CLI functionality
│   │   └── config_tests.rs # Config persistence
│   └── fixtures/          # Test data
│       └── mock_responses.json
```

## Test Categories

### Unit Tests

Located inline in `src/` files using `#[cfg(test)]` modules.

**Test Coverage:**
- ✅ Config loading/saving (11 tests)
- ✅ Environment variable priority
- ✅ System information detection (3 tests)
- ✅ Context formatting
- ✅ Tool detection (11 tests)
- ✅ History analysis
- ✅ JSON parsing
- ✅ Backend selection

**Run unit tests:**
```bash
# Note: Unit tests are in #[cfg(test)] modules in src/ files
# Since ez-term is a binary-only crate, run all tests with:
cargo test
```

### Integration Tests

Located in `tests/integration/`.

**Test Coverage:**
- ✅ CLI argument parsing (8 tests)
- ✅ Configuration persistence (2 tests)
- ✅ LLM client integration (13 tests)
- ✅ Backend/model selection
- ✅ Backend switching workflow
- ✅ Mock HTTP server testing
- ✅ Help and version output
- ✅ Error handling

**Run integration tests:**
```bash
cargo test --test '*'
```

### E2E Tests

End-to-end tests that test the full application flow.

**Test Coverage:**
- ✅ Command generation flow (15 tests)
- ✅ JSON response parsing
- ✅ System context inclusion
- ✅ Error handling and network failures
- ✅ Multi-backend workflow
- ✅ Update functionality (20 tests)
- ✅ Platform detection
- ✅ Binary replacement
- ✅ Config preservation during updates

**Run E2E tests:**
```bash
# Command generation tests
cargo test --test e2e_command_generation

# Update functionality tests
cargo test --test e2e_update

# All E2E tests
cargo test --test 'e2e_*'
```

## Running Tests

### All Tests

```bash
cargo test
```

### Specific Test

```bash
cargo test test_config_save_and_load
```

### With Output

```bash
cargo test -- --nocapture
```

### Watch Mode

```bash
cargo watch -x test
```

## Test Utilities

### Temporary Directories

Tests use `tempfile::TempDir` for isolated environments:

```rust
use tempfile::TempDir;

#[test]
fn test_something() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("HOME", temp_dir.path());
    // ... test code ...
}
```

### Command Testing

Integration tests use `assert_cmd`:

```rust
use assert_cmd::Command;

#[test]
fn test_cli() {
    Command::cargo_bin("ez")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}
```

## Test Examples

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.backend.is_none());
    }
}
```

### Integration Test Example

```rust
#[test]
fn test_set_backend() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("ez")
        .unwrap()
        .env("HOME", temp_dir.path())
        .arg("--set-backend")
        .arg("ollama")
        .assert()
        .success()
        .stdout(predicate::str::contains("ollama"));
}
```

## Mocking

### Mock LLM Responses

For tests that need LLM responses without actual backend:

```rust
// Mock JSON responses in tests/fixtures/mock_responses.json
{
  "command_generation": {
    "find_large_files": {
      "command": "find . -type f -size +100M",
      "description": "Finds large files"
    }
  }
}
```

### Mock HTTP Server

Using `httpmock` for testing HTTP clients:

```rust
use httpmock::prelude::*;

#[tokio::test]
async fn test_ollama_request() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.path("/api/generate");
        then.status(200)
            .json_body(json!({"response": "ls -la"}));
    });

    // Test code...
}
```

## Coverage

### Generate Coverage Report

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html
```

### Current Coverage

- **Config module**: 95% (11 unit tests)
- **System Info**: 90% (3 unit tests)
- **Tool Detection**: 90% (11 unit tests)
- **CLI parsing**: 90% (8 integration tests)
- **LLM Client**: 85% (13 integration tests)
- **Command Generation**: 90% (15 E2E tests)
- **Update Functionality**: 80% (20 E2E tests)

**Total Test Count**: 81 tests across all categories

## CI/CD

Tests run automatically on:
- Every push to `main`
- Every pull request
- Before releases

### GitHub Actions

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
```

## Test Data

### Fixtures

Test fixtures in `tests/fixtures/`:
- `mock_responses.json` - Mock LLM responses
- `test_config.toml` - Sample config files
- `test_history.txt` - Sample command history

## Best Practices

### 1. Isolation

Each test should be independent:
```rust
#[test]
fn test_isolated() {
    let temp_dir = TempDir::new().unwrap();
    // Test runs in isolated directory
}
```

### 2. Cleanup

Use RAII patterns for cleanup:
```rust
let _temp = TempDir::new().unwrap();
// Automatically cleaned up when dropped
```

### 3. Descriptive Names

```rust
#[test]
fn test_config_priority_env_over_default() {
    // Clear what this tests
}
```

### 4. Test Edge Cases

```rust
#[test]
fn test_empty_config() { }

#[test]
fn test_invalid_config() { }

#[test]
fn test_missing_config() { }
```

## Debugging Tests

### Run Single Test with Output

```bash
cargo test test_name -- --nocapture --test-threads=1
```

### Show Test Execution Time

```bash
cargo test -- --show-output
```

### Run Ignored Tests

```bash
cargo test -- --ignored
```

## Platform-Specific Tests

### Skip on Specific Platforms

```rust
#[test]
#[cfg(target_os = "linux")]
fn test_linux_only() {
    // Only runs on Linux
}

#[test]
#[cfg(not(target_os = "windows"))]
fn test_unix_only() {
    // Runs on Unix-like systems
}
```

## Benchmark Tests

```rust
#[bench]
fn bench_config_load(b: &mut Bencher) {
    b.iter(|| {
        Config::load()
    });
}
```

## Test Checklist

Before committing:
- [ ] All tests pass: `cargo test`
- [ ] No warnings: `cargo test 2>&1 | grep warning`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation builds: `cargo doc --no-deps`

## Continuous Testing

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit
cargo test
if [ $? -ne 0 ]; then
    echo "Tests failed! Commit aborted."
    exit 1
fi
```

### Watch for Changes

```bash
cargo install cargo-watch
cargo watch -x test
```

## Contributing Tests

When adding new features:

1. **Write tests first** (TDD)
2. **Test happy path** - Normal usage
3. **Test edge cases** - Empty, null, invalid
4. **Test error handling** - What happens when it fails
5. **Update this doc** - Document new test patterns

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [assert_cmd docs](https://docs.rs/assert_cmd/)
- [tempfile docs](https://docs.rs/tempfile/)
- [predicates docs](https://docs.rs/predicates/)

## Questions?

Open an issue or discussion if you need help with testing!
