use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Agentic context builder that intelligently fetches relevant information
/// based on user queries
pub struct ContextAgent {
    home_dir: PathBuf,
    shell: Option<String>,
}

impl ContextAgent {
    pub fn new() -> Result<Self> {
        let home_dir = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"));

        let shell = std::env::var("SHELL")
            .ok()
            .and_then(|s| s.split('/').last().map(|s| s.to_string()));

        Ok(Self { home_dir, shell })
    }

    /// Analyze user query and fetch relevant context
    pub fn get_relevant_context(&self, query: &str) -> String {
        let mut context = String::new();
        let query_lower = query.to_lowercase();

        // Detect what the user is asking about
        let needs_git = self.mentions_tool(&query_lower, &["git", "repository", "repo", "commit", "branch"]);
        let needs_docker = self.mentions_tool(&query_lower, &["docker", "container", "image"]);
        let needs_npm = self.mentions_tool(&query_lower, &["npm", "node", "package.json", "javascript"]);
        let needs_python = self.mentions_tool(&query_lower, &["python", "pip", "virtualenv", "venv", "conda"]);
        let needs_shell = self.mentions_tool(&query_lower, &["alias", "function", "shell", "bashrc", "zshrc"]);
        let needs_cargo = self.mentions_tool(&query_lower, &["rust", "cargo", "crate"]);
        let needs_kubernetes = self.mentions_tool(&query_lower, &["k8s", "kubernetes", "kubectl", "pod", "deployment"]);

        // Fetch relevant contexts
        if needs_git {
            if let Some(git_context) = self.get_git_context() {
                context.push_str("\n## Git Context\n");
                context.push_str(&git_context);
            }
        }

        if needs_docker {
            if let Some(docker_context) = self.get_docker_context() {
                context.push_str("\n## Docker Context\n");
                context.push_str(&docker_context);
            }
        }

        if needs_npm {
            if let Some(npm_context) = self.get_npm_context() {
                context.push_str("\n## NPM/Node Context\n");
                context.push_str(&npm_context);
            }
        }

        if needs_python {
            if let Some(python_context) = self.get_python_context() {
                context.push_str("\n## Python Context\n");
                context.push_str(&python_context);
            }
        }

        if needs_shell {
            if let Some(shell_context) = self.get_shell_rc_context() {
                context.push_str("\n## Shell Configuration\n");
                context.push_str(&shell_context);
            }
        }

        if needs_cargo {
            if let Some(cargo_context) = self.get_cargo_context() {
                context.push_str("\n## Rust/Cargo Context\n");
                context.push_str(&cargo_context);
            }
        }

        if needs_kubernetes {
            if let Some(k8s_context) = self.get_kubernetes_context() {
                context.push_str("\n## Kubernetes Context\n");
                context.push_str(&k8s_context);
            }
        }

        context
    }

