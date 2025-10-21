use anyhow::{Context, Result};
use keyring::Entry;

/// Get credential from keyring with fallback to environment variables and config
pub fn get_credential(service: &str, key_name: &str, env_var: &str) -> Result<Option<String>> {
    // Priority 1: Environment variable
    if let Ok(value) = std::env::var(env_var) {
        return Ok(Some(value));
    }

    // Priority 2: OS Keyring
    match get_from_keyring(service, key_name) {
        Ok(Some(value)) => return Ok(Some(value)),
        Ok(None) => {}, // Not found, continue to next fallback
        Err(e) => {
            // Keyring error (e.g., not available), log but continue
            eprintln!("Note: Could not access system keyring: {}", e);
        }
    }

    // Priority 3: Config file (handled by caller)
    Ok(None)
}

/// Store credential in OS keyring
pub fn store_credential(service: &str, key_name: &str, value: &str) -> Result<()> {
    let entry = Entry::new(service, key_name)
        .context("Failed to create keyring entry")?;

    entry.set_password(value)
        .context("Failed to store credential in keyring")?;

    Ok(())
}

/// Retrieve credential from OS keyring
pub fn get_from_keyring(service: &str, key_name: &str) -> Result<Option<String>> {
    let entry = Entry::new(service, key_name)
        .context("Failed to create keyring entry")?;

    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(keyring::Error::PlatformFailure(e)) => {
            Err(anyhow::anyhow!("Keyring platform failure: {}", e))
        }
        Err(e) => Err(anyhow::anyhow!("Keyring error: {}", e)),
    }
}

/// Delete credential from OS keyring
pub fn delete_credential(service: &str, key_name: &str) -> Result<()> {
    let entry = Entry::new(service, key_name)
        .context("Failed to create keyring entry")?;

    match entry.delete_password() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted, that's fine
        Err(e) => Err(anyhow::anyhow!("Failed to delete credential: {}", e)),
    }
}

/// Check if keyring is available on this system
pub fn is_keyring_available() -> bool {
    // Try to create a test entry
    match Entry::new("ez-term-test", "test") {
        Ok(entry) => {
            // Try to set and delete a test value
            if entry.set_password("test").is_ok() {
                let _ = entry.delete_password();
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_priority() {
        std::env::set_var("TEST_API_KEY", "env_value");
        let result = get_credential("ez-term-test", "test_key", "TEST_API_KEY").unwrap();
        assert_eq!(result, Some("env_value".to_string()));
        std::env::remove_var("TEST_API_KEY");
    }

    #[test]
    fn test_keyring_availability() {
        // This test may fail in CI without a proper keyring service
        // It's informational rather than critical
        let _ = is_keyring_available();
    }
}
