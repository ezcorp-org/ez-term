use anyhow::{anyhow, Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use crate::verification::{calculate_sha256, parse_checksums_file, get_checksum_for_file};

const GITHUB_REPO: &str = "ezcorp-org/ez-term";
const GITHUB_API_URL: &str = "https://api.github.com/repos/ezcorp-org/ez-term/releases/latest";

#[derive(Debug)]
struct Platform {
    name: &'static str,
    archive_ext: &'static str,
}

fn detect_platform() -> Result<Platform> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    match (os, arch) {
        ("linux", "x86_64") => Ok(Platform {
            name: "linux-x86_64",
            archive_ext: "tar.gz",
        }),
        ("linux", "aarch64") => Ok(Platform {
            name: "linux-aarch64",
            archive_ext: "tar.gz",
        }),
        ("macos", "x86_64") => Ok(Platform {
            name: "macos-x86_64",
            archive_ext: "tar.gz",
        }),
        ("macos", "aarch64") => Ok(Platform {
            name: "macos-aarch64",
            archive_ext: "tar.gz",
        }),
        ("windows", "x86_64") => Ok(Platform {
            name: "windows-x86_64",
            archive_ext: "zip",
        }),
        _ => Err(anyhow!(
            "Unsupported platform: {} {}. Please download manually from: https://github.com/{}/releases",
            os, arch, GITHUB_REPO
        )),
    }
}

fn get_current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn get_latest_version() -> Result<String> {
    let output = Command::new("curl")
        .args(["-s", GITHUB_API_URL])
        .output()
        .context("Failed to fetch latest release info. Is curl installed?")?;

    if !output.status.success() {
        return Err(anyhow!("Failed to fetch release information from GitHub"));
    }

    let response = String::from_utf8_lossy(&output.stdout);

    // Parse JSON to get tag_name
    for line in response.lines() {
        if line.contains("\"tag_name\"") {
            if let Some(version) = line.split('"').nth(3) {
                return Ok(version.trim_start_matches('v').to_string());
            }
        }
    }

    Err(anyhow!("Could not parse version from GitHub API response"))
}

fn get_current_binary_path() -> Result<PathBuf> {
    env::current_exe().context("Failed to get current executable path")
}

