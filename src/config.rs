use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::credentials::get_credential;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend: Option<String>,
    pub model: Option<String>,
    pub ollama_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groq_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_declined: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backend: None,
            model: None,
            ollama_url: None, // Let environment variable or fallback handle this
            groq_api_key: None,
            openai_api_key: None,
            migration_completed: None,
            migration_declined: None,
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".config/ez-term/config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {:?}", path))?;

        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write config to {:?}", path))?;

        Ok(())
    }

    pub fn get_backend(&self) -> String {
        self.backend.clone().unwrap_or_else(|| "groq".to_string())
    }

    pub fn get_groq_api_key(&self) -> Option<String> {
        // Priority: env var > keyring > config (deprecated)
        match get_credential("ez-term", "groq_api_key", "GROQ_API_KEY") {
            Ok(Some(key)) => Some(key),
            Ok(None) => {
                // Fallback to plaintext config (deprecated)
                if let Some(key) = &self.groq_api_key {
                    eprintln!("⚠️  Warning: API key stored in plaintext config (deprecated)");
                    eprintln!("   Run 'ez init' to migrate to secure storage");
                    Some(key.clone())
                } else {
                    None
                }
            }
            Err(_) => {
                // Keyring error, fall back to config
                self.groq_api_key.clone()
            }
        }
    }

    pub fn get_openai_api_key(&self) -> Option<String> {
        // Priority: env var > keyring > config (deprecated)
        match get_credential("ez-term", "openai_api_key", "OPENAI_API_KEY") {
            Ok(Some(key)) => Some(key),
            Ok(None) => {
                // Fallback to plaintext config (deprecated)
                if let Some(key) = &self.openai_api_key {
                    eprintln!("⚠️  Warning: API key stored in plaintext config (deprecated)");
                    eprintln!("   Run 'ez init' to migrate to secure storage");
                    Some(key.clone())
                } else {
                    None
                }
            }
            Err(_) => {
                // Keyring error, fall back to config
                self.openai_api_key.clone()
            }
        }
    }

    pub fn get_ollama_url(&self) -> String {
        // Priority: config file > OLLAMA_HOST env var > default
        self.ollama_url.clone()
            .or_else(|| std::env::var("OLLAMA_HOST").ok())
            .unwrap_or_else(|| "http://localhost:11434".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_env() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        std::env::set_var("HOME", temp_dir.path());
        temp_dir
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.backend.is_none());
        assert!(config.model.is_none());
        assert!(config.ollama_url.is_none());
    }

    #[test]
    fn test_config_save_and_load() {
        let _temp = setup_test_env();

        let config = Config {
            backend: Some("ollama".to_string()),
            model: Some("qwen3-coder:latest".to_string()),
            ollama_url: Some("http://test:11434".to_string()),
            groq_api_key: None,
            openai_api_key: None,
            migration_completed: None,
            migration_declined: None,
        };

        // Save
        config.save().unwrap();

        // Load
        let loaded = Config::load().unwrap();
        assert_eq!(loaded.backend, Some("ollama".to_string()));
        assert_eq!(loaded.model, Some("qwen3-coder:latest".to_string()));
        assert_eq!(loaded.ollama_url, Some("http://test:11434".to_string()));
    }

    #[test]
    fn test_config_load_nonexistent() {
        let _temp = setup_test_env();
        let config = Config::load().unwrap();
        assert!(config.backend.is_none());
    }

    #[test]
    fn test_get_backend_default() {
        let config = Config::default();
        assert_eq!(config.get_backend(), "groq");
    }

    #[test]
    fn test_get_backend_custom() {
        let config = Config {
            backend: Some("ollama".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_backend(), "ollama");
    }

    #[test]
    fn test_get_ollama_url_priority() {
        // Test config file priority
        let config = Config {
            ollama_url: Some("http://config:11434".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_ollama_url(), "http://config:11434");

        // Test env var fallback
        std::env::set_var("OLLAMA_HOST", "http://env:11434");
        let config = Config::default();
        assert_eq!(config.get_ollama_url(), "http://env:11434");
        std::env::remove_var("OLLAMA_HOST");

        // Test default fallback
        let config = Config::default();
        assert_eq!(config.get_ollama_url(), "http://localhost:11434");
    }

    #[test]
    fn test_get_groq_api_key() {
        // Test config file
        let config = Config {
            groq_api_key: Some("config_key".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_groq_api_key(), Some("config_key".to_string()));

        // Test env var
        std::env::set_var("GROQ_API_KEY", "env_key");
        let config = Config::default();
        assert_eq!(config.get_groq_api_key(), Some("env_key".to_string()));
        std::env::remove_var("GROQ_API_KEY");

        // Test none
        let config = Config::default();
        assert_eq!(config.get_groq_api_key(), None);
    }

    #[test]
    fn test_get_openai_api_key() {
        // Test config file
        let config = Config {
            openai_api_key: Some("config_key".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_openai_api_key(), Some("config_key".to_string()));

        // Test env var
        std::env::set_var("OPENAI_API_KEY", "env_key");
        let config = Config::default();
        assert_eq!(config.get_openai_api_key(), Some("env_key".to_string()));
        std::env::remove_var("OPENAI_API_KEY");

        // Test none
        let config = Config::default();
        assert_eq!(config.get_openai_api_key(), None);
    }
}
