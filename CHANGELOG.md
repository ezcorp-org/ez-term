# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2024-10-19

### Added
- **Agentic Context System**: Intelligent context fetching based on query analysis
- Auto-detection of Git repository information (branch, user, remotes, status)
- Auto-detection of Docker environment (version, running containers, compose files)
- Auto-detection of NPM/Node.js projects (version, package.json, scripts)
- Auto-detection of Python environments (version, venv, requirements, pyproject.toml)
- Auto-detection of Shell configurations (aliases, functions, exports from RC files)
- Auto-detection of Rust/Cargo projects (version, Cargo.toml, package info)
- Auto-detection of Kubernetes context (kubectl, current context, namespace)
- Smart keyword pattern matching to trigger relevant context fetching
- RC file parsing for shell aliases and custom configurations
- CLI tool version detection for better compatibility suggestions
- Context agent integration tests
- `docs/AGENTIC_CONTEXT.md` - Comprehensive agentic system documentation

### Changed
- Enhanced `process_query` to use agentic context agent
- Context now adapts based on what user is asking about
- LLM receives targeted, relevant context instead of generic system info

### Improved
- Commands now aware of user's actual git setup
- Commands leverage user's custom shell aliases
- Commands match user's installed tool versions
- Commands work with user's project structure

## [0.2.0] - 2024-10-19

### Added
- **OS-Specific Context Injection**: System info now includes OS-specific command guidance
- Enhanced LLM prompt with explicit OS instructions (Linux/macOS/Windows)
- OS detection tests (4 unit tests + 2 E2E tests)
- Comprehensive test suite: 85 tests total (26 unit + 23 integration + 36 E2E)
- Unit tests for config module (11 tests)
- Unit tests for system_info module (4 tests)
- Unit tests for tool_detection module (11 tests)
- Integration tests for LLM client (13 tests)
- E2E tests for command generation (17 tests)
- E2E tests for update functionality (20 tests)
- Development install scripts (`dev-install.sh`, `quick-install.sh`, `setup-rust.sh`)
- Comprehensive documentation:
  - `docs/TESTING.md` - Testing guide
  - `docs/TEST_SUMMARY.md` - Test suite summary
  - `docs/OS_DETECTION.md` - OS detection documentation
  - `docs/INSTALLATION.md` - Installation guide
  - `INSTALL.md` - Quick reference
- Test infrastructure with `test-install.sh`

### Changed
- System info format now includes "Operating System:" prefix
- Enhanced prompt with OS-specific command examples
- Improved shell wrapper installation with permission checks
- Better error messages for missing backends

### Fixed
- Format string error in main.rs OS parameter
- Removed unused imports (Subcommand, json!)
- Shell RC file permission handling in install scripts

### Documentation
- Project reorganization following open-source best practices
- CONTRIBUTING.md with contribution guidelines
- CODE_OF_CONDUCT.md
- GitHub issue templates (bug report, feature request)
- Comprehensive documentation in `docs/` directory

## [0.1.0] - 2024-10-17

### Added
- Initial release of ez CLI command generator
- Multi-backend LLM support (Ollama, Groq, OpenAI)
- JSON output format with command + description
- Shell integration for zsh and bash (`print -z` command injection)
- Safety-first command generation with refusal system
- System and tool detection for context-aware commands
- Interactive mode with command history
- Self-update functionality (`ez --update`)
- Cross-platform prebuilt binaries (Linux, macOS, Windows)
- Universal installer script (no Rust required)
- GitHub Actions workflow for automated releases

### Features
- **Command Generation**: Natural language to shell commands
- **Safety**: Preview mode by default, refuses unsafe operations
- **Shell Integration**: Commands appear in terminal ready to execute
- **Multi-Backend**: Support for local (Ollama) and cloud (Groq, OpenAI) LLMs
- **System-Aware**: Detects OS, shell, and available tools
- **Self-Updating**: One-command updates (`ez --update`)
- **Configuration**: Persistent settings for backend and model
- **Interactive Mode**: REPL-style interface with history

### Documentation
- README.md with installation and usage
- docs/INSTALL.md - Installation guide
- docs/QUICKSTART.md - Quick start tutorial
- docs/UPDATE.md - Update guide
- docs/SHELL_INTEGRATION.md - Shell integration details
- docs/COMMAND_GENERATOR.md - Command generation guide
- docs/OLLAMA_SETUP.md - Ollama configuration
- docs/DISTRIBUTION.md - Distribution guide for maintainers

### Installation
- One-line installer: `curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash`
- Cargo install: `cargo install ez-cli`
- Prebuilt binaries available for all major platforms

### Supported Platforms
- Linux x86_64
- Linux ARM64 (aarch64)
- macOS x86_64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x86_64

[Unreleased]: https://github.com/ezcorp-org/ez-term/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ezcorp-org/ez-term/releases/tag/v0.1.0