    /// Check if query mentions any of the given keywords
    fn mentions_tool(&self, query: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|kw| query.contains(kw))
    }

    /// Get Git repository context
    fn get_git_context(&self) -> Option<String> {
        let mut context = String::new();

        // Check if we're in a git repo
        if let Ok(output) = Command::new("git")
            .args(&["rev-parse", "--is-inside-work-tree"])
            .output()
        {
            if !output.status.success() {
                return None;
            }
        } else {
            return None;
        }

        // Get current branch
        if let Ok(output) = Command::new("git")
            .args(&["branch", "--show-current"])
            .output()
        {
            if output.status.success() {
                if let Ok(branch) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Current branch: {}", branch.trim()));
                }
            }
        }

        // Get git config
        if let Ok(output) = Command::new("git")
            .args(&["config", "--get", "user.name"])
            .output()
        {
            if output.status.success() {
                if let Ok(name) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("\nGit user: {}", name.trim()));
                }
            }
        }

        // Get remote info
        if let Ok(output) = Command::new("git")
            .args(&["remote", "-v"])
            .output()
        {
            if output.status.success() {
                if let Ok(remotes) = String::from_utf8(output.stdout) {
                    let remote_lines: Vec<&str> = remotes.lines().take(2).collect();
                    if !remote_lines.is_empty() {
                        context.push_str("\nRemotes:\n");
                        for line in remote_lines {
                            context.push_str(&format!("  {}\n", line));
                        }
                    }
                }
            }
        }

        // Get repo status
        if let Ok(output) = Command::new("git")
            .args(&["status", "--short"])
            .output()
        {
            if output.status.success() {
                if let Ok(status) = String::from_utf8(output.stdout) {
                    let file_count = status.lines().count();
                    if file_count > 0 {
                        context.push_str(&format!("\nUncommitted changes: {} files", file_count));
                    }
                }
            }
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }

    /// Get Docker context
    fn get_docker_context(&self) -> Option<String> {
        let mut context = String::new();

        // Check Docker version
        if let Ok(output) = Command::new("docker")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Docker version: {}", version.trim()));
                }
            }
        } else {
            return None;
        }

        // Get running containers count
        if let Ok(output) = Command::new("docker")
            .args(&["ps", "-q"])
            .output()
        {
            if output.status.success() {
                if let Ok(containers) = String::from_utf8(output.stdout) {
                    let count = containers.lines().count();
                    context.push_str(&format!("\nRunning containers: {}", count));
                }
            }
        }

        // Check for docker-compose.yml
        if PathBuf::from("docker-compose.yml").exists() {
            context.push_str("\nFound: docker-compose.yml in current directory");
        } else if PathBuf::from("compose.yml").exists() {
            context.push_str("\nFound: compose.yml in current directory");
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }

    /// Get NPM/Node context
    fn get_npm_context(&self) -> Option<String> {
        let mut context = String::new();

        // Check Node version
        if let Ok(output) = Command::new("node")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Node version: {}", version.trim()));
                }
            }
        }

        // Check NPM version
        if let Ok(output) = Command::new("npm")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("\nNPM version: {}", version.trim()));
                }
            }
        }

        // Check for package.json
        if PathBuf::from("package.json").exists() {
            context.push_str("\nFound: package.json in current directory");

            // Try to get package name
            if let Ok(content) = fs::read_to_string("package.json") {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(name) = json.get("name").and_then(|v| v.as_str()) {
                        context.push_str(&format!("\nPackage name: {}", name));
                    }

                    // Check for common scripts
                    if let Some(scripts) = json.get("scripts").and_then(|v| v.as_object()) {
                        context.push_str("\nAvailable scripts:");
                        for (key, _) in scripts.iter().take(5) {
                            context.push_str(&format!(" {}", key));
                        }
                    }
                }
            }
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }

    /// Get Python context
    fn get_python_context(&self) -> Option<String> {
        let mut context = String::new();

        // Check Python version
        if let Ok(output) = Command::new("python3")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Python version: {}", version.trim()));
                }
            }
        } else if let Ok(output) = Command::new("python")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Python version: {}", version.trim()));
                }
            }
        }

        // Check for virtual environment
        if std::env::var("VIRTUAL_ENV").is_ok() {
            context.push_str("\nVirtual environment: ACTIVE");
        }

        // Check for requirements.txt
        if PathBuf::from("requirements.txt").exists() {
            context.push_str("\nFound: requirements.txt");
        }

        // Check for Pipfile
        if PathBuf::from("Pipfile").exists() {
            context.push_str("\nFound: Pipfile (using pipenv)");
        }

        // Check for pyproject.toml
        if PathBuf::from("pyproject.toml").exists() {
            context.push_str("\nFound: pyproject.toml");
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }

    /// Get shell RC file context
    fn get_shell_rc_context(&self) -> Option<String> {
        let mut context = String::new();

        // Determine RC file based on shell
        let rc_files = match self.shell.as_deref() {
            Some("zsh") => vec![".zshrc", ".zshenv"],
            Some("bash") => vec![".bashrc", ".bash_profile"],
            Some("fish") => vec![".config/fish/config.fish"],
            _ => vec![".bashrc", ".zshrc"],
        };

        for rc_file in rc_files {
            let rc_path = self.home_dir.join(rc_file);
            if rc_path.exists() {
                if let Ok(content) = fs::read_to_string(&rc_path) {
                    // Extract aliases
                    let aliases: Vec<&str> = content
                        .lines()
                        .filter(|line| line.trim_start().starts_with("alias "))
                        .take(10)
                        .collect();

                    if !aliases.is_empty() {
                        context.push_str(&format!("\nAliases from {}:\n", rc_file));
                        for alias in aliases {
                            context.push_str(&format!("  {}\n", alias.trim()));
                        }
                    }

                    // Extract functions
                    let functions: Vec<&str> = content
                        .lines()
                        .filter(|line| {
                            let trimmed = line.trim();
                            trimmed.ends_with("() {") || trimmed.contains("function ")
                        })
                        .take(5)
                        .collect();

                    if !functions.is_empty() {
                        context.push_str(&format!("\nFunctions from {}:\n", rc_file));
                        for func in functions {
                            context.push_str(&format!("  {}\n", func.trim()));
                        }
                    }

                    // Extract exports
                    let exports: Vec<&str> = content
                        .lines()
                        .filter(|line| line.trim_start().starts_with("export "))
                        .take(10)
                        .collect();

                    if !exports.is_empty() {
                        context.push_str(&format!("\nExports from {}:\n", rc_file));
                        for export in exports {
                            context.push_str(&format!("  {}\n", export.trim()));
                        }
                    }
                }
            }
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }

    /// Get Rust/Cargo context
    fn get_cargo_context(&self) -> Option<String> {
        let mut context = String::new();

        // Check Rust version
        if let Ok(output) = Command::new("rustc")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Rust version: {}", version.trim()));
                }
            }
        }

        // Check Cargo version
        if let Ok(output) = Command::new("cargo")
            .args(&["--version"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("\nCargo version: {}", version.trim()));
                }
            }
        }

        // Check for Cargo.toml
        if PathBuf::from("Cargo.toml").exists() {
            context.push_str("\nFound: Cargo.toml in current directory");

            if let Ok(content) = fs::read_to_string("Cargo.toml") {
                if let Ok(toml) = content.parse::<toml::Value>() {
                    if let Some(package) = toml.get("package") {
                        if let Some(name) = package.get("name").and_then(|v| v.as_str()) {
                            context.push_str(&format!("\nPackage name: {}", name));
                        }
                    }
                }
            }
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }

    /// Get Kubernetes context
    fn get_kubernetes_context(&self) -> Option<String> {
        let mut context = String::new();

        // Check kubectl version
        if let Ok(output) = Command::new("kubectl")
            .args(&["version", "--client", "--short"])
            .output()
        {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("Kubectl version: {}", version.trim()));
                }
            }
        } else {
            return None;
        }

        // Get current context
        if let Ok(output) = Command::new("kubectl")
            .args(&["config", "current-context"])
            .output()
        {
            if output.status.success() {
                if let Ok(ctx) = String::from_utf8(output.stdout) {
                    context.push_str(&format!("\nCurrent context: {}", ctx.trim()));
                }
            }
        }

        // Get current namespace
        if let Ok(output) = Command::new("kubectl")
            .args(&["config", "view", "--minify", "--output", "jsonpath={..namespace}"])
            .output()
        {
            if output.status.success() {
                if let Ok(ns) = String::from_utf8(output.stdout) {
                    if !ns.is_empty() {
                        context.push_str(&format!("\nCurrent namespace: {}", ns.trim()));
                    }
                }
            }
        }

        if context.is_empty() {
            None
        } else {
            Some(context)
        }
    }
}

impl Default for ContextAgent {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
