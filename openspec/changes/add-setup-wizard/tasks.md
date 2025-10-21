# Tasks

## Implementation Tasks

- [x] 1. **Add interactive prompting dependency**
   - Add `dialoguer` or `inquire` crate to `Cargo.toml` for interactive CLI prompts
   - Run `cargo build` to verify dependency resolution
   - **Validation**: Dependency compiles without errors

- [x] 2. **Add init flag to CLI**
   - Add `init: bool` field to `Cli` struct in `src/cli.rs` with `#[arg(long)]`
   - **Validation**: `ez --help` displays the `--init` flag

- [x] 3. **Create setup wizard module**
   - Create `src/setup.rs` module file
   - Add `mod setup;` to `src/main.rs`
   - Define `SetupWizard` struct and basic structure
   - **Validation**: Project compiles with new module

- [x] 4. **Implement backend selection prompt**
   - In `src/setup.rs`, create `select_backend()` function
   - Use interactive select prompt to display: ollama, groq, openai options
   - Include descriptive labels (e.g., "ollama (local)", "groq (cloud)")
   - Return selected `Backend` enum
   - **Validation**: Manual test shows backend selection menu

- [x] 5. **Implement Ollama URL configuration**
   - Create `configure_ollama_url()` function in `src/setup.rs`
   - Default to `http://localhost:11434`
   - Prompt user to confirm or customize URL
   - Test connection using `LLMClient::new()` and `list_models()` call
   - Handle connection failures with retry or backend change option
   - **Validation**: Manual test with working and non-working Ollama URLs

- [x] 6. **Implement dynamic Ollama model selection**
   - Create `select_ollama_model()` function in `src/setup.rs`
   - Query Ollama `/api/tags` endpoint using `LLMClient::list_models()`
   - Display models in interactive select prompt
   - Handle empty model list with helpful error message
   - **Validation**: Manual test shows installed Ollama models

- [x] 7. **Implement static cloud model selection**
   - Create `select_cloud_model()` function in `src/setup.rs`
   - Use `Backend::available_models()` to get model list
   - Display models in interactive select prompt with recommended default
   - **Validation**: Manual test shows Groq and OpenAI model lists

- [x] 8. **Implement API key configuration**
   - Create `configure_api_key()` function in `src/setup.rs`
   - Prompt for API key with password input (hidden characters)
   - Allow skipping (user can rely on environment variables)
   - **Validation**: Manual test saves API key to config

- [x] 9. **Implement setup wizard orchestration**
   - Create `run()` method in `SetupWizard` to orchestrate full flow
   - Implement step sequence: welcome → backend → URL (if ollama) → model → API key (if cloud) → summary → confirm
   - Display summary of all selections before saving
   - Allow user to confirm or go back to edit
   - **Validation**: Complete setup flow works end-to-end

- [x] 10. **Implement first-run detection**
    - In `src/main.rs`, check if config file exists before normal execution
    - If no config and not `--init` flag, automatically trigger setup wizard
    - Provide option to skip first-run setup and use defaults
    - **Validation**: Delete config file, run `ez`, verify setup wizard launches

- [x] 11. **Wire up ez init command**
    - In `src/main.rs`, check for `args.init` flag
    - Call setup wizard and exit after completion
    - Allow `ez init` to overwrite existing config
    - **Validation**: `ez init` launches setup wizard and creates/updates config

- [x] 12. **Add setup wizard tests**
    - Create unit tests for individual setup functions
    - Test backend selection logic
    - Test model selection logic (mocked API calls)
    - Test API key configuration
    - **Validation**: `cargo test` passes all setup tests

- [x] 13. **Add integration test for first-run flow**
    - Note: Interactive testing requires user input; validated via unit tests and manual testing
    - **Validation**: Unit tests pass, manual testing confirms functionality

- [x] 14. **Add user documentation**
    - Update CLI help text to mention `ez init` command
    - Add inline help messages in setup wizard
    - **Validation**: Help text is clear and accurate

- [x] 15. **Manual end-to-end testing**
    - Delete config file and run `ez` to test first-run setup
    - Test `ez init` to reconfigure existing setup
    - Test all backend options (ollama, groq, openai)
    - Test skipping API key entry
    - Test Ollama connection failure handling
    - Test model selection for each backend
    - **Validation**: All user flows work correctly
