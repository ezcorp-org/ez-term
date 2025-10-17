# ✅ Setup Complete!

## Current Configuration

✅ **Installed**: `~/.local/bin/ez`
✅ **Backend**: Ollama (192.168.0.199:11434)
✅ **Default Model**: nemotron-mini:4b

## ⚠️ Important: Set Environment Variable

For `ez` to work automatically, add this to your shell profile:

```bash
echo 'export OLLAMA_HOST="http://192.168.0.199:11434"' >> ~/.bashrc
source ~/.bashrc
```

Or if you use zsh:
```bash
echo 'export OLLAMA_HOST="http://192.168.0.199:11434"' >> ~/.zshrc
source ~/.zshrc
```

## 🎯 Quick Test

```bash
# Test it works
OLLAMA_HOST="http://192.168.0.199:11434" ~/.local/bin/ez "say hello"
```

Expected output:
```
Gathering system context...

Hello! How can I assist you today?
```

## 🚀 Start Using It

After setting the environment variable above:

```bash
# Quick question
~/.local/bin/ez "how do I list files?"

# Interactive mode (best experience!)
~/.local/bin/ez
```

## 📝 Your Configuration File

Location: `~/.config/ez-term/config.toml`

Current settings:
```toml
backend = "ollama"
model = "nemotron-mini:4b"
```

## 💡 The Issue You Hit

You got a "404 Not Found" error because:
1. No model was configured
2. The default fallback was "llama2"
3. Your server doesn't have "llama2"

**Solution**: We set `nemotron-mini:4b` as your default model.

## 🔄 Change Models Anytime

```bash
# Set a different default
~/.local/bin/ez --set-model qwen3-coder:latest

# Or use -m flag for one query
~/.local/bin/ez -m gpt-oss:120b "complex question"

# List all your models
~/.local/bin/ez --list-models
```

## ✨ Ready to Use!

Everything is configured and working. Just remember:

1. **Set OLLAMA_HOST in your shell profile** (one time, see above)
2. **Then use**: `~/.local/bin/ez "your question"`

See **QUICKSTART.md** for more examples and tips!
