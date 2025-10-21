# Developer Guide

Welcome to ez development! This guide will help you get started contributing to the project.

## Table of Contents

- [Quick Start](#quick-start)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Common Tasks](#common-tasks)
- [Debugging](#debugging)
- [Release Process](#release-process)

## Quick Start

### 5-Minute Setup

```bash
# 1. Clone the repository
git clone git@github.com:ezcorp-org/ez-term.git
cd ez-term

# 2. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Build the project
cargo build

# 4. Run tests
cargo test

# 5. Install locally for testing
./dev-install.sh

# 6. Try it out!
ez "list files in current directory"
```

You're ready to contribute! 🎉

## Development Setup

### Prerequisites

- **Rust** 1.70+ ([install](https://rustup.rs))
- **Git** for version control
- **A text editor** (VS Code, Vim, etc.)

### Recommended Tools

```bash
# Rust tooling
cargo install cargo-watch    # Auto-rebuild on changes
cargo install cargo-edit      # Manage dependencies
cargo install cargo-tarpaulin # Code coverage

# For testing
cargo install cargo-nextest   # Faster test runner
```

### VS Code Setup

Recommended extensions:
- `rust-analyzer` - Rust language server
- `crates` - Manage dependencies
- `Better TOML` - TOML file support

Settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.check.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

## Project Structure

```
ez-term/
├── src/                      # Source code
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI parsing
│   ├── config.rs            # Configuration
│   ├── system_info.rs       # System detection
│   ├── tool_detection.rs    # Tool scanning
│   ├── context_agent.rs     # Agentic context
│   ├── llm_client.rs        # LLM backends
│   └── update.rs            # Self-update
│
├── tests/                    # Test files
│   ├── integration/         # Integration tests
│   ├── e2e_*.rs            # End-to-end tests
│   └── test_*.rs           # Unit test helpers
│
├── docs/                     # Documentation
│   ├── ARCHITECTURE.md      # System design
│   ├── DEVELOPER_GUIDE.md   # This file
│   ├── TESTING.md           # Testing guide
│   └── *.md                 # Feature docs
│
├── scripts/                  # Shell scripts
│   ├── ez.zsh              # Zsh wrapper
│   ├── ez.sh               # Bash wrapper
│   └── install.sh          # Production installer
│
├── .github/                  # GitHub configuration
│   ├── workflows/          # CI/CD
│   └── ISSUE_TEMPLATE/     # Issue templates
│
├── Cargo.toml               # Rust dependencies
├── CHANGELOG.md             # Version history
├── CONTRIBUTING.md          # Contribution guide
├── CODE_OF_CONDUCT.md       # Community guidelines
└── README.md                # Project overview
```

### Key Files

| File | Purpose | Modify When |
|------|---------|-------------|
| `src/main.rs` | Entry point, orchestration | Adding new commands or workflow |
| `src/cli.rs` | CLI arguments | Adding new flags or arguments |
| `src/context_agent.rs` | Context detection | Adding new tool detection |
| `src/llm_client.rs` | LLM backends | Adding new LLM providers |
| `Cargo.toml` | Dependencies | Adding new crates |
| `CHANGELOG.md` | Version history | Every feature/fix |

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

Edit code, add tests, update docs.

### 3. Test Locally

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Watch mode (auto-run tests on save)
cargo watch -x test
```

### 4. Check Code Quality

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Check formatting without changing
cargo fmt -- --check
```

### 5. Build and Test Install

```bash
# Build release
cargo build --release

# Test local install
./dev-install.sh

# Try it out
ez "your test query"
```

### 6. Commit Changes

```bash
git add .
git commit -m "feat: add your feature"

# Good commit message format:
# feat: new feature
# fix: bug fix
# docs: documentation
# test: tests
# refactor: code refactoring
# chore: maintenance
```

### 7. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Code Style

### Rust Conventions

Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

```rust
// Good: Clear, descriptive names
fn detect_system_information() -> SystemInfo {
    // ...
}

// Bad: Abbreviated, unclear
fn det_sys_inf() -> SysInf {
    // ...
}
```

### Error Handling

Use `anyhow::Result` for public APIs:

```rust
use anyhow::Result;

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    let contents = fs::read_to_string(&path)
        .context("Failed to read config file")?;
    // ...
}
```

### Comments

Write doc comments for public items:

```rust
/// Detects the current operating system and architecture.
///
/// # Returns
///
/// A `SystemInfo` struct containing:
/// - OS name (linux, macos, windows)
/// - Architecture (x86_64, aarch64, etc.)
/// - Shell type (bash, zsh, fish)
///
/// # Examples
///
/// ```
/// let info = SystemInfo::detect();
/// println!("OS: {}", info.os);
/// ```
pub fn detect() -> SystemInfo {
    // ...
}
```

### Module Organization

```rust
// Imports
use anyhow::Result;
use std::path::PathBuf;

// Type definitions
pub struct MyStruct {
    field: String,
}

// Implementation
impl MyStruct {
    pub fn new() -> Self {
        // ...
    }
}

// Tests at bottom
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ...
    }
}
```

## Testing

### Writing Tests

#### Unit Tests

Add to the same file:

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

#### Integration Tests

Create in `tests/integration/`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_version() {
    Command::cargo_bin("ez")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ez"));
}
```

#### E2E Tests

Create in `tests/`:

```rust
#[test]
fn test_e2e_command_generation() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.path("/api/generate");
        then.json_body(json!({
            "response": "{\"command\": \"ls -la\", \"description\": \"List files\"}"
        }));
    });

    // Test full workflow
    Command::cargo_bin("ez")
        .unwrap()
        .env("OLLAMA_HOST", server.base_url())
        .arg("list files")
        .assert()
        .success();

    mock.assert();
}
```

### Test Organization

- **Unit tests**: In `src/` files with `#[cfg(test)]`
- **Integration tests**: In `tests/integration/`
- **E2E tests**: In `tests/e2e_*.rs`
- **Test utilities**: Shared helpers in `tests/`

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test file
cargo test --test e2e_command_generation

# With output
cargo test -- --nocapture

# Single test
cargo test test_config_default

# Watch mode
cargo watch -x test
```

## Common Tasks

### Adding a New CLI Flag

1. Edit `src/cli.rs`:
```rust
#[derive(Parser, Debug)]
pub struct Cli {
    /// Your new flag description
    #[arg(long)]
    pub new_flag: bool,
}
```

2. Handle in `src/main.rs`:
```rust
if args.new_flag {
    // Handle the flag
}
```

3. Add test in `tests/integration/cli_tests.rs`:
```rust
#[test]
fn test_new_flag() {
    Command::cargo_bin("ez")
        .unwrap()
        .arg("--new-flag")
        .assert()
        .success();
}
```

4. Update `README.md` and `--help` text

### Adding a New Backend

1. Edit `src/llm_client.rs`:
```rust
pub enum Backend {
    Ollama,
    Groq,
    OpenAI,
    NewBackend,  // Add here
}

impl Backend {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "ollama" => Ok(Self::Ollama),
            "groq" => Ok(Self::Groq),
            "openai" => Ok(Self::OpenAI),
            "newbackend" => Ok(Self::NewBackend),  // Add here
            _ => Err(anyhow!("Unknown backend")),
        }
    }

    pub fn default_model(&self) -> &str {
        match self {
            // ...
            Self::NewBackend => "default-model",
        }
    }
}
```

2. Implement backend method:
```rust
async fn generate_and_collect_newbackend(
    &self,
    prompt: &str,
    system_context: &str
) -> Result<String> {
    // API call implementation
}
```

3. Add to `generate_and_collect()`:
```rust
pub async fn generate_and_collect(
    &self,
    query: &str,
    system_context: &str
) -> Result<String> {
    match self.backend {
        Backend::NewBackend => {
            self.generate_and_collect_newbackend(query, system_context).await
        }
        // ...
    }
}
```

4. Add tests in `tests/integration/llm_client_tests.rs`

5. Update documentation in `README.md`

### Adding a New Context Detector

1. Edit `src/context_agent.rs`:
```rust
fn get_tool_context(&self) -> Option<String> {
    let mut context = String::new();

    // Check if tool is installed
    if let Ok(output) = Command::new("tool")
        .args(&["--version"])
        .output()
    {
        if output.status.success() {
            // Parse and add to context
        }
    }

    Some(context)
}
```

2. Add keyword triggers:
```rust
pub fn get_relevant_context(&self, query: &str) -> String {
    let query_lower = query.to_lowercase();
    let needs_tool = self.mentions_tool(&query_lower, &["tool", "keyword"]);

    if needs_tool {
        if let Some(ctx) = self.get_tool_context() {
            context.push_str("\n## Tool Context\n");
            context.push_str(&ctx);
        }
    }
}
```

3. Add tests in `tests/test_context_agent.rs`

4. Document in `docs/AGENTIC_CONTEXT.md`

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update all dependencies
cargo update

# Update specific dependency
cargo update serde

# Add new dependency
cargo add <crate-name>

# Add dev dependency
cargo add --dev <crate-name>
```

## Debugging

### Print Debugging

```rust
// Simple print
println!("Debug: {:?}", variable);

// Pretty print
dbg!(&variable);

// Conditional debug print
#[cfg(debug_assertions)]
println!("Only in debug builds");
```

### Using rust-lldb/gdb

```bash
# Debug build
cargo build

# Run with debugger
rust-lldb target/debug/ez

# Set breakpoint
(lldb) breakpoint set --name main
(lldb) run
```

### Logging

```rust
// Add to Cargo.toml
// env_logger = "0.10"
// log = "0.4"

// In code
use log::{info, debug, error};

info!("Information message");
debug!("Debug message");
error!("Error message");

// Run with logging
RUST_LOG=debug cargo run
```

### Common Issues

**Issue**: `cargo build` fails with dependency errors

**Solution**:
```bash
cargo clean
cargo update
cargo build
```

**Issue**: Tests fail with "permission denied"

**Solution**: Tests create temp files. Make sure you have write permissions.

**Issue**: Shell wrapper not working

**Solution**:
```bash
# Reload shell configuration
source ~/.zshrc  # or ~/.bashrc

# Check if function exists
type ez
```

## Release Process

### Version Bump

1. Update `Cargo.toml`:
```toml
[package]
version = "0.4.0"
```

2. Update `CHANGELOG.md`:
```markdown
## [0.4.0] - 2024-10-20

### Added
- New feature description

### Fixed
- Bug fix description
```

3. Commit:
```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.4.0"
```

### Create Release

1. Tag the release:
```bash
git tag v0.4.0
git push origin v0.4.0
```

2. GitHub Actions will automatically:
   - Build binaries for all platforms
   - Create GitHub release
   - Attach binaries

3. Announce release in README and discussions

## Best Practices

### DO

✅ Write tests for new features
✅ Update documentation
✅ Follow Rust conventions
✅ Run `cargo fmt` and `cargo clippy`
✅ Write clear commit messages
✅ Ask questions in issues/discussions

### DON'T

❌ Commit directly to `main`
❌ Skip tests
❌ Leave TODO comments without issues
❌ Break existing functionality
❌ Ignore clippy warnings

## Getting Help

### Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Clap Documentation](https://docs.rs/clap/)

### Community

- GitHub Issues: Bug reports and feature requests
- GitHub Discussions: Questions and ideas
- Pull Requests: Code contributions

### Questions?

Don't hesitate to:
- Open an issue
- Start a discussion
- Ask in your PR

We're here to help! 🎉

## Next Steps

Now that you're set up:

1. Check [good first issues](https://github.com/ezcorp-org/ez-term/labels/good%20first%20issue)
2. Read [CONTRIBUTING.md](../CONTRIBUTING.md)
3. Join discussions on new features
4. Start coding!

Happy hacking! 🚀
