# Tool Detection Capability

## ADDED Requirements

### Requirement: PATH Tool Scanning
The system SHALL scan the user's PATH to identify installed command-line tools.

#### Scenario: Scan PATH directories
- **WHEN** detecting installed tools
- **THEN** the system SHALL enumerate executables in directories listed in the PATH environment variable

#### Scenario: Identify common tools
- **WHEN** scanning PATH
- **THEN** the system SHALL identify common development tools (git, docker, npm, cargo, python, etc.)

#### Scenario: PATH not available
- **WHEN** PATH environment variable is not set
- **THEN** the system SHALL skip PATH scanning without failing

### Requirement: Common Location Scanning
The system SHALL check common installation directories for tools not in PATH.

#### Scenario: Check standard locations
- **WHEN** detecting installed tools
- **THEN** the system SHALL check common directories like `/usr/local/bin`, `~/.cargo/bin`, `~/.local/bin`

#### Scenario: Platform-specific locations
- **WHEN** running on specific platforms
- **THEN** the system SHALL check platform-appropriate directories (e.g., `/opt/homebrew/bin` on macOS, `%USERPROFILE%\bin` on Windows)

### Requirement: Usage Pattern Analysis
The system SHALL analyze shell history to identify frequently used commands.

#### Scenario: Parse bash history
- **WHEN** the user has a `.bash_history` file
- **THEN** the system SHALL parse it to identify frequently used commands

#### Scenario: Parse zsh history
- **WHEN** the user has a `.zsh_history` file
- **THEN** the system SHALL parse it to identify frequently used commands

#### Scenario: History file not readable
- **WHEN** shell history files cannot be accessed
- **THEN** the system SHALL skip usage analysis without failing

#### Scenario: Extract base commands
- **WHEN** analyzing history
- **THEN** the system SHALL extract base commands (e.g., "git" from "git commit -m message")

### Requirement: Tool Context Formatting
The system SHALL format detected tools and usage patterns for LLM context.

#### Scenario: Generate tool context string
- **WHEN** preparing a prompt
- **THEN** the system SHALL format detected tools and frequently used commands as concise context

#### Scenario: Prioritize frequent tools
- **WHEN** many tools are detected
- **THEN** the system SHALL prioritize frequently used tools in the context to avoid overwhelming the prompt
