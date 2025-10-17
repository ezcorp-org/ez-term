use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backend: Option<String>,
    pub model: Option<String>,
    pub ollama_url: Option<String>,
    pub groq_api_key: Option<String>,
    pub openai_api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            backend: None,
            model: None,
            ollama_url: None, // Let environment variable or fallback handle this
            groq_api_key: None,
            openai_api_key: None,
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
        self.groq_api_key.clone()
            .or_else(|| std::env::var("GROQ_API_KEY").ok())
    }

    pub fn get_openai_api_key(&self) -> Option<String> {
        self.openai_api_key.clone()
            .or_else(|| std::env::var("OPENAI_API_KEY").ok())
    }

    pub fn get_ollama_url(&self) -> String {
        // Priority: config file > OLLAMA_HOST env var > default
        self.ollama_url.clone()
            .or_else(|| std::env::var("OLLAMA_HOST").ok())
            .unwrap_or_else(|| "http://localhost:11434".to_string())
    }
}
