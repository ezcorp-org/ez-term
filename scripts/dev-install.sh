#!/usr/bin/env bash
# Development install script for ez
# This script builds and installs ez locally for testing

set -e  # Exit on error

echo "🔧 EZ Development Install Script"
echo "================================"
echo ""

# Detect shell
SHELL_NAME=$(basename "$SHELL")
echo "📍 Detected shell: $SHELL_NAME"

# Build the project
echo ""
echo "🔨 Building ez..."
cargo build --release

# Create local bin directory
mkdir -p "$HOME/.local/bin"

# Install binary
echo ""
echo "📦 Installing binary to ~/.local/bin/ez..."
cp target/release/ez "$HOME/.local/bin/ez"
chmod +x "$HOME/.local/bin/ez"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "⚠️  Warning: ~/.local/bin is not in your PATH"
    echo "   Add this line to your shell RC file:"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

# Install shell wrapper
echo ""
if [ "$SHELL_NAME" = "zsh" ]; then
    echo "🐚 Installing zsh wrapper..."
    cp scripts/ez.zsh "$HOME/.ez.zsh"

    # Check if already sourced
    if [ -w "$HOME/.zshrc" ]; then
        if ! grep -q "source.*\.ez\.zsh" "$HOME/.zshrc" 2>/dev/null; then
            echo "source ~/.ez.zsh" >> "$HOME/.zshrc"
            echo "✅ Added to ~/.zshrc"
        else
            echo "✅ Already in ~/.zshrc"
        fi
    else
        echo "⚠️  Cannot write to ~/.zshrc (permission denied)"
        echo "   Please add this line manually:"
        echo "   source ~/.ez.zsh"
    fi

    echo ""
    echo "To activate in this session, run:"
    echo "  source ~/.zshrc"

elif [ "$SHELL_NAME" = "bash" ]; then
    echo "🐚 Installing bash wrapper..."
    cp scripts/ez.sh "$HOME/.ez.sh"

    # Check if already sourced
    if [ -w "$HOME/.bashrc" ]; then
        if ! grep -q "source.*\.ez\.sh" "$HOME/.bashrc" 2>/dev/null; then
            echo "source ~/.ez.sh" >> "$HOME/.bashrc"
            echo "✅ Added to ~/.bashrc"
        else
            echo "✅ Already in ~/.bashrc"
        fi
    else
        echo "⚠️  Cannot write to ~/.bashrc (permission denied)"
        echo "   Please add this line manually:"
        echo "   source ~/.ez.sh"
    fi

    echo ""
    echo "To activate in this session, run:"
    echo "  source ~/.bashrc"
else
    echo "⚠️  Unsupported shell: $SHELL_NAME"
    echo "   Shell wrapper not installed. You can still use: ez <query>"
fi

# Test binary installation
echo ""
echo "🧪 Testing binary..."
if "$HOME/.local/bin/ez" --version; then
    echo "✅ Binary installed successfully!"
else
    echo "❌ Binary test failed"
    exit 1
fi

# Check for backends
echo ""
echo "🔍 Checking for LLM backends..."

# Check for Ollama
if command -v ollama &> /dev/null; then
    echo "✅ Ollama found: $(which ollama)"
    OLLAMA_RUNNING=false
    if curl -s http://localhost:11434/api/tags &> /dev/null; then
        echo "   ✅ Ollama is running"
        OLLAMA_RUNNING=true
    else
        echo "   ⚠️  Ollama not running. Start with: ollama serve"
    fi
else
    echo "⚠️  Ollama not found (install from https://ollama.ai)"
fi

# Check for API keys
if [ -n "$GROQ_API_KEY" ]; then
    echo "✅ GROQ_API_KEY is set"
elif [ -n "$OPENAI_API_KEY" ]; then
    echo "✅ OPENAI_API_KEY is set"
else
    echo "⚠️  No cloud API keys found (GROQ_API_KEY or OPENAI_API_KEY)"
fi

# Setup backend
echo ""
echo "⚙️  Setting up default backend..."

if [ "$OLLAMA_RUNNING" = true ]; then
    echo "Setting backend to Ollama (local)..."
    "$HOME/.local/bin/ez" --set-backend ollama
    echo "✅ Backend set to: ollama"
elif [ -n "$GROQ_API_KEY" ]; then
    echo "Setting backend to Groq (cloud)..."
    "$HOME/.local/bin/ez" --set-backend groq
    echo "✅ Backend set to: groq"
elif [ -n "$OPENAI_API_KEY" ]; then
    echo "Setting backend to OpenAI (cloud)..."
    "$HOME/.local/bin/ez" --set-backend openai
    echo "✅ Backend set to: openai"
else
    echo "⚠️  No backend available. Please set up:"
    echo "   - Install Ollama: https://ollama.ai"
    echo "   - OR set GROQ_API_KEY: export GROQ_API_KEY='your-key'"
    echo "   - OR set OPENAI_API_KEY: export OPENAI_API_KEY='your-key'"
fi

# Print next steps
echo ""
echo "════════════════════════════════"
echo "✨ Installation Complete!"
echo "════════════════════════════════"
echo ""
echo "Next steps:"
echo "1. Reload your shell:"
if [ "$SHELL_NAME" = "zsh" ]; then
    echo "   source ~/.zshrc"
elif [ "$SHELL_NAME" = "bash" ]; then
    echo "   source ~/.bashrc"
fi
echo ""
echo "2. Test it out:"
echo "   ez \"list files in current directory\""
echo ""
echo "3. Available commands:"
echo "   ez --help              # Show help"
echo "   ez --list-backends     # List available backends"
echo "   ez --set-backend NAME  # Change backend"
echo "   ez --update            # Update to latest version"
echo ""
echo "Happy commanding! 🚀"
