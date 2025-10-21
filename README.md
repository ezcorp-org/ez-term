# ez - Privacy-First AI CLI Command Generator

[![GitHub release](https://img.shields.io/github/v/release/ezcorp-org/ez-term)](https://github.com/ezcorp-org/ez-term/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey)](https://github.com/ezcorp-org/ez-term/releases)
[![Tests](https://img.shields.io/badge/tests-85%20passing-brightgreen)](https://github.com/ezcorp-org/ez-term)
[![Privacy](https://img.shields.io/badge/privacy-local%20first-green)](https://ollama.ai)

**100% Local. 100% Private. 100% Yours.**

Ez is a privacy-first CLI command generator designed to run **completely locally** using Ollama. No data leaves your machine. No telemetry. No cloud dependencies. Just you, your terminal, and AI-powered command generation that respects your privacy.

Ask for what you want in natural language, get a safe, context-aware command - all processed locally on your machine.

```bash
$ ez "find large files"
💡 Finds files larger than 100MB in current directory and shows their size in human-readable format.

find . -type f -size +100M -exec ls -lh {} \;
```

[Quick Start](#quick-start) • [Features](#features) • [Installation](#installation) • [Documentation](#documentation) • [Contributing](#contributing)

## ✨ Features

### 🔒 Privacy-First Design

- **100% Local Processing**: Runs entirely on your machine with Ollama
- **Zero Telemetry**: No tracking, no analytics, no data collection
- **No Cloud Dependencies**: Works completely offline (when using Ollama)
- **Your Data Stays Yours**: All context fetching happens locally
- **Open Source**: Audit the code yourself - nothing hidden

> **Why Privacy Matters**: Your command history, project structure, and shell configurations reveal sensitive information about your work. Ez is built from the ground up to keep this data on your machine where it belongs.

### 🧠 Intelligent & Context-Aware

- **OS-Specific**: Generates commands tailored to your OS (Linux/macOS/Windows)
- **Tool-Aware**: Knows what tools you have installed
- **Project-Aware**: Understands your git repo, npm project, Python venv, etc.
- **Alias-Aware**: Learns your custom shell aliases and functions
- **Agentic Context System**: Intelligently fetches relevant information based on your query

### 🛡️ Safe by Default

- **Preview First**: Never executes commands automatically
- **Non-Destructive**: Defaults to safe, read-only operations
- **Refuses Unsafe**: Won't generate dangerous commands like `rm -rf /`
- **Shell Integration**: Commands appear in your terminal for review before running

### 🚀 Powerful Features

- **Primary: Local Ollama** - Recommended for privacy and offline use
- **Optional: Cloud APIs** - Groq and OpenAI available if needed
- **Interactive Mode**: REPL with history and auto-completion
- **Shell Integration**: Commands inject into your shell (zsh/bash)
- **Self-Updating**: `ez --update` to get the latest version
- **Cross-Platform**: Works on Linux, macOS, and Windows

## 🎯 Quick Start

### 1. Install

```bash
# One-line install (no Rust required)
curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash
```

Or with Rust:
```bash
git clone https://github.com/ezcorp-org/ez-term.git
cd ez-term
./dev-install.sh
```

### 2. Configure Backend

**🔒 Recommended: Ollama (100% Local & Private)**

```bash
# 1. Install Ollama
# Visit https://ollama.ai to download and install for your platform

# 2. Pull a recommended model
ollama pull qwen2.5-coder:latest

# 3. Configure ez to use Ollama
ez --set-backend ollama
ez --set-model qwen2.5-coder:latest

# That's it! Everything runs locally now.
```

**Why Ollama?**
- ✅ **100% Private**: Your data never leaves your machine
- ✅ **Offline**: Works without internet connection
- ✅ **Free**: No API costs
- ✅ **Fast**: No network latency
- ✅ **Full Control**: You own the model

<details>
<summary>📦 Alternative: Cloud Options (Not Recommended for Privacy)</summary>

**Option B: Groq (Cloud, Free Tier, Fast)**
```bash
# ⚠️ Data sent to Groq servers
export GROQ_API_KEY='your-key-here'
ez --set-backend groq
```

**Option C: OpenAI (Cloud, Paid, High Quality)**
```bash
# ⚠️ Data sent to OpenAI servers
export OPENAI_API_KEY='your-key-here'
ez --set-backend openai
```

> **Privacy Note**: When using cloud backends, your query and system context are sent to third-party servers. For maximum privacy, always use Ollama.

</details>

### 3. Use It!

```bash
ez "list files sorted by size"
ez "show git branches"
ez "find all python files"
ez "check disk usage"
```

## 🌟 Examples

### Basic Usage

```bash
$ ez "find files modified today"
💡 Finds files in current directory modified within last 24 hours

find . -type f -mtime 0
```

### Git Commands

```bash
$ ez "show uncommitted changes"
# Ez detects you're in a git repo
# Sees your current branch: feature/new-ui
# Knows you have 3 uncommitted files

💡 Shows detailed view of uncommitted changes including diffs

git diff --stat
```

### Docker Commands

```bash
$ ez "stop all containers"
# Ez detects Docker is running
# Sees you have 3 running containers
# Knows about your docker-compose.yml

💡 Stops all currently running Docker containers

docker stop $(docker ps -q)
```

### NPM/Node Projects

```bash
$ ez "run the tests"
# Ez reads your package.json
# Finds available scripts: test, build, start
# Sees you're using Node v20.9.0

💡 Runs the test script defined in package.json

npm test
```

### Shell Aliases

```bash
$ ez "what's my git status alias"
# Ez reads your .zshrc
# Finds: alias gs='git status'

💡 Your git status alias is 'gs'

alias gs
```

### Python Projects

```bash
$ ez "activate my virtual environment"
# Ez detects Python 3.11
# Sees requirements.txt and venv/ folder
# Knows virtual environment is not active

💡 Activates the Python virtual environment

source venv/bin/activate
```

## 📚 Documentation

### User Documentation

- [Installation Guide](docs/INSTALLATION.md) - Detailed installation instructions
- [Quick Start](docs/QUICKSTART.md) - Get started in 5 minutes
- [Usage Guide](docs/USAGE.md) - Comprehensive usage examples
- [Agentic Context](docs/AGENTIC_CONTEXT.md) - How intelligent context works
- [OS Detection](docs/OS_DETECTION.md) - OS-specific command generation

### Developer Documentation

- [Developer Guide](docs/DEVELOPER_GUIDE.md) - **Start here for contributing!**
- [Architecture](docs/ARCHITECTURE.md) - System design and architecture
- [Testing Guide](docs/TESTING.md) - How to test your changes
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community standards

## 🔧 Advanced Usage

### Interactive Mode

```bash
$ ez
ez - terminal assistant (interactive mode)
Backend: groq | Use Ctrl+D or 'exit' to quit

ez> find large files
💡 Finds files larger than 100MB...

ez> show git log
💡 Shows recent git commits...

ez> exit
Goodbye!
```

### Pipe Input

```bash
echo "compress this directory" | ez
```

### Override Backend/Model

```bash
ez -b ollama -m qwen2.5-coder:latest "list processes"
```

### List Available Options

```bash
ez --list-backends    # Show available backends
ez --list-models      # Show available models
ez --help             # Show all options
```

### Update

```bash
ez --update  # Update to latest version
```

## 🎨 Intelligent Features

### Agentic Context System

Ez doesn't just generate commands - it **understands your environment**:

| Query Mentions | Ez Fetches |
|----------------|------------|
| `git`, `branch`, `commit` | Current branch, remotes, uncommitted changes |
| `docker`, `container` | Running containers, docker-compose files |
| `npm`, `node`, `package` | Node version, package.json, available scripts |
| `python`, `pip`, `venv` | Python version, virtual env status, requirements |
| `alias`, `function` | Your custom shell aliases and functions |
| `cargo`, `rust` | Rust version, Cargo.toml, package info |
| `kubectl`, `k8s` | Kubernetes context, namespace |

[Learn more about agentic context →](docs/AGENTIC_CONTEXT.md)

### OS-Specific Commands

Ez generates commands specific to your operating system:

**Linux** (GNU tools):
```bash
$ ez "list processes by memory"
ps aux --sort=-%mem | head -20
```

**macOS** (BSD tools):
```bash
$ ez "list processes by memory"
ps aux -m | head -20
```

**Windows** (PowerShell):
```bash
$ ez "list processes by memory"
Get-Process | Sort-Object -Property WS -Descending | Select-Object -First 20
```

[Learn more about OS detection →](docs/OS_DETECTION.md)

## 🔒 Privacy & Security

### Our Privacy Commitment

**Ez is designed for privacy from the ground up:**

1. **Local-First Architecture**: Built to work entirely offline with Ollama
2. **No Telemetry**: Zero tracking, analytics, or usage data collection
3. **No Network Calls**: When using Ollama, no external network requests
4. **Local Context Only**: All system scanning happens on your machine
5. **Open Source**: Every line of code is auditable

### What Data Gets Processed?

When you run a command like `ez "find large files"`:

**Locally Collected (Never Sent Anywhere)**:
- Your OS and architecture
- Installed tools in your PATH
- Command history statistics (frequency only)

**Sent to LLM (Only With Ollama: Stays Local)**:
- Your natural language query
- Relevant system context (OS, tools)
- Project-specific info if detected (git branch, package.json, etc.)

**Never Collected or Sent**:
- File contents
- Passwords or API keys
- Personal data
- Command execution results
- Directory paths (unless relevant to query)

### Privacy Comparison

| Feature | Ollama (Local) | Cloud APIs |
|---------|----------------|------------|
| Data leaves machine | ❌ Never | ✅ Yes |
| Requires internet | ❌ No | ✅ Yes |
| Third-party access | ❌ No | ✅ Yes |
| Audit logs | ✅ Your control | ⚠️ Provider's logs |
| Cost | ✅ Free | 💰 Paid/Limited |

**Recommendation**: Always use Ollama for maximum privacy.

## 🛠️ Configuration

### Config File

Located at `~/.config/ez-term/config.toml`:

```toml
# Privacy-first configuration (recommended)
backend = "ollama"
model = "qwen2.5-coder:latest"
ollama_url = "http://localhost:11434"

# Only add these if using cloud providers:
# groq_api_key = "your-key"
# openai_api_key = "your-key"
```

### Environment Variables

```bash
# For local Ollama setup (recommended)
export OLLAMA_HOST="http://localhost:11434"

# Only if using cloud providers
# export GROQ_API_KEY="your-key"
# export OPENAI_API_KEY="your-key"
```

### Priority

1. Command-line flags (highest)
2. Environment variables
3. Config file
4. Defaults: **Ollama on localhost** (most private)

## 🤝 Contributing

We love contributions! Ez is designed to be easy to extend and improve.

### Quick Contribution Guide

1. **Fork & Clone**: Fork the repo and clone it locally
2. **Set Up**: Run `./dev-install.sh` to set up development environment
3. **Make Changes**: Create a feature branch and make your changes
4. **Test**: Run `cargo test` to ensure tests pass
5. **Submit**: Create a Pull Request with a clear description

### What to Contribute

- 🐛 **Bug Fixes**: Found a bug? Fix it!
- ✨ **New Features**: Want agentic detection for a new tool? Add it!
- 📝 **Documentation**: Improve docs, add examples
- 🧪 **Tests**: Add more test coverage
- 🎨 **Code Quality**: Refactoring, optimizations

### Getting Started

1. Read the [Developer Guide](docs/DEVELOPER_GUIDE.md)
2. Check [good first issues](https://github.com/ezcorp-org/ez-term/labels/good%20first%20issue)
3. Read [CONTRIBUTING.md](CONTRIBUTING.md)
4. Join discussions and ask questions!

## 📊 Project Stats

- **85 tests** across unit, integration, and E2E
- **90%+ code coverage** on core modules
- **8 intelligent context detectors** (Git, Docker, NPM, Python, Shell, Cargo, K8s, more)
- **3 LLM backends** supported (Ollama, Groq, OpenAI)
- **Support for 3 major OSes** (Linux, macOS, Windows)

## 🗺️ Roadmap

### v0.4.0 (Next)
- [ ] Linux distribution detection (Ubuntu, Fedora, Arch)
- [ ] Container/VM detection (Docker, WSL)
- [ ] More shell support (fish, PowerShell native)
- [ ] Command history learning
- [ ] Response caching

### v0.5.0
- [ ] Plugin system for custom detectors
- [ ] Streaming mode with progress
- [ ] Multi-step command generation
- [ ] Custom prompt templates

### Long Term
- [ ] GUI/TUI interface
- [ ] Command execution with safety checks
- [ ] Integration with more tools (terraform, ansible)
- [ ] Local model fine-tuning

## 🙏 Acknowledgments

### Built With Privacy-First Tools

- [Rust](https://www.rust-lang.org/) - Memory-safe systems programming
- [Ollama](https://ollama.ai/) - Local LLM runtime (our recommended backend)
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [tokio](https://tokio.rs/) - Async runtime
- [serde](https://serde.rs/) - Serialization framework

### Our Philosophy

Inspired by tools like GitHub Copilot CLI, but with a different mission:

**Ez is built for users who value:**
- 🔒 **Privacy**: Your code and commands stay on your machine
- 🛡️ **Safety**: Preview before executing, refuse dangerous commands
- 🌐 **Offline**: Works without internet using Ollama
- 🔓 **Open Source**: Fully auditable, no hidden behavior
- 🎯 **Extensibility**: Easy to add new features and detectors

**We believe AI assistance shouldn't require sacrificing privacy.**

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🔗 Links

- **GitHub**: [https://github.com/ezcorp-org/ez-term](https://github.com/ezcorp-org/ez-term)
- **Issues**: [Report a bug](https://github.com/ezcorp-org/ez-term/issues/new?template=bug_report.md)
- **Feature Requests**: [Suggest a feature](https://github.com/ezcorp-org/ez-term/issues/new?template=feature_request.md)
- **Discussions**: [Ask questions, share ideas](https://github.com/ezcorp-org/ez-term/discussions)

## ⭐ Star History

If you find ez useful, please consider starring the repository!

---

Made with ❤️ by the EZ Corp community. Happy commanding! 🚀
