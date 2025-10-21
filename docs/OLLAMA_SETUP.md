# Using ez with Ollama

This guide shows how to use `ez` with Ollama, both locally and remotely.

## Local Ollama Setup

### 1. Install Ollama

```bash
# Linux
curl -fsSL https://ollama.com/install.sh | sh

# macOS
brew install ollama

# Or download from https://ollama.com
```

### 2. Start Ollama

```bash
ollama serve
```

### 3. Pull a Model

```bash
# Small and fast
ollama pull qwen3-coder:latest

# Or a larger model
ollama pull llama2
ollama pull mistral
```

### 4. Use with ez

```bash
# Build ez
cargo build --release

# Run with Ollama backend
./target/release/ez -b ollama

# Or set as default
./target/release/ez --set-backend ollama
./target/release/ez
```

## Remote Ollama Setup

If you have Ollama running on another machine (e.g., a more powerful server), you can connect to it.

### 1. Start Ollama on Remote Server

On your remote machine (e.g., 192.168.0.199):

```bash
# Start Ollama and make it listen on all interfaces
OLLAMA_HOST=0.0.0.0:11434 ollama serve
```

### 2. Set OLLAMA_HOST on Client

On your local machine where you're running `ez`:

```bash
# Set the environment variable
export OLLAMA_HOST="http://192.168.0.199:11434"

# Or add to your shell profile (~/.bashrc or ~/.zshrc)
echo 'export OLLAMA_HOST="http://192.168.0.199:11434"' >> ~/.bashrc
source ~/.bashrc
```

### 3. Test Connection

```bash
# List models on remote Ollama
./target/release/ez -b ollama --list-models

# Should show models from your remote server
```

### 4. Use ez with Remote Ollama

```bash
# Interactive mode
OLLAMA_HOST="http://192.168.0.199:11434" ./target/release/ez -b ollama

# Or if you exported it globally
./target/release/ez -b ollama

# One-shot query
./target/release/ez -b ollama "how do I find large files?"
```

## Configuration Priority

`ez` resolves the Ollama URL in this order:

1. **Config file** (`~/.config/ez-term/config.toml`)
   ```toml
   ollama_url = "http://192.168.0.199:11434"
   ```

2. **Environment variable**
   ```bash
   export OLLAMA_HOST="http://192.168.0.199:11434"
   ```

3. **Default**
   ```
   http://localhost:11434
   ```

## Example: Your Setup

Based on your configuration with Ollama at `192.168.0.199`:

```bash
# Add to your shell profile
echo 'export OLLAMA_HOST="http://192.168.0.199:11434"' >> ~/.bashrc
source ~/.bashrc

# Build ez
cd /home/dev/work/ez-term
cargo build --release
sudo cp target/release/ez /usr/local/bin/

# Set Ollama as default backend
ez --set-backend ollama

# List available models
ez --list-models

# Start interactive mode
ez

# You'll see:
ez - terminal assistant (interactive mode)
Backend: ollama | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel

ez> what is rust?
[AI responds using your remote Ollama...]

ez> how do I use docker?
[Another response...]

ez> exit
Goodbye!
```

## Available Models

Your Ollama instance has these models:
- `qwen3-coder:latest` - 30.5B parameter coding model
- `gpt-oss:20b` - 20.9B parameter model
- `gpt-oss:120b` - 116.8B parameter model
- `nemotron-mini:4b` - 4.2B parameter small/fast model

Use them with:

```bash
ez -b ollama -m qwen3-coder:latest "explain async/await in rust"
ez -b ollama -m nemotron-mini:4b "what is 2+2?"
```

## Troubleshooting

### Connection Refused

```bash
# Test direct connection
curl http://192.168.0.199:11434/api/tags

# If this fails, check:
# 1. Ollama is running on remote machine
# 2. Firewall allows port 11434
# 3. Ollama is listening on 0.0.0.0, not just localhost
```

### Models Not Showing

```bash
# On the remote machine, check models
ollama list

# Pull models if needed
ollama pull qwen3-coder
```

### Wrong URL Being Used

```bash
# Check what URL is being used
# Run with debug (temporarily add eprintln! to code)
# Or check config
cat ~/.config/ez-term/config.toml

# Clear config if needed
rm ~/.config/ez-term/config.toml
```

## Performance Tips

1. **Use appropriate model sizes**
   - Small queries: `nemotron-mini:4b`
   - Coding tasks: `qwen3-coder:latest`
   - Complex reasoning: `gpt-oss:120b`

2. **Network latency**
   - Remote Ollama adds network latency
   - For best performance, use gigabit LAN
   - WiFi may be slower for large responses

3. **Model warm-up**
   - First query loads the model (slow)
   - Subsequent queries are faster
   - Keep Ollama running to avoid reloading

## Next Steps

- Set `OLLAMA_HOST` permanently in your shell profile
- Set Ollama as your default backend: `ez --set-backend ollama`
- Try different models: `ez --list-models`
- Enjoy fast, private AI assistance!
