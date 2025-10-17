# Shell Integration - Auto-Execute Commands

The `ez` shell wrapper allows you to generate commands and execute them with confirmation.

## Features

- 🎯 **Generates command** from natural language
- 👀 **Shows command** before executing
- ✅ **Prompts for confirmation** (Y/n/edit)
- ✏️ **Edit before execution** option
- 📋 **Adds to shell history** if declined
- 🛡️ **Safety-first** - you control execution

## Installation

### For Bash

Add to your `~/.bashrc`:

```bash
# Load ez command generator
if [ -f ~/work/ez-term/ez.sh ]; then
    source ~/work/ez-term/ez.sh
fi

# Set Ollama host (if not already set)
export OLLAMA_HOST="http://192.168.0.199:11434"
```

Then reload:
```bash
source ~/.bashrc
```

### For Zsh

Add to your `~/.zshrc`:

```zsh
# Load ez command generator
if [ -f ~/work/ez-term/ez.zsh ]; then
    source ~/work/ez-term/ez.zsh
fi

# Set Ollama host (if not already set)
export OLLAMA_HOST="http://192.168.0.199:11434"
```

Then reload:
```zsh
source ~/.zshrc
```

## Usage

### Basic Usage

```bash
$ ez "find large files"

🤖 Generating command...

📋 Generated command:
   find . -type f -size +100M -print

Execute this command? [Y/n/e(dit)]
```

### Response Options

#### Y or Enter - Execute
```bash
Execute this command? [Y/n/e(dit)]
▶️  Executing: find . -type f -size +100M -print
[command output appears here]
```

#### n - Don't Execute (adds to history)
```bash
Execute this command? [Y/n/e(dit)] n
❌ Command not executed
```

The command is added to your shell history, so you can press ⬆️ to get it later.

#### e - Edit First
```bash
Execute this command? [Y/n/e(dit)] e
Edit command: find . -type f -size +100M -print
[you can edit the command here]
▶️  Executing: [your edited command]
```

### Safety Refusals

If the command is unsafe, it's automatically refused:

```bash
$ ez "delete all files"

🤖 Generating command...

📋 Generated command:
   echo "Refusing: unsafe operation"

⚠️  Command refused for safety
```

## Examples

### File Operations

```bash
$ ez "show disk usage"
🤖 Generating command...

📋 Generated command:
   du -h --max-depth=1 ~

Execute this command? [Y/n/e(dit)] y
▶️  Executing: du -h --max-depth=1 ~
100M    /home/dev/work
50M     /home/dev/docs
...
```

### Git Operations

```bash
$ ez "show recent commits"
🤖 Generating command...

📋 Generated command:
   git log --oneline -10

Execute this command? [Y/n/e(dit)]
▶️  Executing: git log --oneline -10
a1b2c3d Latest changes
...
```

### Compression

```bash
$ ez "compress this folder"
🤖 Generating command...

📋 Generated command:
   zip -r archive.zip .

Execute this command? [Y/n/e(dit)] e
Edit command: zip -r my-project.zip .
▶️  Executing: zip -r my-project.zip .
  adding: file1.txt
  adding: file2.txt
...
```

## Interactive Mode Still Available

If you run `ez` without arguments, it enters interactive mode (no auto-execute):

```bash
$ ez

ez - terminal assistant (interactive mode)
Backend: ollama | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel

ez> find large files
find . -type f -size +100M -print

ez> show git log
git log --oneline -10

ez> exit
Goodbye!
```

In interactive mode, commands are **not** auto-executed - they're just displayed.

## Flags Still Work

Special flags bypass the wrapper:

```bash
ez --list-models       # Lists models
ez --set-model qwen3-coder:latest
ez --list-backends
ez --help
```

## How It Works

1. **Wrapper intercepts**: The `ez` function in your shell catches the call
2. **Calls binary**: Runs `~/.local/bin/ez` to generate command
3. **Extracts command**: Parses output to get the generated command
4. **Shows preview**: Displays the command to you
5. **Prompts**: Asks if you want to execute
6. **Executes**: Runs command if confirmed

## Safety Features

### Preview Before Execute
You **always** see the command before it runs.

### Edit Option
You can modify the command before execution.

### History Integration
If you decline, the command is added to history for later use.

### Refusal Detection
Commands marked as "Refusing: unsafe operation" are not executed.

## Workflow

### Traditional CLI
```
1. Ask how to do something
2. Read man pages
3. Try to construct command
4. Hope it's right
5. Execute and see what happens
```

### With ez Shell Integration
```
1. Ask in natural language: ez "find large files"
2. See generated command: find . -type f -size +100M -print
3. Confirm or edit
4. Execute
```

## Tips

1. **Review commands**: Always read what will be executed
2. **Use edit mode**: Tweak commands before running
3. **Decline if unsure**: Press 'n' to add to history instead
4. **Interactive for exploration**: Use `ez` (no args) to explore without executing
5. **Trust but verify**: The AI is good, but always check the command

## Configuration

The wrapper automatically sets `OLLAMA_HOST` if not already set. You can override:

```bash
# In your ~/.bashrc or ~/.zshrc
export OLLAMA_HOST="http://your-server:11434"
```

## Uninstalling

To disable the wrapper, remove or comment out the source line in your shell config:

```bash
# ~/.bashrc or ~/.zshrc
# source ~/work/ez-term/ez.sh  # Commented out
```

Then reload your shell.

## Comparison

### Without Shell Integration

```bash
$ ~/.local/bin/ez "find large files"
Gathering system context...

find . -type f -size +100M -print

$ # Now you copy-paste the command
$ find . -type f -size +100M -print
```

### With Shell Integration

```bash
$ ez "find large files"
🤖 Generating command...

📋 Generated command:
   find . -type f -size +100M -print

Execute this command? [Y/n/e(dit)]
▶️  Executing: find . -type f -size +100M -print
[results appear immediately]
```

## Security

### What's Safe
- ✅ You control execution (requires confirmation)
- ✅ Commands are shown before running
- ✅ Edit option available
- ✅ Refusals are detected
- ✅ Uses `eval` safely (you see the command first)

### What to Watch
- ⚠️ Still eval's the command (after confirmation)
- ⚠️ Review commands before pressing Y
- ⚠️ Edit mode allows any command

**Bottom line**: You're in control. The wrapper adds convenience, not risk.

## Troubleshooting

### "ez: command not found"

The wrapper isn't loaded. Check your shell config:

```bash
# Bash
cat ~/.bashrc | grep ez.sh

# Zsh
cat ~/.zshrc | grep ez.zsh
```

Add the source line if missing, then reload.

### No command generated

Check your model is set:

```bash
cat ~/.config/ez-term/config.toml
```

Should show:
```toml
backend = "ollama"
model = "qwen3-coder:latest"
```

### Commands timeout

The model might be slow. Try a smaller model or increase timeout.

## Next Steps

1. **Add to shell config**: Source the wrapper
2. **Test it**: `ez "show disk space"`
3. **Explore**: Try different requests
4. **Customize**: Edit the wrapper script if needed

Enjoy seamless command generation with execution! 🎉
