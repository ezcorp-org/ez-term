# Architecture

Complete architecture documentation for ez - the AI-powered CLI command generator.

## Table of Contents

- [Overview](#overview)
- [System Architecture](#system-architecture)
- [Module Structure](#module-structure)
- [Data Flow](#data-flow)
- [Key Components](#key-components)
- [Design Decisions](#design-decisions)
- [Extension Points](#extension-points)

## Overview

Ez is designed as a modular, extensible CLI tool that uses LLMs to generate context-aware shell commands.

### Core Principles

1. **Modularity** - Each component has a single responsibility
2. **Extensibility** - Easy to add new backends, detectors, or features
3. **Testability** - Comprehensive test coverage with clear boundaries
4. **Privacy** - All context gathering is local-only
5. **Safety** - Commands are generated with safety-first principles

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         User Input                           │
│                    (CLI Arguments/Query)                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      CLI Parser (clap)                       │
│                   Parse flags & arguments                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                    Configuration Layer                       │
│          • Load ~/.config/ez-term/config.toml               │
│          • Merge with environment variables                  │
│          • Determine backend & model                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                   Context Detection Layer                    │
│                                                              │
│  ┌────────────────┐  ┌──────────────┐  ┌────────────────┐ │
│  │ System Info    │  │ Tool         │  │ Context        │ │
│  │                │  │ Detection    │  │ Agent          │ │
│  │ • OS           │  │              │  │                │ │
│  │ • Architecture │  │ • PATH scan  │  │ • Git context  │ │
│  │ • Shell        │  │ • History    │  │ • Docker       │ │
│  │ • Memory       │  │ • Freq cmds  │  │ • NPM          │ │
│  │ • OS guidance  │  │              │  │ • Python       │ │
│  └────────────────┘  └──────────────┘  │ • Shell RC     │ │
│                                         │ • Cargo        │ │
│                                         │ • K8s          │ │
│                                         └────────────────┘ │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                     Prompt Construction                      │
│  • Base system prompt (safety rules, format)                │
│  • System context (OS, tools, installed software)           │
│  • Agentic context (query-specific: git, docker, etc.)      │
│  • User query                                                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      LLM Client Layer                        │
│                                                              │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐             │
│  │ Ollama   │    │  Groq    │    │ OpenAI   │             │
│  │ (Local)  │    │ (Cloud)  │    │ (Cloud)  │             │
│  └──────────┘    └──────────┘    └──────────┘             │
│                                                              │
│  • Generate and collect response                             │
│  • Streaming support (future)                               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                    Response Processing                       │
│  • Parse JSON response                                       │
│  • Extract command & description                             │
│  • Fallback to plain text if JSON fails                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      Output Layer                            │
│  • Display description (💡)                                  │
│  • Output command (for shell wrapper)                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                    Shell Integration                         │
│  • zsh: print -z (inject into line buffer)                  │
│  • bash: history -s + manual up-arrow                        │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules

```
src/
├── main.rs              # Entry point, orchestration
├── cli.rs               # CLI argument parsing
├── config.rs            # Configuration management
├── system_info.rs       # OS & system detection
├── tool_detection.rs    # Installed tool scanning
├── context_agent.rs     # Agentic context fetching
├── llm_client.rs        # Multi-backend LLM client
└── update.rs            # Self-update functionality
```

### Module Dependencies

```
main.rs
  ├─> cli.rs (argument parsing)
  ├─> config.rs (load configuration)
  ├─> system_info.rs (detect OS)
  ├─> tool_detection.rs (scan tools)
  ├─> context_agent.rs (fetch agentic context)
  ├─> llm_client.rs (call LLM)
  └─> update.rs (self-update)

llm_client.rs
  └─> (external) reqwest, serde_json

context_agent.rs
  ├─> (external) std::process::Command
  └─> (external) std::fs
```

## Data Flow

### 1. Command Generation Flow

```
User Query
    │
    ├─> Parse CLI args (cli.rs)
    │
    ├─> Load config (config.rs)
    │       ├─> Read ~/.config/ez-term/config.toml
    │       └─> Merge with env vars
    │
    ├─> Detect System (system_info.rs)
    │       ├─> OS, Architecture
    │       ├─> Shell type
    │       └─> Memory info
    │
    ├─> Detect Tools (tool_detection.rs)
    │       ├─> Scan PATH
    │       └─> Parse history files
    │
    ├─> Fetch Agentic Context (context_agent.rs)
    │       ├─> Analyze query keywords
    │       ├─> Fetch relevant context:
    │       │   ├─> Git (if query mentions git)
    │       │   ├─> Docker (if query mentions docker)
    │       │   ├─> NPM (if query mentions npm/node)
    │       │   ├─> Python (if query mentions python)
    │       │   ├─> Shell RC (if query mentions alias/function)
    │       │   ├─> Cargo (if query mentions rust/cargo)
    │       │   └─> K8s (if query mentions kubectl/k8s)
    │       └─> Combine contexts
    │
    ├─> Build Full Prompt
    │       ├─> System prompt (safety rules, format)
    │       ├─> System context
    │       ├─> Agentic context
    │       └─> User query
    │
    ├─> Call LLM (llm_client.rs)
    │       ├─> Ollama (local)
    │       ├─> Groq (cloud)
    │       └─> OpenAI (cloud)
    │
    ├─> Parse Response
    │       ├─> Try parse as JSON
    │       ├─> Extract command & description
    │       └─> Fallback to plain text
    │
    └─> Output
            ├─> Print description (💡)
            └─> Print command
```

### 2. Configuration Flow

```
Startup
    │
    ├─> Check ~/.config/ez-term/config.toml exists
    │       ├─> Yes: Load TOML
    │       └─> No: Use defaults
    │
    ├─> Check Environment Variables
    │       ├─> GROQ_API_KEY
    │       ├─> OPENAI_API_KEY
    │       └─> OLLAMA_HOST
    │
    └─> Merge (env vars override config file)
            ├─> Backend selection
            ├─> Model selection
            └─> API keys
```

## Key Components

### 1. CLI Parser (`cli.rs`)

**Purpose**: Parse command-line arguments using clap

**Key Fields**:
- `query: Option<String>` - User's natural language query
- `backend: Option<String>` - Override backend (ollama/groq/openai)
- `model: Option<String>` - Override model
- `list_backends: bool` - List available backends
- `set_backend: Option<String>` - Set default backend
- `update: bool` - Trigger self-update

### 2. Configuration (`config.rs`)

**Purpose**: Manage persistent configuration

**Storage**: `~/.config/ez-term/config.toml`

**Fields**:
```rust
pub struct Config {
    pub backend: Option<String>,
    pub model: Option<String>,
    pub ollama_url: Option<String>,
    pub groq_api_key: Option<String>,
    pub openai_api_key: Option<String>,
}
```

**Priority**:
1. Environment variables (highest)
2. Config file
3. Defaults (lowest)

### 3. System Info (`system_info.rs`)

**Purpose**: Detect OS and system characteristics

**Collected Data**:
- OS name (linux/macos/windows)
- Architecture (x86_64/aarch64)
- Shell (bash/zsh/fish)
- Memory (total/available)
- OS-specific command guidance

### 4. Tool Detection (`tool_detection.rs`)

**Purpose**: Detect installed tools and user preferences

**Scans**:
- PATH directories for common tools
- Shell history files (.bash_history, .zsh_history)
- Frequency analysis of commands

**Output**:
- List of installed tools
- Top 20 most frequent commands

### 5. Context Agent (`context_agent.rs`)

**Purpose**: Intelligently fetch relevant context based on query

**Detectors**:
- Git: branch, user, remotes, uncommitted files
- Docker: version, running containers, compose files
- NPM: node version, package.json, scripts
- Python: version, venv, requirements files
- Shell: aliases, functions, exports from RC files
- Cargo: rust version, Cargo.toml
- Kubernetes: context, namespace

**Trigger**: Keyword pattern matching in user query

### 6. LLM Client (`llm_client.rs`)

**Purpose**: Abstract interface to multiple LLM backends

**Supported Backends**:
- **Ollama**: Local models via HTTP API
- **Groq**: Cloud API (fast inference)
- **OpenAI**: Cloud API (GPT models)

**Methods**:
- `generate_and_collect()` - Send prompt, collect full response
- `list_models()` - Get available models for backend

**Response Format**:
```json
{
  "command": "ls -la",
  "description": "Lists all files with details"
}
```

### 7. Update System (`update.rs`)

**Purpose**: Self-update from GitHub releases

**Flow**:
1. Detect current version (from Cargo.toml)
2. Query GitHub API for latest release
3. Detect platform (linux/macos/windows)
4. Download appropriate binary
5. Replace running binary
6. Verify installation

## Design Decisions

### Why Rust?

- **Performance**: Fast startup, minimal overhead
- **Safety**: Memory safe, no runtime errors
- **Cross-platform**: Single binary for Linux/macOS/Windows
- **Ecosystem**: Excellent CLI libraries (clap, tokio, reqwest)

### Why Multiple Backends?

- **Flexibility**: Users choose based on needs
- **Offline**: Ollama works without internet
- **Quality**: OpenAI for best results
- **Speed**: Groq for fast responses
- **Cost**: Free tier options (Groq, Ollama)

### Why Agentic Context?

- **Relevance**: Only fetch context that matters
- **Performance**: Don't parse everything upfront
- **Accuracy**: LLM gets targeted, specific information
- **Privacy**: Only read what's needed

### Why Local Context Fetching?

- **Privacy**: No data sent except LLM prompt
- **Speed**: Local filesystem reads are fast
- **Offline**: Works without internet (with Ollama)
- **Control**: User owns all data

## Extension Points

### Adding a New Backend

1. Add enum variant to `Backend` in `llm_client.rs`:
```rust
pub enum Backend {
    Ollama,
    Groq,
    OpenAI,
    NewBackend,  // Add here
}
```

2. Implement backend methods:
```rust
async fn generate_and_collect_newbackend(
    &self,
    prompt: &str,
    system_context: &str
) -> Result<String> {
    // Implementation
}
```

3. Add to `list_models()` and `default_model()`

4. Update documentation in README.md

### Adding a New Context Detector

1. Add detection method in `context_agent.rs`:
```rust
fn get_tool_context(&self) -> Option<String> {
    // Check if tool is installed
    // Fetch relevant information
    // Return formatted context
}
```

2. Add keywords to trigger detection:
```rust
let needs_tool = self.mentions_tool(
    &query_lower,
    &["keyword1", "keyword2"]
);
```

3. Integrate in `get_relevant_context()`:
```rust
if needs_tool {
    if let Some(ctx) = self.get_tool_context() {
        context.push_str("\n## Tool Context\n");
        context.push_str(&ctx);
    }
}
```

4. Add tests in `tests/test_context_agent.rs`

5. Document in `docs/AGENTIC_CONTEXT.md`

### Adding System Information

1. Add field to `SystemInfo` struct in `system_info.rs`:
```rust
pub struct SystemInfo {
    pub os: String,
    pub new_field: String,  // Add here
}
```

2. Detect in `SystemInfo::detect()`:
```rust
let new_field = detect_new_field();
```

3. Format in `format_context()`:
```rust
context.push_str(&format!("\nNew Field: {}", self.new_field));
```

4. Add tests for new detection

## Testing Architecture

```
tests/
├── integration/
│   ├── cli_tests.rs           # CLI argument parsing
│   ├── config_tests.rs        # Config persistence
│   └── llm_client_tests.rs    # Backend integration
│
├── e2e_command_generation.rs  # Full command generation flow
├── e2e_update.rs              # Update functionality
└── test_context_agent.rs      # Context detection

src/
├── config.rs                  # Unit tests inline
├── system_info.rs             # Unit tests inline
└── tool_detection.rs          # Unit tests inline
```

### Test Strategy

- **Unit Tests**: Test individual functions in isolation
- **Integration Tests**: Test module interactions
- **E2E Tests**: Test complete user workflows
- **Mock External Dependencies**: Use httpmock for API calls

## Performance Considerations

### Startup Time

- Config loading: ~1ms
- System detection: ~5ms
- Tool detection: ~10ms (PATH scan + history)
- Context agent: ~50ms (only when triggered)

**Total**: <100ms cold start

### Memory Usage

- Base binary: ~5MB
- Runtime memory: ~10-20MB
- Negligible for modern systems

### Optimization Strategies

1. **Lazy Loading**: Context agent only runs when needed
2. **Caching**: Tool detection results cached per session
3. **Async I/O**: Non-blocking LLM calls
4. **Minimal Dependencies**: Only essential crates

## Security Considerations

### API Keys

- Stored in config file with user-only permissions (600)
- Environment variables preferred for CI/CD
- Never logged or printed

### Command Execution

- Generated commands are displayed, NOT executed
- User must review before running
- Shell wrapper injects into readline buffer

### Context Privacy

- All context fetching is local
- No telemetry or analytics
- Only LLM prompt contains context

### Update Security

- Binaries downloaded from official GitHub releases
- HTTPS only
- Checksum verification (TODO)

## Future Architecture Enhancements

### Planned Improvements

1. **Plugin System**
   - Load external context detectors
   - Custom backend implementations

2. **Caching Layer**
   - Cache LLM responses for identical queries
   - Persistent context cache

3. **Streaming Support**
   - Real-time command generation
   - Progressive output

4. **Multi-step Commands**
   - Chain multiple commands
   - Conditional execution

5. **Learning System**
   - Learn from user's command selections
   - Personalized suggestions

## Contributing to Architecture

Want to extend the architecture? See:

- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [docs/TESTING.md](./TESTING.md) - Testing guide
- [docs/AGENTIC_CONTEXT.md](./AGENTIC_CONTEXT.md) - Context system details

## Questions?

Open an issue for architecture discussions or questions!
