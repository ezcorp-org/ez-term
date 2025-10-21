# OS Detection and Context Injection

This document explains how `ez` detects the user's operating system and injects OS-specific context into LLM prompts.

## Overview

`ez` automatically detects the user's operating system, architecture, shell, and installed tools. This information is injected into every LLM prompt to ensure generated commands are compatible with the user's system.

## System Information Collected

### 1. Operating System
- **Detection Method**: `std::env::consts::OS`
- **Values**: `linux`, `macos`, `windows`
- **Usage**: Determines OS-specific command syntax and available tools

### 2. Architecture
- **Detection Method**: `std::env::consts::ARCH`
- **Values**: `x86_64`, `aarch64`, `arm`, `x86`
- **Usage**: Architecture-specific optimizations and compatibility

### 3. Shell
- **Detection Method**: `$SHELL` environment variable
- **Values**: `bash`, `zsh`, `fish`, `powershell`, etc.
- **Usage**: Shell-specific command syntax and features

### 4. Memory Information
- **Detection Method**: `sysinfo` crate
- **Values**: Total and available memory in GB
- **Usage**: Memory-aware command suggestions

### 5. Installed Tools
- **Detection Method**: Scanning `$PATH` for common tools
- **Checked Tools**: git, docker, npm, node, python, cargo, rustc, go, java, kubectl, vim, nvim, code, curl, wget
- **Usage**: Only suggest commands using tools that are actually installed

### 6. Command History
- **Detection Method**: Parsing `~/.bash_history` and `~/.zsh_history`
- **Usage**: Suggests commands similar to user's frequent commands

## Context Injection

### System Context Format

```
Operating System: linux (x86_64)
Shell: zsh
Memory: 16.0 GB total, 8.5 GB available
OS Commands: Use Linux/GNU commands (apt/dnf for packages, systemctl for services, /proc for system info)
Installed tools: git, docker, npm, python3, cargo, vim
Frequently used commands: ls, cd, git, npm, docker
```

### Prompt Enhancement

The system context is injected into the LLM prompt with explicit instructions:

```
IMPORTANT: Generate commands specific to the user's OS (see system info below)

Command generation guidelines:
- **MATCH the user's OS**: Use OS-specific commands and flag syntax (Linux/macOS/Windows)
- **Linux**: Use GNU-style commands (apt/dnf, systemctl, --long-flags, /proc)
- **macOS**: Use BSD-style commands (brew, launchctl, single-dash flags, different flag syntax)
- **Windows**: Use PowerShell commands or cmd.exe built-ins

User's system information:
[SYSTEM CONTEXT HERE]

IMPORTANT: The user is on LINUX. Generate commands that work specifically on this OS!
```

## OS-Specific Command Examples

### Linux
```bash
# Process listing (GNU ps)
ps aux --sort=-%mem | head -20

# Package management
apt list --installed
dnf search package-name

# Service management
systemctl status nginx
systemctl list-units --type=service
```

### macOS
```bash
# Process listing (BSD ps)
ps aux -m | head -20

# Package management
brew list
brew search package-name

# Service management
launchctl list
launchctl print system/com.apple.example
```

### Windows
```powershell
# Process listing
Get-Process | Sort-Object -Property WS -Descending | Select-Object -First 20

# Package management
choco list --local-only

# Service management
Get-Service
Get-Service -Name 'ServiceName'
```

## Implementation Details

### Code Location

#### System Info Detection
File: `src/system_info.rs`

```rust
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub shell: Option<String>,
    pub total_memory: u64,
    pub available_memory: u64,
}

impl SystemInfo {
    pub fn detect() -> Self {
        // Detection logic
    }

    pub fn format_context(&self) -> String {
        // Format context with OS-specific guidance
    }
}
```

#### Tool Detection
File: `src/tool_detection.rs`

