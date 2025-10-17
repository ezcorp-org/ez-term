use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct ToolDetection {
    pub installed_tools: Vec<String>,
    pub frequent_commands: Vec<(String, usize)>,
}

impl ToolDetection {
    pub fn detect() -> Self {
        let installed_tools = Self::scan_path();
        let frequent_commands = Self::analyze_history();

        Self {
            installed_tools,
            frequent_commands,
        }
    }

    fn scan_path() -> Vec<String> {
        let path_var = match std::env::var("PATH") {
            Ok(p) => p,
            Err(_) => return Vec::new(),
        };

        let mut tools = Vec::new();
        let common_tools = [
            "git", "docker", "npm", "node", "python", "python3", "cargo",
            "rustc", "go", "java", "kubectl", "vim", "nvim", "code", "curl", "wget"
        ];

        for tool in &common_tools {
            for dir in path_var.split(':') {
                let tool_path = PathBuf::from(dir).join(tool);
                if tool_path.exists() {
                    tools.push(tool.to_string());
                    break;
                }
            }
        }

        tools
    }

    fn analyze_history() -> Vec<(String, usize)> {
        let mut command_counts: HashMap<String, usize> = HashMap::new();

        // Try bash history
        if let Ok(home) = std::env::var("HOME") {
            let bash_history = PathBuf::from(&home).join(".bash_history");
            if let Ok(contents) = fs::read_to_string(bash_history) {
                for line in contents.lines() {
                    if let Some(cmd) = line.split_whitespace().next() {
                        *command_counts.entry(cmd.to_string()).or_insert(0) += 1;
                    }
                }
            }

            // Try zsh history
            let zsh_history = PathBuf::from(&home).join(".zsh_history");
            if let Ok(contents) = fs::read_to_string(zsh_history) {
                for line in contents.lines() {
                    // zsh history format: : timestamp:0;command
                    let cmd_part = line.split(';').nth(1).unwrap_or(line);
                    if let Some(cmd) = cmd_part.split_whitespace().next() {
                        *command_counts.entry(cmd.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut counts: Vec<(String, usize)> = command_counts.into_iter().collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts.truncate(20); // Top 20 commands
        counts
    }

    pub fn format_context(&self) -> String {
        let mut context = String::new();

        if !self.installed_tools.is_empty() {
            context.push_str("Installed tools: ");
            context.push_str(&self.installed_tools.join(", "));
        }

        if !self.frequent_commands.is_empty() {
            context.push_str("\nFrequently used commands: ");
            let top_cmds: Vec<String> = self.frequent_commands
                .iter()
                .take(10)
                .map(|(cmd, _)| cmd.clone())
                .collect();
            context.push_str(&top_cmds.join(", "));
        }

        context
    }
}
