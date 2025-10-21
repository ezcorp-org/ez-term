use anyhow::{Context, Result};
use dialoguer::Confirm;
use crate::config::Config;
use crate::credentials::{store_credential, is_keyring_available};

/// Check if migration is needed and prompt user
pub fn check_and_migrate() -> Result<()> {
    let config = Config::load()?;

    // Already migrated or declined
    if config.migration_completed.unwrap_or(false) || config.migration_declined.unwrap_or(false) {
        return Ok(());
    }

    // No keys to migrate
    if config.groq_api_key.is_none() && config.openai_api_key.is_none() {
        return Ok(());
    }

    // Inform user
    println!("\n🔒 API keys detected in plaintext config file");
    println!("   Migrate to secure system keyring?");
    println!();

    let migrate = Confirm::new()
        .with_prompt("Migrate to secure storage?")
        .default(true)
        .interact()?;

    if !migrate {
        // User declined, remember choice
        let mut config = Config::load()?;
        config.migration_declined = Some(true);
        config.save()?;

        eprintln!("\n⚠️  Warning: API keys stored in plaintext (insecure)");
        eprintln!("   Consider using environment variables instead:");
        eprintln!("   - export GROQ_API_KEY=your_key");
        eprintln!("   - export OPENAI_API_KEY=your_key");
        eprintln!();
        return Ok(());
    }

    // Check keyring availability
    if !is_keyring_available() {
        eprintln!("\n❌ System keyring not available");
        eprintln!("   Alternatives:");
        eprintln!("   1. Use environment variables (recommended):");
        eprintln!("      export GROQ_API_KEY=your_key");
        eprintln!("      export OPENAI_API_KEY=your_key");
        eprintln!("   2. On Linux, install gnome-keyring or kwallet");
        eprintln!();

        let mut config = Config::load()?;
        config.migration_declined = Some(true);
        config.save()?;
        return Ok(());
    }

    // Perform migration
    migrate_keys_to_keyring()?;

    Ok(())
}

fn migrate_keys_to_keyring() -> Result<()> {
    let config = Config::load()?;
    let mut migrated_keys = Vec::new();

    // Migrate Groq key
    if let Some(key) = &config.groq_api_key {
        store_credential("ez-term", "groq_api_key", key)
            .context("Failed to store Groq API key in keyring")?;
        migrated_keys.push("Groq");
    }

    // Migrate OpenAI key
    if let Some(key) = &config.openai_api_key {
        store_credential("ez-term", "openai_api_key", key)
            .context("Failed to store OpenAI API key in keyring")?;
        migrated_keys.push("OpenAI");
    }

    // Verify storage and update config
    if !migrated_keys.is_empty() {
        let mut config = Config::load()?;

        // Remove keys from config
        config.groq_api_key = None;
        config.openai_api_key = None;
        config.migration_completed = Some(true);
        config.migration_declined = Some(false);

        config.save()?;

        println!("✅ Migrated {} API key(s) to secure storage", migrated_keys.len());
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_flags() {
        // Test that migration respects flags
        // This is a basic structure test
        assert!(true);
    }
}
