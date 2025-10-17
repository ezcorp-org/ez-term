# Testing Guide

## Testing Interactive Mode

### 1. Get a Groq API Key

Visit [https://console.groq.com/keys](https://console.groq.com/keys) and create a free account to get your API key.

### 2. Set the API Key

```bash
export GROQ_API_KEY="gsk_your_key_here"
```

### 3. Test Interactive Mode

```bash
./target/release/ez
```

You should see:
```
ez - terminal assistant (interactive mode)
Backend: groq | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel

ez>
```

### 4. Test Features

**Test basic query:**
```
ez> how do I list files?
```

**Test history navigation:**
- Type a query and press Enter
- Type another query and press Enter
- Press ⬆️ (up arrow) - you should see your previous query
- Press ⬆️ again - you should see your first query
- Press ⬇️ (down arrow) - you should navigate forward

**Test Ctrl+C:**
- Start typing a query
- Press Ctrl+C - should cancel and show a new prompt

**Test exit:**
- Type `exit` or `quit` and press Enter
- Or press Ctrl+D
- Should display "Goodbye!" and exit

### 5. Verify History Persistence

```bash
# Check history file was created
cat ~/.config/ez-term/history.txt

# Should contain your previous queries
```

### 6. Test with Different Backends

```bash
# Test with specific model
./target/release/ez -m mixtral-8x7b-32768

# If you have Ollama running
./target/release/ez -b ollama

# If you have OpenAI key
export OPENAI_API_KEY="sk-..."
./target/release/ez -b openai
```

## Testing One-Shot Mode

```bash
# Single query
./target/release/ez "how do I check disk usage?"

# Should process query and exit
```

## Testing Piped Input

```bash
echo "explain git rebase" | ./target/release/ez

# Should process and exit
```

## Testing Configuration

```bash
# Set backend
./target/release/ez --set-backend groq

# Set model
./target/release/ez --set-model llama-3.3-70b-versatile

# Verify config
cat ~/.config/ez-term/config.toml

# Start interactive mode (should use configured defaults)
./target/release/ez
```

## Expected Behavior

### Interactive Mode
✅ Shows prompt with backend info
✅ Accepts queries at `ez>` prompt
✅ Streams responses from LLM
✅ Supports up/down arrows for history
✅ Ctrl+C cancels current input
✅ `exit`, `quit`, or Ctrl+D exits gracefully
✅ Saves history to ~/.config/ez-term/history.txt
✅ Loads previous history on startup

### One-Shot Mode
✅ Processes single query
✅ Exits after response
✅ Works with all flags (-b, -m)

### Piped Mode
✅ Reads from stdin
✅ Processes and exits
✅ No interactive prompt

## Troubleshooting

### "Groq API key not configured"
```bash
export GROQ_API_KEY="gsk_your_key_here"
```

### Arrow keys print escape sequences instead of navigating
- This shouldn't happen with rustyline
- Make sure you're in interactive mode (no query argument)
- Check terminal supports ANSI escape codes

### History not saving
```bash
# Check directory exists
ls -la ~/.config/ez-term/

# Check permissions
chmod 755 ~/.config/ez-term
```

### No response in interactive mode
- Check API key is set correctly
- Check internet connection for cloud backends
- Try with verbose error: the error should be displayed

## Performance Testing

### Response Speed
Time how long responses take:
```bash
time ./target/release/ez "what is rust?"
```

Groq should be very fast (< 5 seconds for most queries).

### Memory Usage
Check memory consumption in interactive mode:
```bash
./target/release/ez &
PID=$!
ps -p $PID -o rss,vsz,cmd
```

Should be lightweight (< 50MB typical).

## Full Test Session Example

```bash
# Setup
export GROQ_API_KEY="gsk_..."

# Test interactive
./target/release/ez
ez> how do I find files?
[waits for response]
ez> explain git rebase
[waits for response]
[press up arrow twice to see "how do I find files?"]
[press down arrow once to see "explain git rebase"]
ez> exit
Goodbye!

# Test one-shot
./target/release/ez "what is docker?"

# Test piped
echo "how do I compress files?" | ./target/release/ez

# Test configuration
./target/release/ez --list-backends
./target/release/ez --list-models
./target/release/ez --set-backend groq
./target/release/ez --set-model llama-3.3-70b-versatile

# Verify config
cat ~/.config/ez-term/config.toml

# Verify history
cat ~/.config/ez-term/history.txt
```

All tests should pass! 🎉
