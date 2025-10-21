use anyhow::{anyhow, Context, Result};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Calculate SHA256 checksum of a file
pub fn calculate_sha256(path: &Path) -> Result<String> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open file for checksum: {:?}", path))?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = reader.read(&mut buffer)
            .context("Failed to read file during checksum calculation")?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

/// Parse checksums file format: "<sha256>  <filename>"
pub fn parse_checksums_file(content: &str) -> HashMap<String, String> {
    let mut checksums = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let checksum = parts[0];
            let filename = parts[1..].join(" ");
            checksums.insert(filename, checksum.to_string());
        }
    }

    checksums
}

/// Verify file checksum against expected value
pub fn verify_checksum(file_path: &Path, expected: &str) -> Result<bool> {
    let actual = calculate_sha256(file_path)?;
    Ok(actual.eq_ignore_ascii_case(expected))
}

/// Extract checksum for a specific file from checksums map
pub fn get_checksum_for_file(checksums: &HashMap<String, String>, filename: &str) -> Result<String> {
    // Try exact match first
    if let Some(checksum) = checksums.get(filename) {
        return Ok(checksum.clone());
    }

    // Try matching just the basename
    for (key, value) in checksums.iter() {
        if key.ends_with(filename) {
            return Ok(value.clone());
        }
    }

    Err(anyhow!("No checksum found for file: {}", filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_calculate_sha256() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"hello world").unwrap();
        temp_file.flush().unwrap();

        let checksum = calculate_sha256(temp_file.path()).unwrap();
        // SHA256 of "hello world"
        assert_eq!(checksum, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    }

    #[test]
    fn test_parse_checksums_file() {
        let content = r#"
a1b2c3d4  ez-linux-x86_64.tar.gz
e5f6g7h8  ez-macos-aarch64.tar.gz
        "#;

        let checksums = parse_checksums_file(content);
        assert_eq!(checksums.len(), 2);
        assert_eq!(checksums.get("ez-linux-x86_64.tar.gz"), Some(&"a1b2c3d4".to_string()));
        assert_eq!(checksums.get("ez-macos-aarch64.tar.gz"), Some(&"e5f6g7h8".to_string()));
    }

    #[test]
    fn test_verify_checksum_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test data").unwrap();
        temp_file.flush().unwrap();

        let expected = calculate_sha256(temp_file.path()).unwrap();
        let result = verify_checksum(temp_file.path(), &expected).unwrap();
        assert!(result);
    }

    #[test]
    fn test_verify_checksum_failure() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test data").unwrap();
        temp_file.flush().unwrap();

        let result = verify_checksum(temp_file.path(), "wrong_checksum").unwrap();
        assert!(!result);
    }

    #[test]
    fn test_get_checksum_for_file() {
        let mut checksums = HashMap::new();
        checksums.insert("ez-linux-x86_64.tar.gz".to_string(), "abc123".to_string());
        checksums.insert("dir/ez-macos.tar.gz".to_string(), "def456".to_string());

        // Exact match
        assert_eq!(get_checksum_for_file(&checksums, "ez-linux-x86_64.tar.gz").unwrap(), "abc123");

        // Basename match
        assert_eq!(get_checksum_for_file(&checksums, "ez-macos.tar.gz").unwrap(), "def456");

        // No match
        assert!(get_checksum_for_file(&checksums, "nonexistent.tar.gz").is_err());
    }
}
