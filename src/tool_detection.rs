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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_creates_tool_detection() {
        let detection = ToolDetection::detect();
        // Should at least have some structure
        assert!(detection.installed_tools.len() >= 0);
        assert!(detection.frequent_commands.len() >= 0);
    }

    #[test]
    fn test_scan_path_empty_when_no_path() {
        // Temporarily unset PATH
        let original_path = std::env::var("PATH").ok();
        std::env::remove_var("PATH");

        let tools = ToolDetection::scan_path();
        assert_eq!(tools.len(), 0);

        // Restore PATH
        if let Some(path) = original_path {
            std::env::set_var("PATH", path);
        }
    }

    #[test]
    fn test_format_context_with_tools() {
        let detection = ToolDetection {
            installed_tools: vec!["git".to_string(), "docker".to_string(), "npm".to_string()],
            frequent_commands: vec![
                ("ls".to_string(), 100),
                ("cd".to_string(), 80),
                ("git".to_string(), 60),
            ],
        };

        let context = detection.format_context();
        assert!(context.contains("Installed tools: git, docker, npm"));
        assert!(context.contains("Frequently used commands: ls, cd, git"));
    }

    #[test]
    fn test_format_context_empty_tools() {
        let detection = ToolDetection {
            installed_tools: vec![],
            frequent_commands: vec![],
        };

        let context = detection.format_context();
        assert_eq!(context, "");
    }

    #[test]
    fn test_format_context_only_tools() {
        let detection = ToolDetection {
            installed_tools: vec!["git".to_string(), "docker".to_string()],
            frequent_commands: vec![],
        };

        let context = detection.format_context();
        assert!(context.contains("Installed tools: git, docker"));
        assert!(!context.contains("Frequently used commands"));
    }

    #[test]
    fn test_format_context_only_commands() {
        let detection = ToolDetection {
            installed_tools: vec![],
            frequent_commands: vec![
                ("ls".to_string(), 50),
                ("cd".to_string(), 30),
            ],
        };

        let context = detection.format_context();
        assert!(!context.contains("Installed tools"));
        assert!(context.contains("Frequently used commands: ls, cd"));
    }

    #[test]
    fn test_format_context_truncates_top_10_commands() {
        let detection = ToolDetection {
            installed_tools: vec![],
            frequent_commands: vec![
                ("cmd1".to_string(), 100),
                ("cmd2".to_string(), 90),
                ("cmd3".to_string(), 80),
                ("cmd4".to_string(), 70),
                ("cmd5".to_string(), 60),
                ("cmd6".to_string(), 50),
                ("cmd7".to_string(), 40),
                ("cmd8".to_string(), 30),
                ("cmd9".to_string(), 20),
                ("cmd10".to_string(), 10),
                ("cmd11".to_string(), 5), // Should not appear
            ],
        };

        let context = detection.format_context();
        assert!(context.contains("cmd1"));
        assert!(context.contains("cmd10"));
        assert!(!context.contains("cmd11")); // Should be truncated
    }

    #[test]
    fn test_analyze_history_with_bash_history() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());

        // Create fake bash history
        let bash_history = temp_dir.path().join(".bash_history");
        fs::write(
            &bash_history,
            "ls -la\ncd /tmp\nls\ngit status\nls\ngit commit\n",
        )
        .unwrap();

        let commands = ToolDetection::analyze_history();

        // ls should appear 3 times, so it should be first
        assert!(!commands.is_empty());
        let ls_count = commands.iter().find(|(cmd, _)| cmd == "ls");
        assert!(ls_count.is_some());
        assert_eq!(ls_count.unwrap().1, 3);
    }

    #[test]
    fn test_analyze_history_with_zsh_history() {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());

        // Create fake zsh history
        let zsh_history = temp_dir.path().join(".zsh_history");
        fs::write(
            &zsh_history,
            ": 1234567890:0;ls -la\n: 1234567891:0;cd /tmp\n: 1234567892:0;ls\n",
        )
        .unwrap();

        let commands = ToolDetection::analyze_history();

        assert!(!commands.is_empty());
        let ls_count = commands.iter().find(|(cmd, _)| cmd == "ls");
        assert!(ls_count.is_some());
        assert_eq!(ls_count.unwrap().1, 2);
    }

    #[test]
    fn test_analyze_history_empty_when_no_home() {
        // Temporarily unset HOME
        let original_home = std::env::var("HOME").ok();
        std::env::remove_var("HOME");

        let commands = ToolDetection::analyze_history();
        assert_eq!(commands.len(), 0);

        // Restore HOME
        if let Some(home) = original_home {
            std::env::set_var("HOME", home);
        }
    }
}
