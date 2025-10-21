# Release Process - ez-term

This document describes the automated release process for ez-term.

---

## Overview

ez-term uses **GitHub Actions** to automatically build, package, and release binaries for all major platforms when a new version tag is pushed.

---

## Supported Platforms

| Platform | Architecture | Binary Format |
|----------|-------------|---------------|
| **Linux** | x86_64 | `ez-linux-x86_64.tar.gz` |
| **Linux** | ARM64 (aarch64) | `ez-linux-aarch64.tar.gz` |
| **macOS** | x86_64 (Intel) | `ez-macos-x86_64.tar.gz` |
| **macOS** | ARM64 (Apple Silicon) | `ez-macos-aarch64.tar.gz` |
| **Windows** | x86_64 | `ez-windows-x86_64.exe.zip` |

---

## How to Create a Release

### 1. Update Version

Edit `Cargo.toml`:

```toml
[package]
version = "0.5.7"  # Bump to new version
```

### 2. Update CHANGELOG.md

Add release notes for the new version:

```markdown
## [0.5.7] - 2025-10-21

### Added
- New feature description

### Fixed
- Bug fix description
```

### 3. Commit Changes

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to v0.5.7"
git push origin main
```

### 4. Create and Push Tag

```bash
# Create annotated tag
git tag -a v0.5.7 -m "Release v0.5.7"

# Push tag to trigger release
git push origin v0.5.7
```

**That's it!** GitHub Actions will automatically:
1. Build binaries for all platforms
2. Generate SHA256 checksums
3. Create a GitHub Release
4. Upload all binaries and checksums

---

## Release Workflow

The release workflow is defined in `.github/workflows/release.yml`

### Build Job

For each platform:
1. **Checkout code**
2. **Install Rust** with the target platform
3. **Install cross-compilation tools** (for ARM64 Linux)
4. **Build binary** with `cargo build --release --target <platform>`
5. **Strip and compress**:
   - Unix: `tar czf ez-<platform>.tar.gz ez`
   - Windows: ZIP archive
6. **Upload artifacts**

### Release Job

After all builds complete:
1. **Download all artifacts**
2. **Generate checksums**: `sha256sum` for all binaries
3. **Create GitHub Release** with:
   - Auto-generated release notes
   - All platform binaries
   - `checksums.txt` file

---

## Installation URLs

### One-Line Installer

```bash
curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash
```

The installer (`scripts/install.sh`):
1. Detects platform and architecture
2. Fetches latest release from GitHub API
3. Downloads correct binary for the platform
4. Extracts and installs to `~/.local/bin/ez`
5. Sets up shell integration

### Download URLs

Latest release binaries are available at:

```
https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-linux-x86_64.tar.gz
https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-linux-aarch64.tar.gz
https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-macos-x86_64.tar.gz
https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-macos-aarch64.tar.gz
https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-windows-x86_64.exe.zip
```

### GitHub API

Latest release info:

```bash
curl -s https://api.github.com/repos/ezcorp-org/ez-term/releases/latest
```

---

## Website Integration

The website (`website/`) displays the one-line installer command:

**Location**: `website/src/lib/components/Hero.svelte`

```svelte
const installCommand = 'curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash';
```

**Also in**:
- `website/src/lib/components/QuickStart.svelte`
- `website/src/lib/components/HowItWorks.svelte`

All components use the correct URL: `ezcorp-org/ez-term`

---

## Verification

### After Creating a Release

1. **Check GitHub Release Page**:
   ```
   https://github.com/ezcorp-org/ez-term/releases
   ```

2. **Verify All Binaries Uploaded**:
   - [x] `ez-linux-x86_64.tar.gz`
   - [x] `ez-linux-aarch64.tar.gz`
   - [x] `ez-macos-x86_64.tar.gz`
   - [x] `ez-macos-aarch64.tar.gz`
   - [x] `ez-windows-x86_64.exe.zip`
   - [x] `checksums.txt`

3. **Test Installation**:
   ```bash
   curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash
   ```

4. **Verify Checksum**:
   ```bash
   # Download binary and checksum file
   curl -L -O https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-linux-x86_64.tar.gz
   curl -L -O https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/checksums.txt

   # Verify
   sha256sum -c checksums.txt --ignore-missing
   ```

---

## Troubleshooting

### Release Workflow Failed

1. **Check Actions tab**: `https://github.com/ezcorp-org/ez-term/actions`
2. **Review build logs** for the failed job
3. **Common issues**:
   - Cross-compilation tools not installed
   - Target not available
   - Cargo build error

