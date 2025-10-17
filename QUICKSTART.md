# Quick Start Guide

## ✅ Already Installed!

`ez` is installed at `~/.local/bin/ez`

## 🚀 Setup (Required - First Time Only)

### 1. Set Environment Variable

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
export OLLAMA_HOST="http://192.168.0.199:11434"
```

Then reload:
```bash
source ~/.bashrc
```

### 2. Set Default Model

**IMPORTANT**: Use a coding model for best results!

```bash
# Set default model (RECOMMENDED)
~/.local/bin/ez --set-model qwen3-coder:latest      # ⭐ Best for command generation!

# Other options:
~/.local/bin/ez --set-model gpt-oss:120b            # Most powerful (slower)
~/.local/bin/ez --set-model nemotron-mini:4b        # Fast but less accurate
```

**Why coding models?** They follow instructions better and generate more accurate commands.

## 📝 Usage

`ez` is a **command generator** - it outputs safe shell commands, not explanations.

### Generate Commands

```bash
# Find files
~/.local/bin/ez "find files larger than 100MB"
# Output: find . -type f -size +100M -print

# Compress directory
~/.local/bin/ez "compress this directory"
# Output: zip -r archive.zip .

# Show disk usage
~/.local/bin/ez "show disk usage"
# Output: du -h --max-depth=1 ~
```

### Interactive Mode (Best Experience!)

```bash
~/.local/bin/ez
```

You'll get:
```
ez - terminal assistant (interactive mode)
Backend: ollama | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel

ez> how do I compress a directory?
[AI responds...]

ez> explain docker compose
[Another response...]

ez> [press ⬆️ to see previous queries]
ez> exit
Goodbye!
```

### Use Different Models

```bash
# Fast model for simple questions
~/.local/bin/ez -m nemotron-mini:4b "what is 2+2?"

# Coding model for programming help
~/.local/bin/ez -m qwen3-coder:latest "explain rust closures"

# Most powerful model for complex questions
~/.local/bin/ez -m gpt-oss:120b "explain quantum computing"
```

## 🔍 Useful Commands

```bash
# List all available models on your Ollama
~/.local/bin/ez --list-models

# See current configuration
cat ~/.config/ez-term/config.toml

# View command history
cat ~/.config/ez-term/history.txt

# Get help
~/.local/bin/ez --help
```

## ⚡ Make It Easier (Optional)

If you want to just type `ez` instead of the full path:

### Check if already in PATH:
```bash
which ez
```

If it says "not found", add to your shell profile:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.local/bin:$PATH"

# Reload
source ~/.bashrc
```

Now you can just use:
```bash
ez "your question"
```

## 🎯 Your Available Models

On your Ollama server (192.168.0.199):

| Model | Size | Best For | Speed |
|-------|------|----------|-------|
| `nemotron-mini:4b` | 4.2B | Simple queries, quick answers | ⚡⚡⚡ Fast |
| `qwen3-coder:latest` | 30.5B | Coding, technical questions | ⚡⚡ Medium |
| `gpt-oss:20b` | 20.9B | General purpose | ⚡⚡ Medium |
| `gpt-oss:120b` | 116.8B | Complex reasoning, analysis | ⚡ Slow |

## 🆘 Troubleshooting

### "404 Not Found" Error

This means no model is configured. Fix it:

```bash
# Set a default model
~/.local/bin/ez --set-model nemotron-mini:4b

# Or use -m flag
~/.local/bin/ez -m nemotron-mini:4b "your question"
```

### "Connection Failed" Error

Check your environment variable:

```bash
echo $OLLAMA_HOST
# Should show: http://192.168.0.199:11434

# If empty, set it:
export OLLAMA_HOST="http://192.168.0.199:11434"
```

### Command Not Found

Use full path:
```bash
~/.local/bin/ez "your question"
```

Or add to PATH (see "Make It Easier" above).

## 💡 Tips

1. **Interactive mode is best** - Use `ez` without arguments for conversations
2. **History works** - Press ⬆️/⬇️ to navigate previous queries
3. **Start with fast model** - Use `nemotron-mini:4b` by default, switch to larger models when needed
4. **System-aware** - ez knows your OS, shell, and installed tools
5. **Tab-friendly** - Works great for piping: `echo "question" | ez`

## 📚 Examples

```bash
# System help
ez "how do I check disk usage?"

# Git help
ez "what's the difference between git merge and rebase?"

# Docker help
ez "how do I list running containers?"

# Rust help (use coding model)
ez -m qwen3-coder:latest "explain ownership in rust"

# File operations
ez "how do I find all .rs files modified today?"

# Interactive session
ez
ez> how do I compress files with tar?
ez> what flags should I use for gzip compression?
ez> exit
```

## ✅ You're Ready!

Just remember these two commands:

1. **Quick question**: `~/.local/bin/ez "your question"`
2. **Interactive mode**: `~/.local/bin/ez`

Enjoy your AI-powered terminal assistant! 🎉
