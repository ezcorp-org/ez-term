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
            "System: {} ({})",
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

        context
    }
}
