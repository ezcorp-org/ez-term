# Installation Guide

## 🚀 Quick Install (Recommended)

### One-Line Install

Copy and paste this into your terminal:

```bash
curl -sSL https://raw.githubusercontent.com/chopshop1/ez-term/main/install-ez.sh | bash
```

**What it does:**
1. ✅ Installs Rust/Cargo (if not already installed)
2. ✅ Clones and builds `ez` from source
3. ✅ Installs binary to `~/.local/bin/ez`
4. ✅ Sets up shell integration (zsh/bash)
5. ✅ Configures Ollama host
6. ✅ Adds to your PATH automatically

After installation:
```bash
source ~/.zshrc  # or ~/.bashrc
ez "find large files"
```

---

## 📦 Via Cargo (Rust Package Manager)

If you already have Rust installed:

```bash
cargo install ez-cli
```

This installs the binary but you'll need to manually set up shell integration:

### Zsh Setup
```bash
curl -sSL https://raw.githubusercontent.com/chopshop1/ez-term/main/ez.zsh -o ~/.ez.zsh
echo 'source ~/.ez.zsh' >> ~/.zshrc
echo 'export OLLAMA_HOST="http://localhost:11434"' >> ~/.zshrc
source ~/.zshrc
```

### Bash Setup
```bash
curl -sSL https://raw.githubusercontent.com/chopshop1/ez-term/main/ez.sh -o ~/.ez.sh
echo 'source ~/.ez.sh' >> ~/.bashrc
echo 'export OLLAMA_HOST="http://localhost:11434"' >> ~/.bashrc
source ~/.bashrc
```

---

## 🔧 Manual Installation

### Prerequisites
- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Git

### Steps

```bash
# Clone repository
git clone https://github.com/chopshop1/ez-term.git
cd ez-term

# Build release binary
cargo build --release

# Install binary
mkdir -p ~/.local/bin
cp target/release/ez ~/.local/bin/
chmod +x ~/.local/bin/ez

# Copy shell integration
cp ez.zsh ~/.ez.zsh  # for zsh
# or
cp ez.sh ~/.ez.sh    # for bash

# Add to shell config
echo 'source ~/.ez.zsh' >> ~/.zshrc
echo 'export OLLAMA_HOST="http://localhost:11434"' >> ~/.zshrc

# Reload shell
source ~/.zshrc
```

---

## 🍺 Homebrew (Coming Soon)

```bash
brew install ez-cli
```

> Note: Homebrew formula is planned for a future release

---

## 🐳 Docker (Alternative)

If you prefer containerized installation:

```bash
docker run -it --rm \
  -v ~/.config/ez-term:/root/.config/ez-term \
  -e OLLAMA_HOST=http://host.docker.internal:11434 \
  chopshop1/ez "find large files"
```

> Note: Docker image is planned for a future release

---

## 🔍 Verify Installation

```bash
# Check binary is installed
which ez
# Should output: /home/username/.local/bin/ez

# Check version
ez --version

# List available models (requires Ollama running)
ez --list-models

# Test command generation
ez "show disk usage"
```

---

## 🛠️ Configuration

### Set Default Backend

```bash
ez --set-backend ollama   # Local Ollama
ez --set-backend groq     # Cloud Groq (requires GROQ_API_KEY)
ez --set-backend openai   # Cloud OpenAI (requires OPENAI_API_KEY)
```

### Set Default Model

```bash
ez --set-model qwen3-coder:latest  # Best for command generation
ez --set-model llama3.1:latest
ez --set-model gpt-oss:120b
```

### Set Ollama Host

```bash
export OLLAMA_HOST="http://192.168.1.100:11434"
```

Add to your `~/.zshrc` or `~/.bashrc` to persist.

---

## 📚 Post-Installation

### First Steps

1. **Test basic command:**
   ```bash
   ez "list files"
   ```

2. **Try shell integration:**
   ```bash
   ez "find markdown files"
   # Command appears in your terminal - press Enter to run
   ```

3. **Explore interactive mode:**
   ```bash
   ez
   ez> show disk usage
   ez> compress this folder
   ez> exit
   ```

### Recommended Settings

For best results with command generation:

```bash
export OLLAMA_HOST="http://localhost:11434"
ez --set-backend ollama
ez --set-model qwen3-coder:latest
```

**Why qwen3-coder?**
- 30B parameter coding model
- Excellent at following structured output format
- Generates accurate, executable commands
- Faster than larger models

---

## ❓ Troubleshooting

### "ez: command not found"

Add `~/.local/bin` to your PATH:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### "Ollama request failed: Connection refused"

Make sure Ollama is running:
```bash
# Check if Ollama is running
curl http://localhost:11434/api/tags

# Start Ollama (if needed)
ollama serve
```

### "No model configured"

Set a default model:
```bash
ez --set-model qwen3-coder:latest
```

### Shell integration not working

Make sure you sourced the shell config:
```bash
source ~/.zshrc  # or ~/.bashrc
```

---

## 🆘 Getting Help

- **Documentation**: https://github.com/chopshop1/ez-term
- **Issues**: https://github.com/chopshop1/ez-term/issues
- **Command help**: `ez --help`

---

## 🚀 You're Ready!

Start generating commands:

```bash
ez "find files modified today"
ez "show git status"
ez "compress documents folder"
ez "list biggest directories"
```

Enjoy! 🎉
