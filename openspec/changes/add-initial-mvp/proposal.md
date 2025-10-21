# Proposal: Initial MVP Implementation

## Why
ez-term currently has no implementation. This proposal establishes the foundational MVP capabilities: connecting to AI backends (local Ollama or cloud APIs) for inference, detecting system information, identifying installed tools and usage patterns, and providing a command-line interface for user interaction with backend selection.

## What Changes
- Add multi-backend LLM client supporting both local Ollama and cloud APIs (Groq, OpenAI)
- Add backend selection and configuration management
- Add system information detection (OS, architecture, shell, memory)
- Add tool detection by scanning PATH and common install locations
- Add usage pattern analysis from shell history files
- Add CLI interface with model and backend selection
- Add configuration management for user preferences (backend, model, API keys)

## Impact
- **Affected specs**: `llm-client` (renamed from ollama-client), `system-detection`, `tool-detection`, `cli-interface` (all new)
- **Affected code**: Initial Rust project setup with `main.rs`, `Cargo.toml`, and module structure
- **Dependencies**: clap, tokio, reqwest, serde/serde_json, sysinfo