pub fn update() -> Result<()> {
    println!("🔍 Checking for updates...");
    println!();

    let current_version = get_current_version();
    let latest_version = get_latest_version()
        .context("Failed to check for updates. Please check your internet connection.")?;

    println!("📦 Current version: v{}", current_version);
    println!("📦 Latest version:  v{}", latest_version);
    println!();

    if current_version == latest_version {
        println!("✅ You are already running the latest version!");
        return Ok(());
    }

    println!("🆕 New version available: v{}", latest_version);
    println!();

    // Detect platform
    let platform = detect_platform()?;
    println!("📦 Detected platform: {}", platform.name);
    println!();

    // Construct download URL
    let download_url = format!(
        "https://github.com/{}/releases/download/v{}/ez-{}.{}",
        GITHUB_REPO, latest_version, platform.name, platform.archive_ext
    );

    // Create temp directory
    let tmp_dir = env::temp_dir().join(format!("ez-update-{}", latest_version));
    fs::create_dir_all(&tmp_dir).context("Failed to create temporary directory")?;

    let archive_path = tmp_dir.join(format!("ez-{}.{}", platform.name, platform.archive_ext));

    // Download new version
    println!("📥 Downloading v{}...", latest_version);
    let status = Command::new("curl")
        .args(["-L", "-o", archive_path.to_str().unwrap(), &download_url])
        .status()
        .context("Failed to download update. Is curl installed?")?;

    if !status.success() {
        return Err(anyhow!("Download failed. Please try again or download manually from: https://github.com/{}/releases", GITHUB_REPO));
    }

    // Validate file size
    let metadata = fs::metadata(&archive_path)
        .context("Failed to read downloaded file metadata")?;
    let file_size = metadata.len();

    if file_size == 0 {
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(anyhow!("Downloaded file is empty (0 bytes). Please try again."));
    }

    if file_size > 50 * 1024 * 1024 {
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(anyhow!("Downloaded file is too large (>50MB). This may not be a valid release."));
    }

    // Download and verify checksums
    println!("📥 Downloading checksums...");
    let checksums_url = format!(
        "https://github.com/{}/releases/download/v{}/checksums.txt",
        GITHUB_REPO, latest_version
    );
    let checksums_path = tmp_dir.join("checksums.txt");

    let checksums_status = Command::new("curl")
        .args(["-L", "-o", checksums_path.to_str().unwrap(), &checksums_url])
        .status()
        .context("Failed to download checksums")?;

    if !checksums_status.success() {
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(anyhow!(
            "❌ Could not download checksums for verification.\n   \
            Please check your internet connection or try again later."
        ));
    }

    // Parse checksums file
    let checksums_content = fs::read_to_string(&checksums_path)
        .context("Failed to read checksums file")?;
    let checksums = parse_checksums_file(&checksums_content);

    if checksums.is_empty() {
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(anyhow!("Checksums file is empty or invalid"));
    }

    // Get expected checksum for our archive
    let archive_filename = format!("ez-{}.{}", platform.name, platform.archive_ext);
    let expected_checksum = get_checksum_for_file(&checksums, &archive_filename)
        .with_context(|| format!("No checksum found for {}", archive_filename))?;

    // Verify checksum
    println!("🔐 Verifying integrity...");
    let actual_checksum = calculate_sha256(&archive_path)
        .context("Failed to calculate checksum of downloaded file")?;

    if !actual_checksum.eq_ignore_ascii_case(&expected_checksum) {
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(anyhow!(
            "❌ Checksum verification failed! Downloaded file may be corrupted or tampered with.\n\n   \
            Expected: {}\n   \
            Actual:   {}\n\n   \
            Suggestions:\n   \
            - Check your internet connection\n   \
            - Try again later\n   \
            - Report if problem persists at https://github.com/{}/issues",
            expected_checksum, actual_checksum, GITHUB_REPO
        ));
    }

    println!("✅ Verification successful");

    // Extract archive
    println!("📦 Extracting...");
    let extract_status = if platform.archive_ext == "tar.gz" {
        Command::new("tar")
            .args(["-xzf", archive_path.to_str().unwrap(), "-C", tmp_dir.to_str().unwrap()])
            .status()
            .context("Failed to extract archive. Is tar installed?")?
    } else {
        Command::new("unzip")
            .args(["-q", archive_path.to_str().unwrap(), "-d", tmp_dir.to_str().unwrap()])
            .status()
            .context("Failed to extract archive. Is unzip installed?")?
    };

    if !extract_status.success() {
        return Err(anyhow!("Failed to extract downloaded archive"));
    }

    // Get current binary path
    let current_binary = get_current_binary_path()?;
    let binary_name = if cfg!(windows) { "ez.exe" } else { "ez" };
    let new_binary = tmp_dir.join(binary_name);

    // Replace current binary
    println!("📥 Installing update...");

    // On Unix systems, we need to be careful about replacing a running binary
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // Make new binary executable
        let mut perms = fs::metadata(&new_binary)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&new_binary, perms)?;

        // Create backup
        let backup_path = current_binary.with_extension("bak");
        let _ = fs::copy(&current_binary, &backup_path);

        // Replace binary
        fs::copy(&new_binary, &current_binary)
            .context("Failed to replace binary. You may need to run with sudo or install manually.")?;
    }

    #[cfg(windows)]
    {
        // On Windows, we can't replace a running exe, so we need a different approach
        let backup_path = current_binary.with_extension("bak");
        fs::rename(&current_binary, &backup_path)
            .context("Failed to backup current binary")?;

        if let Err(e) = fs::copy(&new_binary, &current_binary) {
            // Restore backup on failure
            let _ = fs::rename(&backup_path, &current_binary);
            return Err(e.into());
        }
    }

    // Cleanup
    let _ = fs::remove_dir_all(&tmp_dir);

    println!();
    println!("✅ Successfully updated to v{}", latest_version);
    println!();
    println!("🔄 Please restart your shell or run a new ez command to use the updated version.");

    Ok(())
}
