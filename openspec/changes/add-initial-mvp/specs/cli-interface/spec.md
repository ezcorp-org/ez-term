# CLI Interface Capability

## ADDED Requirements

### Requirement: Command-Line Argument Parsing
The system SHALL accept and parse command-line arguments for user queries and options.

#### Scenario: Accept query as arguments
- **WHEN** the user runs `ez-term "how do I list files"`
- **THEN** the system SHALL parse the query and process it

#### Scenario: Accept query from stdin
- **WHEN** the user pipes input or runs without arguments
- **THEN** the system SHALL read the query from stdin

#### Scenario: Model selection flag
- **WHEN** the user specifies `--model <name>` or `-m <name>`
- **THEN** the system SHALL use the specified model for this invocation

#### Scenario: Backend selection flag
- **WHEN** the user specifies `--backend <name>` or `-b <name>`
- **THEN** the system SHALL use the specified backend (ollama, groq, openai) for this invocation

### Requirement: Configuration Management
The system SHALL persist user preferences in a configuration file.

#### Scenario: Default configuration location
- **WHEN** no config file exists
- **THEN** the system SHALL create a default configuration at `~/.config/ez-term/config.toml`

#### Scenario: Store preferred model
- **WHEN** the user specifies a preferred model
- **THEN** the system SHALL save it in the configuration file

#### Scenario: Store backend preference
- **WHEN** the user configures a preferred backend
- **THEN** the system SHALL save it in the configuration file

#### Scenario: Store API keys
- **WHEN** the user configures API keys for cloud backends
- **THEN** the system SHALL save them securely in the configuration file

#### Scenario: Store Ollama URL
- **WHEN** the user configures a custom Ollama URL
- **THEN** the system SHALL save it in the configuration file

#### Scenario: Load configuration on startup
- **WHEN** the application starts
- **THEN** it SHALL load preferences from the configuration file if it exists

### Requirement: Interactive Backend and Model Selection
The system SHALL allow users to list and select backends and models interactively.

#### Scenario: List available backends
- **WHEN** the user runs `ez-term --list-backends`
- **THEN** the system SHALL display all supported backends (ollama, groq, openai)

#### Scenario: List available models
- **WHEN** the user runs `ez-term --list-models`
- **THEN** the system SHALL display all available models for the current backend

#### Scenario: Set default backend
- **WHEN** the user runs `ez-term --set-backend <name>`
- **THEN** the system SHALL save the specified backend as the default in configuration

#### Scenario: Set default model
- **WHEN** the user runs `ez-term --set-model <name>`
- **THEN** the system SHALL save the specified model as the default in configuration

### Requirement: Output Formatting
The system SHALL display LLM responses in a readable terminal format.

#### Scenario: Stream output
- **WHEN** receiving streaming responses from Ollama
- **THEN** the system SHALL display tokens as they arrive for responsive feedback

#### Scenario: Error display
- **WHEN** an error occurs
- **THEN** the system SHALL display a clear, actionable error message

#### Scenario: Plain text output
- **WHEN** outputting responses
- **THEN** the system SHALL use plain text suitable for terminal display and piping

### Requirement: Help and Documentation
The system SHALL provide help information to users.

#### Scenario: Display help
- **WHEN** the user runs `ez-term --help`
- **THEN** the system SHALL display usage information, available flags, and examples

#### Scenario: Display version
- **WHEN** the user runs `ez-term --version`
- **THEN** the system SHALL display the current version number
