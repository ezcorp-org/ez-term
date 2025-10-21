# Agentic Context System

Ez features an intelligent **agentic context system** that automatically detects what tools and configurations are relevant to your query and injects that information into the LLM prompt.

## Overview

When you ask ez to generate a command, it doesn't just use generic system information. Instead, it intelligently analyzes your query and fetches relevant context from:

- Git repositories
- Docker environment
- NPM/Node.js projects
- Python environments
- Shell configuration files (aliases, functions, exports)
- Rust/Cargo projects
- Kubernetes clusters

## How It Works

### 1. Query Analysis

The context agent scans your query for keywords:

```rust
"show me git branches"  → Fetches git context
"list docker containers" → Fetches docker context
"run npm test"          → Fetches npm context
"check my aliases"      → Fetches shell RC files
```

### 2. Context Fetching

Based on detected keywords, the agent fetches relevant information:

#### Git Context
```
Current branch: main
Git user: John Doe
Remotes:
  origin  git@github.com:user/repo.git (fetch)
  origin  git@github.com:user/repo.git (push)
Uncommitted changes: 3 files
```

#### Docker Context
```
Docker version: Docker version 24.0.6
Running containers: 2
Found: docker-compose.yml in current directory
```

#### NPM Context
```
Node version: v20.9.0
NPM version: 10.1.0
Found: package.json in current directory
Package name: my-app
Available scripts: test build start dev lint
```

#### Python Context
```
Python version: Python 3.11.5
Virtual environment: ACTIVE
Found: requirements.txt
Found: pyproject.toml
```

#### Shell RC Context
```
Aliases from .zshrc:
  alias ll='ls -la'
  alias gs='git status'
  alias gp='git push'

Functions from .zshrc:
  myfunc() {
```

#### Cargo Context
```
Rust version: rustc 1.73.0
Cargo version: cargo 1.73.0
Found: Cargo.toml in current directory
Package name: ez-cli
```

#### Kubernetes Context
```
Kubectl version: Client Version: v1.28.0
Current context: my-cluster
Current namespace: default
```

## Examples

### Git Example

**Query**: "show me uncommitted changes"

**Context Injected**:
```
## Git Context
Current branch: feature/new-ui
Git user: Jane Developer
Remotes:
  origin  git@github.com:company/project.git (fetch)
  origin  git@github.com:company/project.git (push)
Uncommitted changes: 5 files
```

**Generated Command**:
```bash
git status --short
```

### Docker Example

**Query**: "stop all running containers"

**Context Injected**:
```
## Docker Context
Docker version: Docker version 24.0.6
Running containers: 3
Found: docker-compose.yml in current directory
```

**Generated Command**:
```bash
docker stop $(docker ps -q)
```

### NPM Example

**Query**: "run the tests"

**Context Injected**:
```
## NPM/Node Context
Node version: v20.9.0
NPM version: 10.1.0
Found: package.json in current directory
Package name: my-webapp
Available scripts: test build start dev lint
```

**Generated Command**:
```bash
npm test
```

### Shell Alias Example

**Query**: "what's my git status alias"

**Context Injected**:
```
## Shell Configuration
Aliases from .zshrc:
  alias gs='git status'
  alias gp='git push'
  alias gl='git log --oneline'
```

**Generated Command**:
```bash
alias gs
# Or suggests: Use the alias 'gs' which maps to 'git status'
```

## Keyword Triggers

The agent activates for these keyword patterns:

| Tool | Keywords |
|------|----------|
| Git | git, repository, repo, commit, branch, clone, merge, pull, push |
| Docker | docker, container, image, compose, dockerfile |
| NPM | npm, node, package.json, javascript, yarn, pnpm |
| Python | python, pip, virtualenv, venv, conda, requirements |
| Shell | alias, function, shell, bashrc, zshrc, export |
| Rust | rust, cargo, crate, rustc |
| Kubernetes | k8s, kubernetes, kubectl, pod, deployment, service |

## Configuration Files Scanned

### Shell RC Files
- `~/.zshrc` (zsh)
- `~/.zshenv` (zsh)
- `~/.bashrc` (bash)
- `~/.bash_profile` (bash)
- `~/.config/fish/config.fish` (fish)

### Project Files
- `package.json` (Node.js)
- `Cargo.toml` (Rust)
- `requirements.txt` (Python)
- `Pipfile` (Python/Pipenv)
- `pyproject.toml` (Python)
- `docker-compose.yml` (Docker)
- `compose.yml` (Docker)

## Implementation Details

### Context Agent Module

Located in: `src/context_agent.rs`

```rust
pub struct ContextAgent {
    home_dir: PathBuf,
    shell: Option<String>,
}

impl ContextAgent {
    /// Analyze user query and fetch relevant context
    pub fn get_relevant_context(&self, query: &str) -> String {
        // Detect what tools are mentioned
        let needs_git = self.mentions_tool(query, &["git", "repository"]);

        // Fetch relevant contexts
        if needs_git {
            if let Some(git_context) = self.get_git_context() {
                // ... inject context
            }
        }
    }
}
```

### Integration

