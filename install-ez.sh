#!/bin/bash
# ez - One-line installer
# Usage: curl -sSL https://raw.githubusercontent.com/chopshop1/ez-term/main/install-ez.sh | bash

set -e

echo "🚀 Installing ez - AI-powered CLI command generator"
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

# Check if Rust/Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "📦 Downloading ez from GitHub..."
git clone https://github.com/chopshop1/ez-term.git
cd ez-term

echo "🔨 Building ez (this may take a minute)..."
cargo build --release

echo "📥 Installing binary to ~/.local/bin..."
mkdir -p ~/.local/bin
cp target/release/ez ~/.local/bin/ez
chmod +x ~/.local/bin/ez

# Detect shell and add to PATH if needed
SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
fi

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  ~/.local/bin is not in your PATH"
    if [ -n "$SHELL_RC" ]; then
        echo "Adding to $SHELL_RC..."
        echo '' >> "$SHELL_RC"
        echo '# Added by ez installer' >> "$SHELL_RC"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    fi
fi

# Ask about shell integration
echo ""
echo "🔧 Shell Integration Setup"
echo ""
echo "Would you like to install shell integration? (recommended)"
echo "This allows commands to appear directly in your terminal."
echo ""
read -p "Install shell integration? [Y/n] " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
    if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
        echo "Installing for zsh..."
        cp ez.zsh ~/.ez.zsh
        if ! grep -q "source.*\.ez\.zsh" "$HOME/.zshrc" 2>/dev/null; then
            echo '' >> "$HOME/.zshrc"
            echo '# Load ez command generator' >> "$HOME/.zshrc"
            echo 'if [ -f ~/.ez.zsh ]; then' >> "$HOME/.zshrc"
            echo '    source ~/.ez.zsh' >> "$HOME/.zshrc"
            echo 'fi' >> "$HOME/.zshrc"
        fi
    elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
        echo "Installing for bash..."
        cp ez.sh ~/.ez.sh
        if ! grep -q "source.*\.ez\.sh" "$HOME/.bashrc" 2>/dev/null; then
            echo '' >> "$HOME/.bashrc"
            echo '# Load ez command generator' >> "$HOME/.bashrc"
            echo 'if [ -f ~/.ez.sh ]; then' >> "$HOME/.bashrc"
            echo '    source ~/.ez.sh' >> "$HOME/.bashrc"
            echo 'fi' >> "$HOME/.bashrc"
        fi
    fi
fi

# Prompt for Ollama host
echo ""
echo "🌐 Ollama Configuration"
echo ""
echo "Enter your Ollama host URL (press Enter for default: http://localhost:11434):"
read -r OLLAMA_URL
OLLAMA_URL=${OLLAMA_URL:-http://localhost:11434}

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "OLLAMA_HOST" "$SHELL_RC" 2>/dev/null; then
        echo '' >> "$SHELL_RC"
        echo '# Ollama configuration for ez' >> "$SHELL_RC"
        echo "export OLLAMA_HOST=\"$OLLAMA_URL\"" >> "$SHELL_RC"
    fi
fi

# Set default model
echo ""
echo "🤖 Setting default model..."
export OLLAMA_HOST="$OLLAMA_URL"
~/.local/bin/ez --set-backend ollama
~/.local/bin/ez --set-model qwen3-coder:latest 2>/dev/null || echo "Note: Set your model later with: ez --set-model <model-name>"

# Cleanup
cd ~
rm -rf "$TMP_DIR"

echo ""
echo "✅ Installation complete!"
echo ""
echo "📖 Quick Start:"
echo "   1. Reload your shell: source $SHELL_RC"
echo "   2. Try it: ez \"find large files\""
echo ""
echo "📚 Documentation: https://github.com/chopshop1/ez-term"
echo ""
