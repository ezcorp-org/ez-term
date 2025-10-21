# Contributing to ez

Thank you for your interest in contributing to `ez`! This document provides guidelines and information for contributors.

## 🎯 Ways to Contribute

- 🐛 **Bug Reports** - Help us identify and fix issues
- ✨ **Feature Requests** - Suggest new capabilities
- 📝 **Documentation** - Improve guides and examples
- 💻 **Code Contributions** - Submit bug fixes and features
- 🧪 **Testing** - Test on different platforms and configurations

## 🐛 Reporting Bugs

Before creating a bug report:
1. Check if the issue already exists in [Issues](https://github.com/ezcorp-org/ez-term/issues)
2. Make sure you're using the latest version (`ez --update`)

When reporting a bug, include:
- **Version**: Output of `ez --version`
- **Platform**: OS and architecture (`uname -a`)
- **Reproduction steps**: Clear steps to reproduce the issue
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Logs/Output**: Relevant error messages or command output

## ✨ Requesting Features

Feature requests are welcome! Please:
1. Check existing feature requests first
2. Describe the use case and why it's valuable
3. Provide examples of how it would work
4. Consider implementation complexity

## 💻 Development Setup

### Prerequisites

- Rust 1.70+ (`rustup` recommended)
- Git

### Clone and Build

```bash
# Clone repository
git clone https://github.com/ezcorp-org/ez-term.git
cd ez-term

# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run -- "your query here"
```

### Project Structure

```
ez-term/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument parsing
│   ├── config.rs         # Configuration management
│   ├── llm_client.rs     # LLM backend integration
│   ├── system_info.rs    # System detection
│   ├── tool_detection.rs # Tool usage analysis
│   └── update.rs         # Self-update functionality
├── scripts/
│   ├── install.sh        # Universal installer
│   ├── install-ez.sh     # Source build installer
│   ├── ez.sh             # Bash shell integration
│   └── ez.zsh            # Zsh shell integration
├── docs/                 # Documentation
├── .github/              # GitHub-specific files
└── openspec/             # OpenSpec specifications
```

## 📝 Code Guidelines

### Rust Style

Follow standard Rust conventions:
- Use `rustfmt` for formatting: `cargo fmt`
- Check with `clippy`: `cargo clippy`
- Write tests for new functionality
- Document public APIs with doc comments

### Commit Messages

Use clear, descriptive commit messages:

```
Add support for custom Ollama endpoints

- Allow users to specify custom Ollama URL
- Update configuration to persist endpoint
- Add --ollama-url CLI flag
```

Format:
- **First line**: Brief summary (50 chars or less)
- **Body**: Detailed explanation if needed
- **Footer**: Reference issues (`Fixes #123`)

### Pull Request Process

1. **Fork the repository**

2. **Create a feature branch**
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make your changes**
   - Write clean, documented code
   - Add tests if applicable
   - Update documentation

4. **Test thoroughly**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

5. **Commit with clear messages**
   ```bash
   git add .
   git commit -m "Add feature: description"
   ```

6. **Push to your fork**
   ```bash
   git push origin feature/my-new-feature
   ```

7. **Open a Pull Request**
   - Describe what and why
   - Link related issues
   - Explain testing done

### PR Requirements

- ✅ All tests pass
- ✅ Code is formatted (`cargo fmt`)
- ✅ No clippy warnings
- ✅ Documentation updated
- ✅ Clear commit messages

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test llm_client

# With output
cargo test -- --nocapture
```

### Manual Testing

Test on your platform:
```bash
# Build and test
cargo build --release

# Test basic functionality
./target/release/ez "find large files"

# Test shell integration
source scripts/ez.zsh
ez "show disk usage"

# Test update
./target/release/ez --update
```

## 📦 Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes
4. Create and push tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
5. GitHub Actions builds and releases automatically

## 🎨 Documentation

When updating documentation:
- Keep it concise and clear
- Include examples
- Update README if adding features
- Add to appropriate doc in `docs/`

## 🤝 Code of Conduct

Be respectful and constructive:
- Be welcoming to newcomers
- Be patient with questions
- Focus on what's best for the project
- Accept constructive criticism gracefully

## 💬 Getting Help

- **Questions**: Open a [Discussion](https://github.com/ezcorp-org/ez-term/discussions)
- **Bugs**: Open an [Issue](https://github.com/ezcorp-org/ez-term/issues)
- **Chat**: Comment on relevant issues/PRs

## 🏆 Recognition

Contributors will be:
- Listed in release notes
- Credited in commit history
- Appreciated in the community!

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Thank You

Every contribution, no matter how small, helps make `ez` better for everyone. Thank you for being part of this project!

---

**Questions?** Open an issue or discussion - we're here to help!
