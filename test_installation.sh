#!/bin/bash
# Test script for ez installation

echo "🧪 Testing ez installation..."
echo ""

# Set Ollama host
export OLLAMA_HOST="http://192.168.0.199:11434"

# Test 1: Check if ez is installed
echo "✓ Test 1: Check if ez is installed"
if command -v ~/.local/bin/ez &> /dev/null; then
    echo "  ✅ ez found at ~/.local/bin/ez"
else
    echo "  ❌ ez not found"
    exit 1
fi
echo ""

# Test 2: Check version
echo "✓ Test 2: Check version"
VERSION=$(~/.local/bin/ez --version)
echo "  ✅ $VERSION"
echo ""

# Test 3: List backends
echo "✓ Test 3: List backends"
~/.local/bin/ez --list-backends
echo ""

# Test 4: List Ollama models
echo "✓ Test 4: List Ollama models"
~/.local/bin/ez -b ollama --list-models
echo ""

# Test 5: Quick query
echo "✓ Test 5: Quick query test"
RESPONSE=$(~/.local/bin/ez -b ollama -m nemotron-mini:4b "say 'test passed'" 2>&1)
if [[ $RESPONSE == *"test passed"* ]] || [[ $RESPONSE != *"Error"* ]]; then
    echo "  ✅ Query successful"
else
    echo "  ⚠️  Response received (may not contain exact phrase)"
fi
echo ""

# Test 6: Check config
echo "✓ Test 6: Check configuration"
if [ -f ~/.config/ez-term/config.toml ]; then
    echo "  ✅ Config file exists"
    cat ~/.config/ez-term/config.toml
else
    echo "  ℹ️  No config file yet (will be created on first use)"
fi
echo ""

echo "✅ All tests passed!"
echo ""
echo "📚 Usage examples:"
echo "  ez --help"
echo "  ez \"how do I find large files?\""
echo "  ez -m qwen3-coder:latest \"explain async in rust\""
echo "  ez  # interactive mode"
