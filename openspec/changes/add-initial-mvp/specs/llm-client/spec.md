# LLM Client Capability

## ADDED Requirements

### Requirement: Backend Selection
The system SHALL support multiple LLM backends (local Ollama and cloud APIs).

#### Scenario: Select backend via configuration
- **WHEN** the user configures a backend in the config file
- **THEN** the system SHALL use that backend for all requests

#### Scenario: Select backend via CLI flag
- **WHEN** the user specifies `--backend <name>` or `-b <name>`
- **THEN** the CLI argument SHALL override the configured backend

#### Scenario: Default backend selection
- **WHEN** no backend is configured
- **THEN** the system SHALL attempt Ollama first, then fall back to available cloud backends with API keys

### Requirement: Ollama Backend Support
The system SHALL establish HTTP connections to a local Ollama instance for LLM inference.

#### Scenario: Connect to default Ollama endpoint
- **WHEN** using Ollama backend without custom configuration
- **THEN** it SHALL attempt to connect to `http://localhost:11434`

#### Scenario: Connect to custom Ollama endpoint
- **WHEN** the user specifies a custom Ollama URL in configuration
- **THEN** the client SHALL use the configured URL

#### Scenario: Ollama connection failure
- **WHEN** Ollama is not running or unreachable
- **THEN** the system SHALL return a clear error message or attempt fallback if configured

### Requirement: Groq Backend Support
The system SHALL support Groq cloud API for LLM inference.

#### Scenario: Authenticate with Groq API key
- **WHEN** using Groq backend
- **THEN** the system SHALL use the API key from configuration or environment variable `GROQ_API_KEY`

#### Scenario: Groq API request
- **WHEN** the user provides a query with Groq backend
- **THEN** the system SHALL send requests to Groq's `/chat/completions` endpoint

#### Scenario: Groq authentication failure
- **WHEN** no API key is configured for Groq
- **THEN** the system SHALL return an error indicating missing API key

### Requirement: OpenAI Backend Support
The system SHALL support OpenAI API for LLM inference.

#### Scenario: Authenticate with OpenAI API key
- **WHEN** using OpenAI backend
- **THEN** the system SHALL use the API key from configuration or environment variable `OPENAI_API_KEY`

#### Scenario: OpenAI API request
- **WHEN** the user provides a query with OpenAI backend
- **THEN** the system SHALL send requests to OpenAI's `/chat/completions` endpoint

#### Scenario: OpenAI authentication failure
- **WHEN** no API key is configured for OpenAI
- **THEN** the system SHALL return an error indicating missing API key

### Requirement: Model Listing
The system SHALL retrieve available models from the selected backend.

#### Scenario: List Ollama models
- **WHEN** using Ollama backend and user requests available models
- **THEN** the system SHALL query Ollama's `/api/tags` endpoint

#### Scenario: List Groq models
- **WHEN** using Groq backend and user requests available models
- **THEN** the system SHALL return supported Groq models (llama3-70b-8192, mixtral-8x7b-32768, gemma-7b-it)

#### Scenario: List OpenAI models
- **WHEN** using OpenAI backend and user requests available models
- **THEN** the system SHALL return common OpenAI models (gpt-4, gpt-3.5-turbo)

#### Scenario: No models available
- **WHEN** no models are available for the selected backend
- **THEN** the system SHALL inform the user

### Requirement: Model Selection
The system SHALL allow users to select which model to use for inference.

#### Scenario: Use default model per backend
- **WHEN** no model is specified
- **THEN** the system SHALL use backend-specific defaults (llama2 for Ollama, llama3-70b-8192 for Groq, gpt-3.5-turbo for OpenAI)

#### Scenario: Use configured model
- **WHEN** a model is specified in configuration
- **THEN** the system SHALL use that model for inference

#### Scenario: Override model via CLI
- **WHEN** a model is specified via command-line argument
- **THEN** the CLI argument SHALL take precedence

### Requirement: Prompt Generation
The system SHALL send prompts with relevant context to the selected backend.

#### Scenario: Generate prompt with system context
- **WHEN** the user provides a query
- **THEN** the system SHALL include system information and detected tooling as context

#### Scenario: Ollama prompt format
- **WHEN** using Ollama backend
- **THEN** the system SHALL format requests for Ollama's `/api/generate` endpoint

#### Scenario: Cloud API prompt format
- **WHEN** using Groq or OpenAI backend
- **THEN** the system SHALL format requests as chat completion messages with system and user roles

#### Scenario: Stream response
- **WHEN** the backend supports streaming
- **THEN** the system SHALL handle the stream and display output progressively
