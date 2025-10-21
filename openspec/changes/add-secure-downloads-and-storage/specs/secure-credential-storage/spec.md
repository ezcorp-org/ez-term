# Secure Credential Storage Capability

## Overview
Store API keys and sensitive credentials using OS-native credential managers (keyring/keychain) instead of plaintext configuration files.

## ADDED Requirements

### Requirement: OS Keyring Integration
The application MUST support storing API keys in OS-native credential managers.

#### Scenario: Store API key in macOS Keychain
- **WHEN** a user runs setup wizard on macOS and enters a Groq or OpenAI API key
- **THEN** the application MUST store the key in macOS Keychain using service name "ez-term"
- **AND** use account name "groq_api_key" or "openai_api_key"
- **AND** NOT write the key to config.toml
- **AND** confirm successful storage to user

#### Scenario: Store API key in Linux Secret Service
- **WHEN** a user runs setup wizard on Linux with Secret Service available and enters an API key
- **THEN** the application MUST store the key using Secret Service API in the default keyring
- **AND** use label "ez-term groq_api_key" or "ez-term openai_api_key"
- **AND** NOT write the key to config.toml

#### Scenario: Store API key in Windows Credential Manager
- **WHEN** a user runs setup wizard on Windows and enters an API key
- **THEN** the application MUST store the key in Windows Credential Manager
- **AND** use target name "ez-term/groq_api_key" or "ez-term/openai_api_key"
- **AND** NOT write the key to config.toml

---

### Requirement: Retrieve Credentials from Keyring
The application MUST retrieve API keys from OS keyring during runtime.

#### Scenario: Successfully retrieve API key
- **WHEN** an API key is stored in the OS keyring and the application needs to make an API call
- **THEN** the application MUST query the OS keyring using the appropriate service/account name
- **AND** retrieve the API key
- **AND** use it for authentication
- **AND** NOT cache the key in memory longer than needed

#### Scenario: API key not found in keyring
- **WHEN** no API key is stored in the OS keyring and the application attempts to retrieve it
- **THEN** the application MUST check environment variables (GROQ_API_KEY, OPENAI_API_KEY)
- **AND** fall back to config file (deprecated, with warning)
- **AND** if none found, prompt user to run `ez init`
- **AND** display helpful error message

---

### Requirement: Automatic Migration from Plaintext
The application MUST automatically migrate plaintext API keys to secure storage.

#### Scenario: Detect plaintext keys on first run
- **WHEN** the user has API keys in config.toml from previous version and runs ez with the new version for the first time
- **THEN** the application MUST detect plaintext keys in config.toml
- **AND** display message "🔒 Detected API keys in config file. Migrate to secure storage?"
- **AND** prompt with [Y/n]
- **AND** if yes, migrate keys to keyring and remove from config
- **AND** if no, set migration_declined flag and show security warning
- **AND** remember decision to avoid repeated prompts

#### Scenario: Successful migration
- **WHEN** the user chooses to migrate plaintext keys
- **THEN** the application MUST read API keys from config.toml
- **AND** store each key in OS keyring
- **AND** verify successful storage
- **AND** remove keys from config.toml
- **AND** add comment to config "# API keys stored in system keyring"
- **AND** display "✅ API keys migrated to secure storage"

#### Scenario: Migration failure
- **WHEN** the user chooses to migrate plaintext keys but the OS keyring is unavailable or fails
- **THEN** the application MUST display error "❌ Failed to access system keyring"
- **AND** keep keys in config.toml (don't delete)
- **AND** suggest alternatives (environment variables or manual keyring setup)
- **AND** exit gracefully without data loss

---

### Requirement: Graceful Fallback for Missing Keyring
The application MUST handle scenarios where OS keyring is unavailable.

#### Scenario: Keyring unavailable on Linux
- **WHEN** a Linux system without Secret Service daemon running attempts to store an API key
- **THEN** the application MUST detect keyring unavailability
- **AND** display warning "⚠️  System keyring not available"
- **AND** offer alternatives: use environment variables (recommended) or store in config.toml with warning (deprecated)
- **AND** prompt user to choose
- **AND** document keyring requirements

#### Scenario: User chooses environment variable fallback
- **WHEN** keyring storage fails or is unavailable and the user selects environment variable option
- **THEN** the application MUST NOT store the key anywhere persistent
- **AND** display instructions for setting GROQ_API_KEY or OPENAI_API_KEY
- **AND** provide shell-specific export commands
- **AND** explain how to make variables persistent (.bashrc, .zshrc)

---

### Requirement: Config File Security Hardening
The application MUST enforce secure permissions on configuration files.

#### Scenario: Set restrictive permissions on config file
- **WHEN** the config file is created or modified and the application saves the config
- **THEN** the application MUST set file permissions to 0600 (owner read/write only) on Unix
- **AND** warn if unable to set restrictive permissions
- **AND** verify parent directory permissions are appropriate

#### Scenario: Warn about insecure config permissions
- **WHEN** an existing config file has overly permissive permissions (e.g., 0644)
- **THEN** the application MUST display warning "⚠️  Config file has insecure permissions"
- **AND** suggest "Run: chmod 600 ~/.config/ez-term/config.toml"
- **AND** continue operation (don't block)

---

### Requirement: Setup Wizard Integration
The setup wizard MUST use secure storage by default.

#### Scenario: Setup wizard stores keys securely
- **WHEN** a user runs `ez init` for the first time and configures a cloud backend (Groq or OpenAI)
- **THEN** the setup wizard MUST prompt for API key
- **AND** attempt to store in OS keyring
- **AND** if successful, confirm and proceed
- **AND** if failed, offer fallback options
- **AND** NEVER write to config.toml without user consent

#### Scenario: Setup wizard skips API key for Ollama
- **WHEN** a user runs `ez init` and selects Ollama backend
- **THEN** the application MUST NOT prompt for API keys
- **AND** NOT attempt keyring storage
- **AND** only store backend selection and Ollama URL

---

### Requirement: Environment Variable Priority
The application MUST respect environment variable overrides for API keys.

#### Scenario: Environment variable takes precedence
- **WHEN** an API key is stored in both keyring and environment variable
- **THEN** the application MUST use the environment variable value (highest priority)
- **AND** ignore the keyring value
- **AND** NOT display warnings about duplicate keys

#### Scenario: Credential retrieval priority order
- **WHEN** the application needs an API key
- **THEN** the application MUST check sources in this order: 1) Environment variable, 2) OS keyring, 3) Config file (deprecated), 4) None found (prompt user to configure)

---

## MODIFIED Requirements

### Requirement: Config Structure
The config.toml file structure SHALL be modified to exclude plaintext API keys.

#### Scenario: New config file format
- **WHEN** a user configures ez-term with secure storage and the config.toml is created
- **THEN** the file MUST contain backend selection (ollama/groq/openai)
- **AND** contain model selection
- **AND** contain ollama_url (if applicable)
- **AND** contain comment "# API keys stored securely in system keyring"
- **AND** NOT contain groq_api_key field
- **AND** NOT contain openai_api_key field
- **AND** NOT contain any plaintext secrets

---

## Dependencies
- `keyring` crate (cross-platform keyring abstraction)
  - macOS: Uses Security framework
  - Linux: Uses Secret Service API
  - Windows: Uses Credential Manager API
- Optional fallback to config file with warnings
- Existing: `toml`, `serde` for config management
