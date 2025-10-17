# Installation Instructions for ez

## ✅ Binary Installed
- Location: `~/.local/bin/ez`
- Version: 0.1.0

## 🔧 Shell Integration Setup

Since your `.zshrc` is managed by Nix/home-manager, you need to add the configuration to your home-manager setup.

### Option 1: Manual Source (Quick Test)

In any new terminal session, run:

```zsh
source ~/work/ez-term/ez.zsh
export OLLAMA_HOST="http://192.168.0.199:11434"
```

Then test:
```zsh
ez "find large files"
```

The command should appear in your terminal prompt ready to execute.

### Option 2: Add to Nix Home Manager (Permanent)

Find your home-manager configuration file (usually `~/.config/home-manager/home.nix` or similar) and add:

```nix
programs.zsh = {
  initExtra = ''
    # Load ez command generator
    if [ -f ~/work/ez-term/ez.zsh ]; then
      source ~/work/ez-term/ez.zsh
    fi

    # Set Ollama host
    export OLLAMA_HOST="http://192.168.0.199:11434"
  '';
};
```

Then rebuild:
```bash
home-manager switch
```

## 📝 How It Works

When you run:
```zsh
ez "find large files"
```

1. **Generates command** using Ollama
2. **Displays command**:
   ```
   📋 Generated command:
      find . -type f -size +100M -print
   ```
3. **Injects into terminal** using `print -z`
4. Command appears in your prompt ready to run/edit

## 🎯 Example Usage

```zsh
$ ez "show disk usage"
🤖 Generating command...

📋 Generated command:
   df -h

$ df -h█  # ← Command appears here, press Enter or edit
```

## ⚠️ Important Notes

- The command is **NOT auto-executed** - it just appears in your terminal
- You can **edit** it before pressing Enter
- You can **cancel** with Ctrl+C
- Safe commands only (refuses `sudo`, `rm -rf`, etc.)

## 🧪 Testing

To test without modifying your config:

```zsh
# Open new terminal
zsh

# Source the wrapper
source ~/work/ez-term/ez.zsh
export OLLAMA_HOST="http://192.168.0.199:11434"

# Test it
ez "list files"

# You should see the command appear in your prompt
```

## 🔍 Verification

Check installation:
```zsh
~/.local/bin/ez --version
# Should show: ez 0.1.0

~/.local/bin/ez --list-models
# Should show available models
```

## 💡 Current Setup

- **Shell**: zsh (Nix-managed)
- **Ollama**: http://192.168.0.199:11434
- **Model**: qwen3-coder:latest (best for commands)
- **Binary**: ~/.local/bin/ez
- **Wrapper**: ~/work/ez-term/ez.zsh

## 🚀 Ready to Use!

The installation is complete. Just source the wrapper and start generating commands!
