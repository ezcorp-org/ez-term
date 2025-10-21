# Troubleshooting Critical Command Blocking

## Issue: Dangerous commands still being inserted into terminal

If you run `ez "delete all files"` and the command is still being inserted into your terminal, follow these steps:

---

## Step 1: Check if Backend is Configured

The `ez` command needs an LLM backend to generate commands. Check if you have one configured:

```bash
# Check for backends
echo "Groq: ${GROQ_API_KEY:+configured}"
echo "OpenAI: ${OPENAI_API_KEY:+configured}"
which ollama
```

**If all show empty/not found**, you need to set up a backend:

### Option A: Groq (Recommended - Fast & Free)
```bash
export GROQ_API_KEY='your-api-key-here'
# Add to ~/.zshrc to make permanent:
echo "export GROQ_API_KEY='your-api-key-here'" >> ~/.zshrc
```

### Option B: OpenAI
```bash
export OPENAI_API_KEY='your-api-key-here'
echo "export OPENAI_API_KEY='your-api-key-here'" >> ~/.zshrc
```

### Option C: Ollama (Local)
```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull a model
ollama pull llama3.2
```

---

## Step 2: Verify Shell Wrapper is Loaded

Check if the `ez` function is defined in your shell:

```bash
type ez
```

**Expected output:**
```
ez is a shell function from /home/dev/.ez.zsh
```

**If it shows:** `ez is /home/dev/.local/bin/ez` (NOT a function)

Then the wrapper isn't loaded. Load it:

```bash
source ~/.ez.zsh
```

To make permanent (NixOS users):
```nix
# In your home-manager configuration:
programs.zsh.initExtra = ''
  source ~/.ez.zsh
'';
```

---

## Step 3: Test with Mock Command

Test the validator directly without needing an LLM:

```bash
# Run the test script
/home/dev/work/ez-term/test_critical_blocking.sh
```

This should show all ✅ PASS marks. If any tests fail, the issue is in the code.

---

## Step 4: Test with Real Backend

Once you have a backend configured:

```bash
ez "delete all files"
```

**Expected output:**

```
🤖 Generating command...

💡 Finds and deletes all files in the current directory

════════════════════════════════════════════════════════════════
🚨 CRITICAL COMMAND BLOCKED
════════════════════════════════════════════════════════════════

🚨 CRITICAL: This command will DELETE FILES RECURSIVELY!
This find command will delete files matching the criteria.
This operation is irreversible and can destroy important data.
If you really understand the consequences, you must type it manually.

Generated command (NOT inserted):
find . -type f -delete

If you really need to run this, type it manually.

# Your terminal input line should be EMPTY ← IMPORTANT
```

---

## Step 5: Check What Command is Being Generated

If the LLM is generating a different command than `find . -type f -delete`, you can see what it's generating:

```bash
# Run without the wrapper
/home/dev/.local/bin/ez "delete all files"
```

This will show you the raw output from the binary. Check if it:
- Shows the command
- Has `critical:` prefix
- Has the full warning message

---

## Step 6: Verify Binary Version

```bash
~/.local/bin/ez --version
```

Should show: `ez 0.5.6`

If not, reinstall:

```bash
cd /home/dev/work/ez-term
cargo build --release
cp target/release/ez ~/.local/bin/ez
cp scripts/ez.zsh ~/.ez.zsh
source ~/.ez.zsh
```

---

## Step 7: Check for Command Variations

The LLM might generate variations. These are ALL blocked:

✅ `find . -delete`
✅ `find . -type f -delete`
✅ `find / -name '*.txt' -delete`
✅ `find . -exec rm {} \;`
✅ `find . -exec rm -f {} \;`
✅ `find . -type f -exec rm -rf {} \;`

But these are NOT blocked (they're safe):

✅ `find . -type f` (just lists)
✅ `find . -type f -print` (explicit print)

---

## Debugging: Enable Verbose Output

To see exactly what's happening:

```bash
# Set verbose mode
set -x
ez "delete all files"
set +x
```

This will show you:
1. If the wrapper function is being called
2. What the binary outputs
3. If the critical detection is working
4. If the insertion is being blocked

---

## Common Issues

### Issue: "ez binary not found"
**Solution:**
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Issue: Wrapper function not defined
**Solution:**
```bash
source ~/.ez.zsh
```

### Issue: "No backend available"
**Solution:** Configure Groq/OpenAI/Ollama (see Step 1)

### Issue: Command is inserted anyway
**Possible causes:**
1. Wrapper not loaded (function not defined) - Check `type ez`
2. Backend not configured - Check environment variables
3. LLM generating different command - Run binary directly to see output
4. Shell wrapper not updated - Re-copy from `scripts/ez.zsh`

---

## Manual Test Without Backend

You can test the blocking mechanism without a backend:

```bash
# Create a fake output
cat > /tmp/test_blocking.sh << 'EOF'
#!/usr/bin/env bash

# Simulate ez binary output
cat << 'OUTPUT'
💡 Finds and deletes all files

critical: 🚨 CRITICAL: This command will DELETE FILES RECURSIVELY!
This find command will delete files matching the criteria.
This operation is irreversible and can destroy important data.
If you really understand the consequences, you must type it manually.
find . -type f -delete
OUTPUT
EOF

chmod +x /tmp/test_blocking.sh

# Override the binary path
export EZ_BIN=/tmp/test_blocking.sh

# Test
ez "test"
```

This should show the red warning box and NOT insert anything.

---

## Still Not Working?

If you've followed all steps and it's still not working:

1. **Check shell:** Are you using bash or zsh?
   - For bash: `source ~/path/to/ez.sh`
   - For zsh: `source ~/.ez.zsh`

2. **Check wrapper file location:**
   ```bash
   ls -la ~/.ez.zsh
   ```

3. **Verify wrapper content:**
   ```bash
   grep "critical:" ~/.ez.zsh
   ```
   Should show the critical detection logic

4. **Run comprehensive test:**
   ```bash
   /home/dev/work/ez-term/test_critical_blocking.sh
   ```
   All tests should pass

5. **Check for shell conflicts:**
   ```bash
   alias ez  # Should show nothing
   ```

---

## Contact

If none of these steps work, please provide:

1. Output of: `type ez`
2. Output of: `~/.local/bin/ez --version`
3. Output of: `echo $GROQ_API_KEY $OPENAI_API_KEY`
4. Output of: `/home/dev/.local/bin/ez "delete all files"` (raw binary)
5. Output of: `ez "delete all files"` (with wrapper)
6. Your shell: `echo $SHELL`
