# CLI Interface Capability

## ADDED Requirements

### Requirement: Setup Command
The system SHALL provide an init command to launch the setup wizard.

#### Scenario: Run ez init command
- **WHEN** the user runs `ez init`
- **THEN** the system SHALL launch the interactive setup wizard

#### Scenario: Re-run setup
- **WHEN** the user runs `ez init` and a configuration file already exists
- **THEN** the system SHALL allow the user to reconfigure settings, overwriting existing configuration
