# Implementation Tasks

## 1. Project Setup
- [x] 1.1 Initialize Rust project with `cargo init`
- [x] 1.2 Add dependencies to `Cargo.toml` (clap, tokio, reqwest, serde, serde_json, sysinfo, toml)
- [x] 1.3 Set up module structure (`main.rs`, `lib.rs`, module files)

## 2. Configuration Management
- [x] 2.1 Define configuration struct with backend, model, API keys, Ollama URL fields
- [x] 2.2 Implement config file loading from `~/.config/ez-term/config.toml`
- [x] 2.3 Implement config file creation with defaults
- [x] 2.4 Implement config save functionality
- [x] 2.5 Add environment variable support for API keys (GROQ_API_KEY, OPENAI_API_KEY)

## 3. System Detection
- [x] 3.1 Implement OS and architecture detection using `std::env::consts`
- [x] 3.2 Implement shell detection from environment variables
- [x] 3.3 Implement memory detection using `sysinfo` crate
- [x] 3.4 Implement system context formatting function

## 4. Tool Detection
- [x] 4.1 Implement PATH scanning to enumerate executables
- [x] 4.2 Implement common location scanning (platform-specific directories)
- [x] 4.3 Implement bash history parser
- [x] 4.4 Implement zsh history parser
- [x] 4.5 Implement frequency analysis for commands
- [x] 4.6 Implement tool context formatting function

## 5. LLM Client - Backend Abstraction
- [x] 5.1 Define backend trait/enum for Ollama, Groq, OpenAI
- [x] 5.2 Define common request/response types
- [x] 5.3 Implement backend selection logic with fallback

## 6. LLM Client - Ollama Backend
- [x] 6.1 Define Ollama API types (request/response structs)
- [x] 6.2 Implement connection check to Ollama
- [x] 6.3 Implement model listing (`/api/tags` endpoint)
- [x] 6.4 Implement prompt generation for Ollama format
- [x] 6.5 Implement streaming response handler (`/api/generate` endpoint)

## 7. LLM Client - Groq Backend
- [x] 7.1 Define Groq API types (chat completions format)
- [x] 7.2 Implement authentication with API key
- [x] 7.3 Implement model listing (hardcoded supported models)
- [x] 7.4 Implement chat completions request
- [x] 7.5 Implement streaming response handler

## 8. LLM Client - OpenAI Backend
- [x] 8.1 Define OpenAI API types (chat completions format)
- [x] 8.2 Implement authentication with API key
- [x] 8.3 Implement model listing (hardcoded common models)
- [x] 8.4 Implement chat completions request
- [x] 8.5 Implement streaming response handler

## 9. CLI Interface
- [x] 9.1 Define CLI argument structure with clap
- [x] 9.2 Implement query input (args and stdin)
- [x] 9.3 Implement `--backend` / `-b` flag
- [x] 9.4 Implement `--model` / `-m` flag
- [x] 9.5 Implement `--list-backends` command
- [x] 9.6 Implement `--list-models` command
- [x] 9.7 Implement `--set-backend` command
- [x] 9.8 Implement `--set-model` command
- [x] 9.9 Implement help and version flags

## 10. Main Application Flow
- [x] 10.1 Wire up config loading in main
- [x] 10.2 Wire up system and tool detection
- [x] 10.3 Wire up backend selection and client initialization
- [x] 10.4 Implement main query processing flow
- [x] 10.5 Implement streaming output display
- [x] 10.6 Implement error handling and user-friendly messages

## 11. Testing & Validation
- [ ] 11.1 Test with Groq API (primary for testing)
- [ ] 11.2 Test with OpenAI API (if available)
- [ ] 11.3 Test with Ollama (if available)
- [x] 11.4 Test backend and model selection flags
- [ ] 11.5 Test configuration persistence
- [x] 11.6 Test on target platform (Linux)
- [ ] 11.7 Add unit tests for parsers and formatters