The context agent is integrated into the main query processing flow:

```rust
async fn process_query(
    query: &str,
    client: &LLMClient,
    context: &str,
    agent: &ContextAgent,
) -> Result<()> {
    // Get agentic context based on the query
    let agentic_context = agent.get_relevant_context(query);

    // Combine base context with agentic context
    let full_context = format!("{}\n{}", context, agentic_context);

    // Generate command with full context
    let response = client.generate_and_collect(query, &full_context).await?;
}
```

## Benefits

### 1. **Context-Aware Commands**
Commands are generated with full knowledge of your environment:
- Current git branch and status
- Running Docker containers
- Available NPM scripts
- Active Python virtual environment
- Custom shell aliases

### 2. **Reduced Errors**
The LLM knows:
- Which tools are actually installed
- Which versions you're using
- Which configuration files exist
- What commands you already have aliased

### 3. **Smarter Suggestions**
Instead of generic commands, you get commands that:
- Use your existing aliases
- Work with your specific setup
- Match your project structure
- Leverage your custom configurations

### 4. **Automatic Adaptation**
The system automatically adapts to:
- Different projects (NPM vs Cargo vs Python)
- Different shells (bash vs zsh)
- Different environments (Docker vs native)
- Different workflows (git flow, custom scripts)

## Privacy & Security

### What's Collected
- Git branch names, user name, remote URLs
- Docker container counts
- Package/project names from config files
- Shell aliases and function names (not content)
- Tool versions

### What's NOT Collected
- File contents (except config files)
- Credentials or API keys
- Private data
- Command history details

### Local Only
All context fetching happens locally. Nothing is sent anywhere except to your configured LLM backend as part of the prompt.

## Future Enhancements

### Planned Features
- [ ] Cache frequently used context
- [ ] Learn from user's command patterns
- [ ] Suggest custom aliases based on usage
- [ ] Detect more project types (Go, Java, Ruby)
- [ ] Integration with more tools (terraform, ansible)
- [ ] Smart context prioritization (most relevant first)
- [ ] User-configurable keyword triggers

### Example Future Capability
```bash
# Ez learns you always use "yarn" instead of "npm"
$ ez "install dependencies"

# Detects package.json has "yarn.lock"
# Generated: yarn install

# Not: npm install
```

## Troubleshooting

### Context Not Detected

If the agent doesn't detect your context:

1. **Check keywords**: Make sure your query mentions the tool
   ```bash
   # Instead of: "install packages"
   # Try: "npm install packages"
   ```

2. **Verify tools are installed**:
   ```bash
   which git
   which docker
   which node
   ```

3. **Check current directory**:
   ```bash
   # For project-specific context, run from project root
   cd ~/my-project
   ez "run tests"
   ```

### Too Much Context

If you're getting irrelevant context:

1. **Be specific in query**:
   ```bash
   # Instead of: "help with my code"
   # Try: "run python tests"
   ```

2. **The agent only fetches context for mentioned tools**
   - Mentioning "git" won't fetch Docker context
   - Mentioning "npm" won't fetch Python context

## Testing

Test the context agent:

```bash
# Test Git context
cd ~/your-git-repo
ez "show me current branch"

# Test Docker context
ez "list running containers"

# Test NPM context
cd ~/your-npm-project
ez "run the build script"

# Test shell context
ez "what are my git aliases"
```

## API Reference

### ContextAgent

```rust
impl ContextAgent {
    /// Create new context agent
    pub fn new() -> Result<Self>

    /// Get relevant context for a query
    pub fn get_relevant_context(&self, query: &str) -> String

    /// Check if query mentions tool keywords
    fn mentions_tool(&self, query: &str, keywords: &[&str]) -> bool

    /// Get Git repository context
    fn get_git_context(&self) -> Option<String>

    /// Get Docker environment context
    fn get_docker_context(&self) -> Option<String>

    /// Get NPM/Node.js project context
    fn get_npm_context(&self) -> Option<String>

    /// Get Python environment context
    fn get_python_context(&self) -> Option<String>

    /// Get shell RC file context
    fn get_shell_rc_context(&self) -> Option<String>

    /// Get Rust/Cargo project context
    fn get_cargo_context(&self) -> Option<String>

    /// Get Kubernetes cluster context
    fn get_kubernetes_context(&self) -> Option<String>
}
```

## Contributing

Want to add support for more tools? See [CONTRIBUTING.md](../CONTRIBUTING.md).

Example: Adding Go support

```rust
fn get_go_context(&self) -> Option<String> {
    let mut context = String::new();

    // Check Go version
    if let Ok(output) = Command::new("go")
        .args(&["version"])
        .output()
    {
        if output.status.success() {
            // ... parse and add to context
        }
    }

    // Check for go.mod
    if PathBuf::from("go.mod").exists() {
        context.push_str("\nFound: go.mod");
    }

    Some(context)
}
```

## Related Documentation

- [OS_DETECTION.md](./OS_DETECTION.md) - OS-specific context
- [TESTING.md](./TESTING.md) - Testing guide
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture

## Questions?

Open an issue with questions about the agentic context system!
