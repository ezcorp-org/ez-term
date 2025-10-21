# Self-Update Feature

## 🔄 Update ez with One Command

```bash
ez --update
```

That's it! No manual downloads, no rebuilding, no hassle.

## How It Works

### 1. Check for Updates

```bash
$ ez --update
🔍 Checking for updates...

📦 Current version: v0.1.0
📦 Latest version:  v0.1.1

🆕 New version available: v0.1.1
```

### 2. Automatic Download

The update command:
- ✅ Detects your platform (Linux/macOS/Windows, x64/ARM64)
- ✅ Downloads the correct prebuilt binary
- ✅ Verifies the download
- ✅ Extracts the archive

### 3. Self-Replace

```bash
📦 Detected platform: linux-x86_64

📥 Downloading v0.1.1...
📦 Extracting...
📥 Installing update...

✅ Successfully updated to v0.1.1

🔄 Please restart your shell or run a new ez command to use the updated version.
```

The binary replaces itself automatically!

## Supported Platforms

- ✅ **Linux x86_64** (Ubuntu, Debian, Fedora, Arch, etc.)
- ✅ **Linux ARM64** (Raspberry Pi, AWS Graviton)
- ✅ **macOS Intel** (x86_64)
- ✅ **macOS Apple Silicon** (M1, M2, M3)
- ✅ **Windows x86_64**

## Examples

### Check if You're Up to Date

```bash
$ ez --update
🔍 Checking for updates...

📦 Current version: v0.1.1
📦 Latest version:  v0.1.1

✅ You are already running the latest version!
```

### Update from Older Version

```bash
$ ez --update
🔍 Checking for updates...

📦 Current version: v0.1.0
📦 Latest version:  v0.1.2

🆕 New version available: v0.1.2
📦 Detected platform: macos-aarch64

📥 Downloading v0.1.2...
📦 Extracting...
📥 Installing update...

✅ Successfully updated to v0.1.2
```

## Requirements

The update command requires:
- `curl` - for downloading files
- `tar` (Unix) or `unzip` (Windows) - for extracting archives
- Internet connection to GitHub

These are typically pre-installed on most systems.

## How It Works Internally

### 1. Version Check

```rust
let current_version = env!("CARGO_PKG_VERSION");
let latest_version = fetch_from_github_api();
```

Compares your version against the latest GitHub release.

### 2. Platform Detection

```rust
let os = env::consts::OS;      // "linux", "macos", "windows"
let arch = env::consts::ARCH;  // "x86_64", "aarch64"
```

Determines which binary to download.

### 3. Download URL Construction

```
https://github.com/ezcorp-org/ez-term/releases/download/v0.1.1/ez-{platform}.tar.gz
```

### 4. Self-Replace

On Unix systems:
```rust
fs::copy(new_binary, current_binary)?;
```

The running binary is replaced with the new one.

## Safety

### Backup Created

Before updating, a backup is created:
```
~/.local/bin/ez.bak
```

If something goes wrong, you can restore it manually.

### Verification

The update process:
- ✅ Checks GitHub API for latest release
- ✅ Downloads from official GitHub releases only
- ✅ Verifies download success
- ✅ Creates backup before replacing

### Permissions

On Unix, you might need write access to the binary location:
- If installed to `~/.local/bin` → No sudo needed ✅
- If installed to `/usr/local/bin` → Might need sudo ⚠️

## Troubleshooting

### "Download failed"

Check your internet connection and GitHub availability:
```bash
curl -I https://api.github.com/repos/ezcorp-org/ez-term/releases/latest
```

### "Failed to replace binary"

You might not have write permissions:
```bash
# Check where ez is installed
which ez

# Check permissions
ls -la $(which ez)

# If in /usr/local/bin, you might need sudo
sudo ez --update
```

### "Unsupported platform"

Your OS/architecture combination isn't supported yet.

Manual installation:
```bash
# Download manually from:
https://github.com/ezcorp-org/ez-term/releases

# Or build from source:
cargo install ez-cli
```

## Update Frequency

Check for updates:
- ✅ **Regularly** - Run `ez --update` periodically
- ✅ **After features** - When you hear about new features
- ✅ **For fixes** - When bugs are reported as fixed

## What's New in Each Version

Check the [Releases](https://github.com/ezcorp-org/ez-term/releases) page for:
- 📝 Changelog
- 🐛 Bug fixes
- ✨ New features
- ⚠️ Breaking changes

## Rollback

If an update causes issues:

```bash
# Restore backup
cp ~/.local/bin/ez.bak ~/.local/bin/ez

# Or download specific version
VERSION=v0.1.0
PLATFORM=linux-x86_64
curl -L -o ez.tar.gz https://github.com/ezcorp-org/ez-term/releases/download/$VERSION/ez-$PLATFORM.tar.gz
tar xzf ez.tar.gz
mv ez ~/.local/bin/
```

## Automatic Updates (Future)

Coming soon:
- ❌ Auto-check on startup (opt-in)
- ❌ Silent background updates
- ❌ Update notifications

Currently, updates are **manual and explicit** - you control when to update.

## Comparison with Other Tools

| Tool | Update Command | Needs Rebuild |
|------|----------------|---------------|
| **ez** | `ez --update` | ❌ No |
| cargo tools | `cargo install --force` | ✅ Yes (5-10 min) |
| homebrew | `brew upgrade ez` | ❌ No |
| apt/yum | `apt update && apt upgrade` | ❌ No |

`ez --update` combines the simplicity of package managers with the portability of a single binary!

## Summary

Stay updated with:
```bash
ez --update
```

- ✅ One command
- ✅ No rebuilding
- ✅ Works everywhere
- ✅ Automatic platform detection
- ✅ Safe (creates backup)

Keep your `ez` fresh! 🎉