### Binary Not Available

1. **Check if tag was pushed**: `git push origin v0.5.6`
2. **Verify workflow ran**: Check GitHub Actions
3. **Check workflow trigger**: Must be annotated tag starting with `v`

### Installer Fails

1. **Verify release exists**: Check GitHub releases page
2. **Check binary naming**: Must match `ez-<platform>.<ext>` pattern
3. **Test download URL manually**:
   ```bash
   curl -L -O https://github.com/ezcorp-org/ez-term/releases/download/v0.5.6/ez-linux-x86_64.tar.gz
   ```

---

## Manual Release (Fallback)

If automated release fails, you can create a release manually:

### 1. Build Locally

```bash
# Linux
cargo build --release
tar czf ez-linux-x86_64.tar.gz -C target/release ez

# macOS
cargo build --release
tar czf ez-macos-x86_64.tar.gz -C target/release ez
```

### 2. Generate Checksums

```bash
sha256sum ez-*.tar.gz > checksums.txt
```

### 3. Create GitHub Release

1. Go to: `https://github.com/ezcorp-org/ez-term/releases/new`
2. Choose tag: `v0.5.6`
3. Upload binaries and `checksums.txt`
4. Publish release

---

## Security

### Checksums

All releases include a `checksums.txt` file with SHA256 hashes:

```
<sha256>  ez-linux-aarch64/ez-linux-aarch64.tar.gz
<sha256>  ez-linux-x86_64/ez-linux-x86_64.tar.gz
<sha256>  ez-macos-aarch64/ez-macos-aarch64.tar.gz
<sha256>  ez-macos-x86_64/ez-macos-x86_64.tar.gz
<sha256>  ez-windows-x86_64.exe/ez-windows-x86_64.exe.zip
```

Users can verify downloads:

```bash
sha256sum -c checksums.txt --ignore-missing
```

### Binary Stripping

Unix binaries are stripped to:
- Reduce file size
- Remove debugging symbols
- Improve load time

---

## Platform-Specific Notes

### Linux ARM64 (aarch64)

Requires cross-compilation tools:
```bash
sudo apt-get install gcc-aarch64-linux-gnu
```

Already configured in workflow.

### macOS Universal Binary

Currently builds separate binaries for:
- x86_64 (Intel Macs)
- aarch64 (Apple Silicon)

Future: Could use `lipo` to create universal binary.

### Windows

- Binary is `.exe` file
- Packaged in `.zip` (not `.tar.gz`)
- No stripping applied

---

## Future Improvements

- [ ] Publish to crates.io automatically
- [ ] Create Homebrew tap
- [ ] Create Snap package
- [ ] Create AUR package
- [ ] Docker images
- [ ] Binary verification with GPG signatures
- [ ] Release notes auto-generation from CHANGELOG.md
- [ ] Automated version bumping
- [ ] Pre-release (beta) releases

---

## Links

- **GitHub Releases**: https://github.com/ezcorp-org/ez-term/releases
- **GitHub Actions**: https://github.com/ezcorp-org/ez-term/actions
- **Installation Script**: https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh
- **Latest Release API**: https://api.github.com/repos/ezcorp-org/ez-term/releases/latest

---

**Last Updated**: 2025-10-20
**Current Version**: 0.5.6
**Status**: ✅ Automated Release System Active
