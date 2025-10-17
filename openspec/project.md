# Project Context

## Purpose
ez-term is a lightweight CLI tool that provides an intelligent terminal assistant powered by local Ollama models. The tool aims to:
- Provide context-aware AI assistance directly in the terminal
- Detect and utilize system information to give relevant suggestions
- Identify user's preferred tooling based on installation and usage patterns
- Allow easy model selection and configuration
- Keep everything local and fast

## Tech Stack
- **Rust** - Primary language for performance and safety
- **Ollama** - Local LLM inference engine
- **clap** - Command-line argument parsing
- **serde/serde_json** - Serialization for configuration and API communication
- **tokio** - Async runtime for API calls
- **reqwest** - HTTP client for Ollama API
- **sysinfo** - System information gathering

## Project Conventions

### Code Style
- Follow standard Rust conventions (rustfmt)
- Use idiomatic Rust patterns (Result/Option types, ? operator)
- Prefer explicit error handling over panics
- Keep functions small and focused
- Use descriptive variable names
- Document public APIs with doc comments

### Architecture Patterns
- **Modular design**: Separate concerns into modules (system info, ollama client, config, CLI)
- **Configuration management**: Store user preferences (model selection, prompt templates) in a config file
- **System detection**: Non-invasive system scanning to gather context
- **Tool detection**: Scan PATH and common locations for installed tools
- **Usage pattern detection**: Parse shell history to identify frequently used commands/tools

### Testing Strategy
- Unit tests for core logic (system detection, tool identification)
- Integration tests for Ollama API communication
- Mock external dependencies where appropriate
- Test on multiple platforms (Linux, macOS, Windows)

### Git Workflow
- Feature branches for new capabilities
- Descriptive commit messages
- Keep commits atomic and focused
- Tag releases with semantic versioning

## Domain Context
- **Terminal context awareness**: The tool needs to understand terminal environments, shell types, and command-line workflows
- **Tool detection heuristics**: Identify tools by checking PATH, common install locations (/usr/local/bin, ~/.cargo/bin, etc.)
- **Usage pattern analysis**: Parse .bash_history, .zsh_history to identify frequently used commands
- **System capabilities**: Detect OS, architecture, available memory, shell environment

## Important Constraints
- **Local-only**: All LLM inference must run locally via Ollama (no cloud APIs)
- **Lightweight**: Keep binary size small and startup time fast
- **Privacy-first**: No telemetry or data collection
- **Cross-platform**: Should work on Linux, macOS, and Windows where possible

## External Dependencies
- **Ollama**: Must be installed and running locally (typically on localhost:11434)
- **Shell history files**: Optional, for usage pattern detection
- **System commands**: May invoke `which`, `command -v`, or equivalent for tool detection
