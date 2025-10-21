# Installation Guide

Complete installation instructions for `ez` - AI-powered command generator.

## Quick Start (Development)

For developers working on `ez`, use the one-line install script:

```bash
./dev-install.sh
```

This will:
- ✅ Build the Rust binary
- ✅ Install to `~/.local/bin/ez`
- ✅ Set up shell integration (zsh/bash)
- ✅ Configure default backend
- ✅ Test the installation

## Prerequisites

### Required
- **Rust** (1.70+): Install from https://rustup.rs
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Optional (choose one)
- **Ollama** (local LLM): https://ollama.ai
- **Groq API Key** (cloud): https://console.groq.com
- **OpenAI API Key** (cloud): https://platform.openai.com

## Installation Methods

### Method 1: Development Install (Recommended for Testing)

```bash
# Clone the repository
git clone git@github.com:ezcorp-org/ez-term.git
cd ez-term

# Run dev install script
./dev-install.sh

# Reload shell
source ~/.zshrc  # or ~/.bashrc for bash
```

### Method 2: Production Install (from Release)

```bash
# Download and run installer
curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash
```

### Method 3: Manual Install

```bash
# Build
cargo build --release

# Install binary
sudo cp target/release/ez /usr/local/bin/ez
# OR for user install
mkdir -p ~/.local/bin
cp target/release/ez ~/.local/bin/ez

# Install shell wrapper (zsh)
cp scripts/ez.zsh ~/.ez.zsh
echo 'source ~/.ez.zsh' >> ~/.zshrc
source ~/.zshrc

# OR for bash
cp scripts/ez.sh ~/.ez.sh
echo 'source ~/.ez.sh' >> ~/.bashrc
source ~/.bashrc
```

## Post-Installation Setup

### 1. Configure Backend

**Option A: Local (Ollama)**
```bash
# Install Ollama from https://ollama.ai
# Start Ollama
ollama serve

# Set backend
ez --set-backend ollama

# Pull a model (recommended)
ollama pull qwen2.5-coder:latest
ez --set-model qwen2.5-coder:latest
```

**Option B: Cloud (Groq - Free tier available)**
```bash
# Get API key from https://console.groq.com
export GROQ_API_KEY='your-api-key-here'

# Add to shell RC file for persistence
echo 'export GROQ_API_KEY="your-api-key"' >> ~/.zshrc

# Set backend
ez --set-backend groq
```

**Option C: Cloud (OpenAI)**
```bash
# Get API key from https://platform.openai.com
export OPENAI_API_KEY='your-api-key-here'

# Add to shell RC file
echo 'export OPENAI_API_KEY="your-api-key"' >> ~/.zshrc

# Set backend
ez --set-backend openai
```

### 2. Verify Installation

```bash
# Check version
ez --version

# List backends
ez --list-backends

# Test command generation
ez "list files in current directory"
```

### 3. Shell Integration Test

The shell wrapper allows commands to be injected into your terminal:

```bash
# Type this (don't press Enter yet)
ez "find large files"

# The command will appear in your terminal ready to execute
# You can edit it before running
```

## Troubleshooting

### Binary not found

**Problem**: `ez: command not found`

**Solution**: Add `~/.local/bin` to your PATH:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Backend errors

**Problem**: `Error: No backend configured`

**Solution**: Set up a backend (see Post-Installation Setup above)

### Ollama connection failed

**Problem**: `Error connecting to Ollama`

**Solutions**:
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# Start Ollama
ollama serve

# Check custom host
export OLLAMA_HOST=http://localhost:11434
```

### Shell wrapper not working

**Problem**: Commands execute instead of appearing in terminal

**Solution**: Make sure you're using the shell function, not the binary directly:
```bash
# Wrong (calls binary directly)
/usr/local/bin/ez "list files"

# Right (calls shell wrapper)
ez "list files"
```

### Permission denied

**Problem**: `Permission denied` when installing

**Solutions**:
```bash
# Option 1: Install to user directory (no sudo needed)
mkdir -p ~/.local/bin
cp target/release/ez ~/.local/bin/ez

# Option 2: Use sudo for system-wide install
sudo cp target/release/ez /usr/local/bin/ez
```

## Testing Installation

Run the installation test suite:

```bash
./test-install.sh
```

This verifies:
- Script is executable
- Shell detection works
- Required files exist
- Wrapper syntax is valid
- Installation steps succeed

## Updating

### Automatic Update
```bash
ez --update
```

### Manual Update
```bash
cd ez-term
git pull
./dev-install.sh
```

## Uninstallation

```bash
# Remove binary
rm ~/.local/bin/ez
# OR if installed system-wide
sudo rm /usr/local/bin/ez

# Remove shell wrapper (zsh)
rm ~/.ez.zsh
# Remove from ~/.zshrc (delete line: source ~/.ez.zsh)

# OR for bash
rm ~/.ez.sh
# Remove from ~/.bashrc (delete line: source ~/.ez.sh)

# Remove config (optional)
rm -rf ~/.config/ez-term
```

## Platform-Specific Notes

### Linux
- Uses GNU-style commands
- Package managers: apt, dnf, pacman
- Service manager: systemctl

### macOS
- Uses BSD-style commands
- Package manager: brew
- Service manager: launchctl
- May require Xcode Command Line Tools: `xcode-select --install`

### Windows (WSL)
- Install in WSL2 environment
- Use Linux instructions
- Backend should be set to cloud (Groq/OpenAI) unless Ollama runs in WSL

## Development Installation

For contributors and developers:

```bash
# Clone repo
git clone git@github.com:ezcorp-org/ez-term.git
cd ez-term

# Install dev dependencies
cargo build

# Run tests
cargo test

# Install for testing
./dev-install.sh

# Make changes and reinstall
./dev-install.sh
```

## Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `GROQ_API_KEY` | Groq API authentication | `export GROQ_API_KEY="gsk_..."` |
| `OPENAI_API_KEY` | OpenAI API authentication | `export OPENAI_API_KEY="sk-..."` |
| `OLLAMA_HOST` | Ollama server URL | `export OLLAMA_HOST="http://localhost:11434"` |

## Next Steps

After installation:

1. **Read the docs**: Check out [USAGE.md](./USAGE.md) for usage examples
2. **Configure**: Set up your preferred backend and model
3. **Test**: Try some example commands
4. **Customize**: Explore configuration options

## Support

- **Issues**: https://github.com/ezcorp-org/ez-term/issues
- **Discussions**: https://github.com/ezcorp-org/ez-term/discussions
- **Documentation**: https://github.com/ezcorp-org/ez-term/tree/main/docs

## Related Documentation

- [README.md](../README.md) - Project overview
- [USAGE.md](./USAGE.md) - Usage guide
- [TESTING.md](./TESTING.md) - Testing guide
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contributing guide