```rust
pub struct ToolDetection {
    pub installed_tools: Vec<String>,
    pub frequent_commands: Vec<(String, usize)>,
}

impl ToolDetection {
    pub fn detect() -> Self {
        // Scan PATH and analyze history
    }
}
```

#### Prompt Construction
File: `src/main.rs`

```rust
let system_info = SystemInfo::detect();
let tool_detection = ToolDetection::detect();

let system_context = format!(
    "System: {}\n{}",
    system_info.format_context(),
    tool_detection.format_context()
);

let context = format!(
    "You are an expert CLI command generator...\n\
    User's system information:\n{}\n\n\
    IMPORTANT: The user is on {}. Generate commands that work specifically on this OS!",
    system_context,
    system_info.os.to_uppercase()
);
```

## Testing

### Unit Tests
Located in: `src/system_info.rs`, `src/tool_detection.rs`

```rust
#[test]
fn test_system_info_detect() {
    let info = SystemInfo::detect();
    assert!(matches!(info.os.as_str(), "linux" | "macos" | "windows"));
}

#[test]
fn test_format_context() {
    let info = SystemInfo { /* ... */ };
    let context = info.format_context();
    assert!(context.contains("Operating System:"));
    assert!(context.contains("OS Commands:"));
}
```

### E2E Tests
Located in: `tests/e2e_command_generation.rs`

```rust
#[test]
fn test_e2e_system_context_included() {
    // Verifies OS info is included in LLM request
}

#[test]
fn test_e2e_os_specific_guidance_in_prompt() {
    // Verifies OS-specific guidance is in prompt
}
```

## Benefits

### 1. **OS Compatibility**
Commands are guaranteed to work on the user's OS:
- No Linux commands on Windows
- No macOS-specific commands on Linux
- Correct flag syntax for each OS

### 2. **Tool Availability**
Only suggests commands using installed tools:
- Won't suggest `docker` if Docker isn't installed
- Won't suggest `npm` if Node.js isn't installed

### 3. **User Preferences**
Respects user's shell and command history:
- Suggests commands similar to frequently used ones
- Uses shell-specific syntax when needed

### 4. **Safety**
Context-aware safety checks:
- Different safety rules for different OSes
- Platform-specific safe alternatives

## Future Enhancements

### Planned Improvements
- [ ] Detect distribution (Ubuntu, Fedora, Arch, etc.)
- [ ] Version detection for tools (git 2.x vs 3.x)
- [ ] Container/VM detection (Docker, WSL, etc.)
- [ ] More shell-specific optimizations
- [ ] Custom tool paths beyond $PATH
- [ ] User-defined tool preferences
- [ ] OS version detection (macOS 13, 14, etc.)

### Example: Distribution Detection
```rust
pub fn detect_linux_distro() -> Option<String> {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        // Parse ID field
        for line in contents.lines() {
            if line.starts_with("ID=") {
                return Some(line.trim_start_matches("ID=").to_string());
            }
        }
    }
    None
}
```

## Environment Variables

### Override System Detection

You can override detected values with environment variables:

```bash
# Force OS detection
export EZ_FORCE_OS=linux

# Override shell
export SHELL=/bin/zsh

# Custom PATH for tool detection
export PATH=/custom/bin:$PATH
```

## Debugging

### View System Context

Run with debug output to see detected context:

```bash
# Show system info
ez --debug "list files"

# Check detected tools
ez --show-context
```

### Test Different OS Contexts

Use test fixtures to simulate different OSes:

```rust
#[test]
fn test_macos_context() {
    let info = SystemInfo {
        os: "macos".to_string(),
        arch: "aarch64".to_string(),
        shell: Some("zsh".to_string()),
        // ...
    };
    // Test macOS-specific behavior
}
```

## Related Documentation

- [TESTING.md](./TESTING.md) - Testing strategy
- [ARCHITECTURE.md](./ARCHITECTURE.md) - System architecture
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contributing guidelines

## Questions?

Open an issue if you have questions about OS detection or want to suggest improvements!
