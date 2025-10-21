# ez - Command Generator Mode

`ez` has been transformed into a **safe CLI command generator**.

## What Changed

### Before
- General terminal assistant
- Provided explanations and advice
- Chatbot-style responses

### Now
- **Command generator only**
- Outputs single, safe shell commands
- No explanations, just the command
- Safety-first design

## How It Works

### Input
Natural language request:
```
"find files larger than 100MB"
```

### Output
Safe shell command only:
```
find . -type f -size +100M -print
```

## Safety Rules

### Always Safe (Preview Mode)
- `find ... -print` (no -delete)
- `ls`, `du`, `df` (read-only)
- `grep`, `awk`, `sed` (no -i)
- `tar -tzf` (list, not extract)
- `git status`, `git log` (read-only)

### Refuses Unsafe Operations
```bash
$ ez "delete all tmp files"
echo "Refusing: unsafe operation"

$ ez "sudo install package"
echo "Refusing: unsafe operation"
```

### Preview Instead
If you ask for something destructive, it shows a preview:
```bash
$ ez "find tmp files to delete"
find . -type f -name "*.tmp" -print  # Shows matches, doesn't delete
```

## Best Models

**Command generation requires coding models!**

### ⭐ Recommended
- **qwen3-coder:latest** (30.5B) - Best balance of speed and accuracy
- Follows instructions precisely
- Generates correct POSIX commands
- Handles edge cases well

### Good
- **gpt-oss:120b** (116.8B) - Most accurate, but slower
- Perfect for complex commands

### Not Recommended
- ~~nemotron-mini:4b~~ - Too small, incomplete output
- ~~Other chat models~~ - Add unnecessary prose

## Setup

```bash
# 1. Set Ollama server
export OLLAMA_HOST="http://192.168.0.199:11434"

# 2. Use a coding model!
ez --set-model qwen3-coder:latest

# 3. Test it
ez "find large files"
```

## Examples

### File Operations
```bash
$ ez "find all markdown files"
find . -type f -name "*.md" -print

$ ez "show size of each directory"
du -h --max-depth=1 .

$ ez "count lines in rust files"
find . -name "*.rs" -exec wc -l {} +
```

### Git Operations
```bash
$ ez "show recent commits"
git log --oneline -10

$ ez "list unstaged changes"
git diff --name-only

$ ez "show files in last commit"
git diff --name-only HEAD~1
```

### System Info
```bash
$ ez "check memory usage"
free -h

$ ez "show disk space"
df -h

$ ez "list largest directories"
du -h --max-depth=1 | sort -rh | head -10
```

### Archives
```bash
$ ez "compress current directory"
zip -r archive.zip .

$ ez "list tar contents"
tar -tzf file.tar.gz

$ ez "extract specific file from zip"
unzip -l file.zip  # Preview mode - shows contents
```

## Interactive Mode

```bash
$ ez

ez - terminal assistant (interactive mode)
Backend: ollama | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel

ez> find large files
find . -type f -size +100M -print

ez> compress this folder
zip -r archive.zip .

ez> show git status
git status --short

ez> exit
Goodbye!
```

## What It Won't Do

### Never Outputs
- `sudo` commands
- `rm -rf` or destructive deletes
- Blind `chmod 777`
- Raw device writes (`dd`)
- Network-destructive commands
- Commands that expose secrets

### Always Defaults To
- Read-only operations
- Preview mode (`-print`, `--dry-run`)
- Quoted paths
- Safe patterns

## Copy-Paste Ready

All commands are designed to be:
- ✅ Single line (easy to copy)
- ✅ Safe to run (preview by default)
- ✅ Properly quoted (handles spaces)
- ✅ POSIX compatible (works everywhere)

## Tips

1. **Be specific**: "find large files in /var/log" is better than "find files"
2. **Review first**: Always read the command before running it
3. **Use preview**: Default behavior shows what would be affected
4. **Interactive mode**: Great for exploring multiple commands
5. **Coding model**: Always use `qwen3-coder:latest` for best results

## Comparison

### Traditional Way
```bash
$ man find
# Read through manual...
# Try to construct command...
# Hope syntax is correct...
```

### With ez
```bash
$ ez "find files modified today"
find . -type f -mtime 0 -print
# Copy, paste, run
```

## System Prompt

`ez` uses this prompt internally:

```
You are a CLI command generator. Given a natural-language request,
output exactly one terminal command that is safe to run.

Rules:
- Output only the command, on one line
- No prose, no code fences, no explanations
- Default to preview/non-destructive actions
- If unsafe with no safe preview: echo "Refusing: unsafe operation"
- Never output: sudo, rm -rf, destructive operations
- Quote paths properly
- Prefer read-only tools
```

## Why This Design?

### Safety First
- Can't accidentally run destructive commands
- Preview mode shows impact first
- Refusal when truly unsafe

### Easy to Use
- Single command output
- Copy-paste ready
- No need to parse explanations

### System-Aware
- Knows your OS, shell, tools
- Suggests appropriate commands
- Handles platform differences

## Current Configuration

Your setup:
```toml
backend = "ollama"
model = "qwen3-coder:latest"
```

Location: `~/.config/ez-term/config.toml`

## Testing

Test the command generator:

```bash
# Safe operations
ez "list files"              # ls
ez "show disk usage"         # du -h
ez "find python files"       # find ... -print

# Refuses unsafe
ez "delete everything"       # echo "Refusing: unsafe operation"
ez "sudo install"            # echo "Refusing: unsafe operation"

# Preview mode
ez "find tmp files"          # find ... -print (not -delete)
```

## Summary

`ez` is now a **specialized tool** for safe command generation:

- ✅ Outputs commands only
- ✅ Safety-first design
- ✅ Preview mode by default
- ✅ Refuses truly unsafe operations
- ✅ Copy-paste ready output
- ✅ Works offline with Ollama

Perfect for:
- Learning new commands
- Remembering complex syntax
- Safe command exploration
- Quick command generation

Not for:
- General chat
- Explanations
- Tutorials
- Debugging code

Use the right tool for the job!
