# ez

**A safe CLI command generator powered by local or cloud LLMs.**

Ask for what you want in natural language, get a safe command you can run.

## Features

- 🛡️ **Safety-first**: Defaults to non-destructive preview commands
- 🚀 **Interactive prompt** with command history (use arrow keys!)
- 🔄 **Multi-backend support**: Local Ollama or cloud APIs (Groq, OpenAI)
- 🖥️ **System-aware**: Knows your OS, shell, and installed tools
- ⚡ **Streaming responses**: See commands as they're generated
- 💾 **Persistent configuration**: Save your preferred backend and model

## Quick Start

### Installation

```bash
cargo build --release
cp target/release/ez ~/.local/bin/
```

### Setup for Ollama

```bash
# Set your Ollama server
export OLLAMA_HOST="http://192.168.0.199:11434"

# Set a good model for command generation (coding models work best!)
ez --set-model qwen3-coder:latest

# Or use the default backend
ez --set-backend ollama
```

## Usage

### Generate Commands

Just ask what you want to do:

```bash
$ ez "find files larger than 100MB"
find . -type f -size +100M -print

$ ez "compress this directory"
zip -r archive.zip .

$ ez "show disk usage"
du -h --max-depth=1 ~

$ ez "list all rust files"
ls *.rs
```

### Safety Built-in

`ez` refuses unsafe operations:

```bash
$ ez "delete all tmp files"
echo "Refusing: unsafe operation"
```

Instead, it gives you safe preview commands:

```bash
$ ez "find tmp files to delete"
find . -type f -name "*.tmp" -print
```

### Interactive Mode

```bash
ez

ez - terminal assistant (interactive mode)
Backend: ollama | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel

ez> find large files
find . -type f -size +100M -print

ez> list all python files
ls *.py

ez> exit
Goodbye!
```

## How It Works

`ez` is a **command generator**, not a general chatbot. It:

1. Takes your natural language request
2. Generates a **single, safe terminal command**
3. Outputs **only the command** (no explanations)
4. Follows strict safety rules (read-only by default)

### Safety Rules

- ✅ Defaults to preview/read-only commands
- ✅ No `sudo`, `rm -rf`, or destructive operations
- ✅ Properly quotes paths and handles edge cases
- ✅ If unsafe and no safe alternative exists, it refuses

## Configuration

### Environment Variables

```bash
export OLLAMA_HOST="http://192.168.0.199:11434"  # Your Ollama server
export GROQ_API_KEY="gsk_..."                    # For Groq backend
export OPENAI_API_KEY="sk-..."                   # For OpenAI backend
```

### Set Defaults

```bash
# Set backend
ez --set-backend ollama

# Set model (use coding models for best results!)
ez --set-model qwen3-coder:latest

# List available models
ez --list-models

# List backends
ez --list-backends
```

### Config File

Location: `~/.config/ez-term/config.toml`

```toml
backend = "ollama"
model = "qwen3-coder:latest"
```

## Available Backends

### Ollama (Recommended)

```bash
export OLLAMA_HOST="http://192.168.0.199:11434"
ez --set-backend ollama
ez --set-model qwen3-coder:latest  # Best for commands!
```

**Best models for command generation:**
- `qwen3-coder:latest` ⭐ (30.5B - best balance)
- `gpt-oss:120b` (most powerful, slower)
- ~~`nemotron-mini:4b`~~ (too small, use for simple queries only)

### Groq (Cloud, Free)

```bash
export GROQ_API_KEY="gsk_..."
ez --set-backend groq
```

### OpenAI (Cloud, Paid)

```bash
export OPENAI_API_KEY="sk-..."
ez --set-backend openai
```

## Examples

### File Operations

```bash
ez "find all markdown files"
# find . -type f -name "*.md" -print

ez "show size of each directory"
# du -h --max-depth=1 .

ez "count lines in python files"
# find . -name "*.py" -exec wc -l {} +
```

### Git Operations

```bash
ez "show git log"
# git log --oneline -10

ez "list uncommitted changes"
# git status --short

ez "show files changed in last commit"
# git diff --name-only HEAD~1
```

### System Info

```bash
ez "show memory usage"
# free -h

ez "list running processes"
# ps aux | head -20

ez "check disk space"
# df -h
```

### Archives

```bash
ez "compress current directory"
# zip -r archive.zip .

ez "list contents of tar file"
# tar -tzf file.tar.gz

ez "extract zip file"
# unzip -l file.zip
```

## Interactive Mode Features

- ⬆️⬇️ Arrow keys navigate command history
- `Ctrl+C` to cancel current input
- `Ctrl+D` or `exit` to quit
- History saved to `~/.config/ez-term/history.txt`

## Command History

Your queries are saved automatically:

```bash
# View history
cat ~/.config/ez-term/history.txt

# In interactive mode, press ⬆️ to see previous queries
```

## Tips

1. **Use coding models** like `qwen3-coder:latest` for best results
2. **Be specific** - "find large files in /var/log" is better than "find files"
3. **Review before running** - always check the command before executing
4. **Use preview mode** - default behavior shows what would be affected
5. **Interactive mode** - great for exploring options

## Safety Philosophy

`ez` is designed to be **helpful but safe**:

- **Default to preview**: Show what would be affected, don't modify
- **Refuse dangerous ops**: If unsafe with no preview, it refuses
- **Single command**: Outputs exactly one command, easy to review
- **No surprises**: No hidden flags, no piping to shell automatically

## Troubleshooting

### "404 Not Found"

You haven't set a model:

```bash
ez --set-model qwen3-coder:latest
```

### Incomplete or wrong commands

Use a better model:

```bash
# Switch to coding model
ez --set-model qwen3-coder:latest

# Coding models are MUCH better at following instructions
```

### "Connection failed"

Check your OLLAMA_HOST:

```bash
echo $OLLAMA_HOST
# Should show: http://your-server:11434

# Test connection
curl $OLLAMA_HOST/api/tags
```

## What Makes This Different?

Unlike general chatbots:

- ✅ Outputs **only commands**, no explanations
- ✅ **Safety-first** design with preview mode
- ✅ **System-aware** (knows your OS, tools, shell)
- ✅ **Single command** output (easy to review and run)
- ✅ Works **offline** with local Ollama

Unlike `man` pages:

- ✅ Natural language interface
- ✅ Context-aware suggestions
- ✅ Safe defaults built-in

## Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Files

- Binary: `~/.local/bin/ez`
- Config: `~/.config/ez-term/config.toml`
- History: `~/.config/ez-term/history.txt`

## Documentation

- `QUICKSTART.md` - Quick setup guide
- `OLLAMA_SETUP.md` - Ollama-specific setup
- `SETUP_COMPLETE.md` - Configuration details

## License

MIT
