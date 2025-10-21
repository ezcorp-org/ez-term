# Proposal: Add Interactive Setup Wizard

## Why
Currently, users must manually configure ez-term by editing the config file or using individual CLI flags (`--set-backend`, `--set-model`). This creates friction for first-time users who may not know:
- What backends are available
- Which models are installed (especially for Ollama)
- What configuration options exist
- How to get started quickly

An interactive setup wizard triggered on first run (or via `ez init`) will guide users through initial configuration, making the onboarding experience smooth and discoverable.

## What Changes
- Add an `ez init` command that launches an interactive setup wizard
- Detect first-time runs (no config file exists) and automatically trigger the setup wizard
- Allow users to re-run setup at any time via `ez init`
- For Ollama backend, dynamically query installed models using the `/api/tags` endpoint
- Provide an interactive menu for:
  - Backend selection (ollama, groq, openai)
  - Model selection (with dynamic Ollama model list or static lists for cloud providers)
  - API key configuration for cloud backends
  - Ollama URL configuration (with smart defaults)
- Save all selections to the config file
- Support skipping setup to use defaults

## Impact
- **Affected specs**: `cli-interface` (adds `ez init` command and first-run detection), new `setup-wizard` spec
- **Affected code**:
  - `src/cli.rs` - Add `init` flag
  - `src/main.rs` - Add first-run detection and setup wizard invocation logic
  - New module `src/setup.rs` - Interactive setup wizard implementation
  - `src/llm_client.rs` - Already has `list_models()` method, may need minor updates
  - `src/config.rs` - Add helper methods for interactive configuration building
- **Dependencies**: May need `dialoguer` or `inquire` crate for interactive prompts
