# Changelog

## v0.1.0 - Initial Release

### Features

#### Interactive Mode 🎉
- **Default mode**: Run `ez` without arguments to enter interactive prompt
- **Command history**: Navigate previous queries with ⬆️/⬇️ arrow keys
- **Persistent history**: Saved to `~/.config/ez-term/history.txt` across sessions
- **Ctrl+C**: Cancel current input
- **Ctrl+D or `exit`**: Gracefully exit

#### Multi-Backend Support
- **Groq**: Free, fast cloud API (default)
- **OpenAI**: ChatGPT API support
- **Ollama**: Local LLM support for privacy

#### Three Usage Modes
1. **Interactive**: `ez` (default)
2. **One-shot**: `ez "your question"`
3. **Piped**: `echo "question" | ez`

#### System Context Detection
- OS and architecture detection
- Shell environment (bash/zsh/fish)
- Memory information
- Installed tools scanning (git, docker, npm, etc.)
- Shell history analysis for frequently used commands

#### Configuration Management
- Persistent config at `~/.config/ez-term/config.toml`
- Backend selection: `ez --set-backend groq`
- Model selection: `ez --set-model llama-3.3-70b-versatile`
- API key support via environment variables

#### Streaming Responses
- Real-time token streaming from all backends
- Progressive output display

### Commands

```bash
ez                              # Interactive mode
ez "question"                   # One-shot mode
ez -b groq                      # Specify backend
ez -m llama-3.3-70b-versatile   # Specify model
ez --list-backends              # List available backends
ez --list-models                # List models for current backend
ez --set-backend groq           # Set default backend
ez --set-model <model>          # Set default model
ez --help                       # Show help
ez --version                    # Show version
```

### Technical Details

**Built with:**
- Rust 2021 edition
- tokio (async runtime)
- reqwest (HTTP client with rustls)
- rustyline (interactive prompt with history)
- clap (CLI parsing)
- serde (serialization)
- sysinfo (system detection)

**Binary name:** `ez`

**Config locations:**
- Config: `~/.config/ez-term/config.toml`
- History: `~/.config/ez-term/history.txt`

**Supported platforms:**
- Linux ✅
- macOS (should work, untested)
- Windows (should work with some features limited)

### Known Limitations

- History search (Ctrl+R) is basic
- No conversation context between queries in interactive mode
- System detection is read-only (no system modifications)

### Future Enhancements (Potential)

- Conversation memory in interactive mode
- Custom system prompts
- Plugin system for additional backends
- Shell integration (completion, aliases)
- Configuration via CLI flags
- Multi-line input support
- Syntax highlighting in responses
