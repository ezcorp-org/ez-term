# Setup Wizard Capability

## ADDED Requirements

### Requirement: Interactive Setup Command
The system SHALL provide an `ez init` command that launches an interactive setup wizard.

#### Scenario: User runs ez init
- **WHEN** the user runs `ez init`
- **THEN** the system SHALL launch an interactive setup wizard to configure initial settings

#### Scenario: Setup wizard saves configuration
- **WHEN** the user completes the setup wizard
- **THEN** the system SHALL save all selections to `~/.config/ez-term/config.toml`

#### Scenario: User can skip setup
- **WHEN** the user cancels or skips the setup wizard
- **THEN** the system SHALL exit gracefully without modifying the configuration

### Requirement: First-Run Detection
The system SHALL automatically trigger the setup wizard on first run when no configuration exists.

#### Scenario: No config file exists
- **WHEN** the user runs `ez` for the first time and no config file exists at `~/.config/ez-term/config.toml`
- **THEN** the system SHALL automatically launch the setup wizard before proceeding

#### Scenario: Config file exists
- **WHEN** the user runs `ez` and a config file exists
- **THEN** the system SHALL skip automatic setup and use existing configuration

#### Scenario: Skip first-run setup
- **WHEN** the user is prompted with first-run setup and chooses to skip
- **THEN** the system SHALL create a default config file and proceed with defaults

### Requirement: Backend Selection
The system SHALL allow interactive backend selection during setup.

#### Scenario: Display available backends
- **WHEN** the setup wizard reaches the backend selection step
- **THEN** the system SHALL display a menu with options: ollama (local), groq (cloud), openai (cloud)

#### Scenario: Select backend
- **WHEN** the user selects a backend from the menu
- **THEN** the system SHALL save the selection to the configuration

#### Scenario: Backend with default selection
- **WHEN** the setup wizard displays backend options
- **THEN** the system SHALL highlight a recommended default (e.g., ollama if available, otherwise groq)

### Requirement: Model Selection
The system SHALL allow interactive model selection during setup.

#### Scenario: Display Ollama models dynamically
- **WHEN** the user selects the ollama backend
- **THEN** the system SHALL query the Ollama API at `/api/tags` to retrieve installed models

#### Scenario: Display Ollama models from running instance
- **WHEN** querying Ollama for available models
- **THEN** the system SHALL display the list of installed models for the user to select

#### Scenario: Handle Ollama connection failure
- **WHEN** the Ollama API is unreachable or returns an error
- **THEN** the system SHALL display an error message and allow the user to configure the Ollama URL or select a different backend

#### Scenario: Display cloud provider models statically
- **WHEN** the user selects groq or openai backend
- **THEN** the system SHALL display a predefined list of available models for that provider

#### Scenario: Select model
- **WHEN** the user selects a model from the menu
- **THEN** the system SHALL save the selection to the configuration

#### Scenario: Model with default selection
- **WHEN** the setup wizard displays model options
- **THEN** the system SHALL highlight a recommended default model for the selected backend

### Requirement: API Key Configuration
The system SHALL allow API key configuration for cloud backends during setup.

#### Scenario: Prompt for Groq API key
- **WHEN** the user selects the groq backend
- **THEN** the system SHALL prompt the user to enter their Groq API key

#### Scenario: Prompt for OpenAI API key
- **WHEN** the user selects the openai backend
- **THEN** the system SHALL prompt the user to enter their OpenAI API key

#### Scenario: Skip API key entry
- **WHEN** prompted for an API key
- **THEN** the user SHALL be able to skip entry, and the system will rely on environment variables (GROQ_API_KEY, OPENAI_API_KEY)

#### Scenario: Save API key to config
- **WHEN** the user provides an API key
- **THEN** the system SHALL save it to the configuration file

#### Scenario: No API key prompt for Ollama
- **WHEN** the user selects the ollama backend
- **THEN** the system SHALL NOT prompt for an API key

### Requirement: Ollama URL Configuration
The system SHALL allow Ollama URL configuration during setup.

#### Scenario: Default Ollama URL
- **WHEN** the user selects the ollama backend
- **THEN** the system SHALL use `http://localhost:11434` as the default Ollama URL

#### Scenario: Custom Ollama URL
- **WHEN** the user selects the ollama backend
- **THEN** the system SHALL allow the user to specify a custom Ollama URL if the default is incorrect

#### Scenario: Test Ollama connection
- **WHEN** the user provides an Ollama URL
- **THEN** the system SHALL test the connection by querying the `/api/tags` endpoint before proceeding

#### Scenario: Retry on connection failure
- **WHEN** the Ollama connection test fails
- **THEN** the system SHALL allow the user to re-enter the URL or select a different backend

### Requirement: Setup Wizard Flow
The system SHALL guide users through setup in a logical sequence.

#### Scenario: Setup wizard step order
- **WHEN** the setup wizard launches
- **THEN** the system SHALL present configuration options in the following order:
  1. Welcome message and overview
  2. Backend selection
  3. Ollama URL configuration (if ollama selected)
  4. Model selection (dynamic for ollama, static for cloud)
  5. API key configuration (if cloud backend selected)
  6. Summary of selections
  7. Confirmation and save

#### Scenario: Display setup summary
- **WHEN** the user completes all setup steps
- **THEN** the system SHALL display a summary of all selections before saving

#### Scenario: Confirm and save
- **WHEN** the user confirms the setup summary
- **THEN** the system SHALL save the configuration and display a success message

#### Scenario: Edit selections
- **WHEN** viewing the setup summary
- **THEN** the user SHALL be able to go back and edit any selection before confirming
