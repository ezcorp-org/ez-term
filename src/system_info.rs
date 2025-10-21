use sysinfo::System;

pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub shell: Option<String>,
    pub total_memory: u64,
    pub available_memory: u64,
}

impl SystemInfo {
    pub fn detect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();
        let shell = std::env::var("SHELL").ok()
            .and_then(|s| s.split('/').last().map(|s| s.to_string()));

        Self {
            os,
            arch,
            shell,
            total_memory: sys.total_memory(),
            available_memory: sys.available_memory(),
        }
    }

    pub fn format_context(&self) -> String {
        let mut context = format!(
            "Operating System: {} ({})",
            self.os, self.arch
        );

        if let Some(shell) = &self.shell {
            context.push_str(&format!("\nShell: {}", shell));
        }

        context.push_str(&format!(
            "\nMemory: {:.1} GB total, {:.1} GB available",
            self.total_memory as f64 / 1_073_741_824.0,
            self.available_memory as f64 / 1_073_741_824.0
        ));

        // Add OS-specific command guidance
        let os_guidance = match self.os.as_str() {
            "linux" => "\nOS Commands: Use Linux/GNU commands (apt/dnf for packages, systemctl for services, /proc for system info)",
            "macos" => "\nOS Commands: Use macOS/BSD commands (brew for packages, launchctl for services, prefer BSD-style flags)",
            "windows" => "\nOS Commands: Use Windows commands (PowerShell preferred, or cmd.exe built-ins)",
            _ => "",
        };
        context.push_str(os_guidance);

        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_detect() {
        let info = SystemInfo::detect();

        // OS should be one of the supported platforms
        assert!(matches!(
            info.os.as_str(),
            "linux" | "macos" | "windows"
        ));

        // Architecture should be valid
        assert!(!info.arch.is_empty());
        assert!(matches!(
            info.arch.as_str(),
            "x86_64" | "aarch64" | "arm" | "x86"
        ));

        // Memory should be positive
        assert!(info.total_memory > 0);
        assert!(info.available_memory > 0);
        assert!(info.available_memory <= info.total_memory);
    }

    #[test]
    fn test_format_context() {
        let info = SystemInfo {
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
            shell: Some("zsh".to_string()),
            total_memory: 16_000_000_000,
            available_memory: 8_000_000_000,
        };

        let context = info.format_context();

        assert!(context.contains("Operating System: linux"));
        assert!(context.contains("x86_64"));
        assert!(context.contains("Shell: zsh"));
        assert!(context.contains("Memory"));
        assert!(context.contains("OS Commands: Use Linux/GNU commands"));
    }

    #[test]
    fn test_format_context_no_shell() {
        let info = SystemInfo {
            os: "macos".to_string(),
            arch: "aarch64".to_string(),
            shell: None,
            total_memory: 16_000_000_000,
            available_memory: 12_000_000_000,
        };

        let context = info.format_context();

        assert!(context.contains("Operating System: macos"));
        assert!(context.contains("aarch64"));
        assert!(!context.contains("Shell:"));
        assert!(context.contains("Memory"));
        assert!(context.contains("OS Commands: Use macOS/BSD commands"));
    }

    #[test]
    fn test_format_context_windows() {
        let info = SystemInfo {
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            shell: Some("powershell".to_string()),
            total_memory: 32_000_000_000,
            available_memory: 16_000_000_000,
        };

        let context = info.format_context();

        assert!(context.contains("Operating System: windows"));
        assert!(context.contains("x86_64"));
        assert!(context.contains("Shell: powershell"));
        assert!(context.contains("Memory"));
        assert!(context.contains("OS Commands: Use Windows commands"));
    }
}
