#!/bin/bash
# ez - Universal installer (no Rust required!)
# Works on Linux (x86_64, ARM64), macOS (Intel, Apple Silicon)
# Usage: curl -sSL https://raw.githubusercontent.com/chopshop1/ez-term/main/install.sh | bash

set -e

echo "🚀 Installing ez - AI-powered CLI command generator"
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

# Determine platform
case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)
                PLATFORM="linux-x86_64"
                BINARY_NAME="ez"
                ARCHIVE_EXT="tar.gz"
                ;;
            aarch64|arm64)
                PLATFORM="linux-aarch64"
                BINARY_NAME="ez"
                ARCHIVE_EXT="tar.gz"
                ;;
            *)
                echo "❌ Unsupported architecture: $ARCH"
                echo "Supported: x86_64, aarch64/arm64"
                exit 1
                ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64)
                PLATFORM="macos-x86_64"
                BINARY_NAME="ez"
                ARCHIVE_EXT="tar.gz"
                ;;
            arm64)
                PLATFORM="macos-aarch64"
                BINARY_NAME="ez"
                ARCHIVE_EXT="tar.gz"
                ;;
            *)
                echo "❌ Unsupported architecture: $ARCH"
                echo "Supported: x86_64, arm64"
                exit 1
                ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "❌ Windows installation not yet supported via this script"
        echo "Please download from: https://github.com/chopshop1/ez-term/releases"
        exit 1
        ;;
    *)
        echo "❌ Unsupported operating system: $OS"
        exit 1
        ;;
esac

echo "📦 Detected platform: $OS $ARCH ($PLATFORM)"
echo ""

# Get latest release version
echo "🔍 Fetching latest release..."
LATEST_RELEASE=$(curl -s https://api.github.com/repos/chopshop1/ez-term/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "⚠️  No release found, using direct build from main branch..."
    DOWNLOAD_URL="https://github.com/chopshop1/ez-term/releases/download/latest/ez-${PLATFORM}.${ARCHIVE_EXT}"
else
    echo "✅ Latest version: $LATEST_RELEASE"
    DOWNLOAD_URL="https://github.com/chopshop1/ez-term/releases/download/${LATEST_RELEASE}/ez-${PLATFORM}.${ARCHIVE_EXT}"
fi

# Create temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download binary
echo ""
echo "📥 Downloading ez binary..."
if ! curl -L -o "ez-${PLATFORM}.${ARCHIVE_EXT}" "$DOWNLOAD_URL" 2>/dev/null; then
    echo "❌ Download failed. The release might not exist yet."
    echo ""
    echo "📝 Please install manually:"
    echo "   1. Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "   2. Run: curl -sSL https://raw.githubusercontent.com/chopshop1/ez-term/main/install-ez.sh | bash"
    echo ""
    echo "Or visit: https://github.com/chopshop1/ez-term/releases"
    exit 1
fi

# Extract binary
echo "📦 Extracting..."
if [ "$ARCHIVE_EXT" = "tar.gz" ]; then
    tar xzf "ez-${PLATFORM}.${ARCHIVE_EXT}"
else
    unzip -q "ez-${PLATFORM}.${ARCHIVE_EXT}"
fi

# Install binary
echo "📥 Installing to ~/.local/bin..."
mkdir -p ~/.local/bin
cp "$BINARY_NAME" ~/.local/bin/ez
chmod +x ~/.local/bin/ez

# Detect shell
SHELL_RC=""
SHELL_WRAPPER=""
if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
    SHELL_WRAPPER="ez.zsh"
    SHELL_NAME="zsh"
elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
    SHELL_WRAPPER="ez.sh"
    SHELL_NAME="bash"
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

# Install shell integration
echo ""
echo "🔧 Installing shell integration for $SHELL_NAME..."
if [ -n "$SHELL_WRAPPER" ]; then
    curl -sSL "https://raw.githubusercontent.com/chopshop1/ez-term/main/${SHELL_WRAPPER}" -o "$HOME/.${SHELL_WRAPPER}"

    if ! grep -q "source.*\.${SHELL_WRAPPER}" "$SHELL_RC" 2>/dev/null; then
        echo '' >> "$SHELL_RC"
        echo '# Load ez command generator' >> "$SHELL_RC"
        echo "if [ -f ~/.${SHELL_WRAPPER} ]; then" >> "$SHELL_RC"
        echo "    source ~/.${SHELL_WRAPPER}" >> "$SHELL_RC"
        echo 'fi' >> "$SHELL_RC"
    fi
fi

# Configure Ollama
echo ""
echo "🌐 Ollama Configuration"
echo ""
read -p "Enter Ollama host URL [http://localhost:11434]: " OLLAMA_URL
OLLAMA_URL=${OLLAMA_URL:-http://localhost:11434}

if [ -n "$SHELL_RC" ]; then
    if ! grep -q "OLLAMA_HOST" "$SHELL_RC" 2>/dev/null; then
        echo '' >> "$SHELL_RC"
        echo '# Ollama configuration for ez' >> "$SHELL_RC"
        echo "export OLLAMA_HOST=\"$OLLAMA_URL\"" >> "$SHELL_RC"
    fi
fi

# Set default configuration
echo ""
echo "🤖 Configuring defaults..."
export OLLAMA_HOST="$OLLAMA_URL"
~/.local/bin/ez --set-backend ollama 2>/dev/null || true
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
echo "💡 The generated command will appear in your terminal ready to run!"
echo ""
echo "📚 Documentation: https://github.com/chopshop1/ez-term"
echo ""
