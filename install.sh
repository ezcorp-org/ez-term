#!/bin/bash
# Installation script for ez

set -e

echo "📦 Installing ez..."

# Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Install to ~/.local/bin
echo "📁 Installing to ~/.local/bin..."
mkdir -p ~/.local/bin
cp target/release/ez ~/.local/bin/
chmod +x ~/.local/bin/ez

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "⚠️  ~/.local/bin is not in PATH"
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

# Set up OLLAMA_HOST environment variable
echo ""
echo "🔧 Configuration:"
echo ""
echo "To use your Ollama server at 192.168.0.199, add this to your ~/.bashrc or ~/.zshrc:"
echo "    export OLLAMA_HOST=\"http://192.168.0.199:11434\""
echo ""

# Set Ollama as default backend
echo "⚙️  Setting Ollama as default backend..."
OLLAMA_HOST="http://192.168.0.199:11434" ~/.local/bin/ez --set-backend ollama

# Set default model to nemotron-mini (fast and available)
echo "⚙️  Setting default model..."
OLLAMA_HOST="http://192.168.0.199:11434" ~/.local/bin/ez --set-model nemotron-mini:4b

echo ""
echo "✅ Installation complete!"
echo ""
echo "📝 Next steps:"
echo "1. Add to your shell profile:"
echo "     echo 'export OLLAMA_HOST=\"http://192.168.0.199:11434\"' >> ~/.bashrc"
echo "     source ~/.bashrc"
echo ""
echo "2. Test it:"
echo "     ez --list-models"
echo "     ez \"what is rust?\""
echo "     ez  # for interactive mode"
echo ""
echo "3. Set a default model (optional):"
echo "     ez --set-model qwen3-coder:latest"
echo ""
