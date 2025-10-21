use anyhow::{anyhow, Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::config::Config;
use crate::llm_client::{Backend, LLMClient};
use crate::credentials::{store_credential, is_keyring_available};

pub struct SetupWizard {
    pub backend: Option<Backend>,
    pub model: Option<String>,
    pub ollama_url: Option<String>,
    pub api_key: Option<String>,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            backend: None,
            model: None,
            ollama_url: None,
            api_key: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("\n┌─────────────────────────────────────────┐");
        println!("│   Welcome to ez-term Setup Wizard!     │");
        println!("└─────────────────────────────────────────┘\n");
        println!("This wizard will help you configure ez-term for first use.\n");

        // Step 1: Backend selection
        self.backend = Some(self.select_backend()?);

        // Step 2: Backend-specific configuration
        match self.backend.as_ref().unwrap() {
            Backend::Ollama => {
                self.ollama_url = Some(self.configure_ollama_url().await?);
                self.model = Some(self.select_ollama_model(&self.ollama_url.clone().unwrap()).await?);
            }
            Backend::Groq => {
                self.api_key = self.configure_api_key("Groq")?;
                self.model = Some(self.select_cloud_model(Backend::Groq)?);
            }
            Backend::OpenAI => {
                self.api_key = self.configure_api_key("OpenAI")?;
                self.model = Some(self.select_cloud_model(Backend::OpenAI)?);
            }
        }

        // Step 3: Display summary and confirm
        self.display_summary();

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Save this configuration?")
            .default(true)
            .interact()?
        {
            self.save_config()?;
            println!("\n✓ Configuration saved successfully!");
            println!("You can now run 'ez' to start using the assistant.\n");
        } else {
            println!("\nSetup cancelled. No configuration was saved.\n");
        }

        Ok(())
    }

    fn select_backend(&self) -> Result<Backend> {
        let backends = vec![
            "ollama (local)",
            "groq (cloud)",
            "openai (cloud)",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select your preferred backend")
            .items(&backends)
            .default(0)
            .interact()?;

        match selection {
            0 => Ok(Backend::Ollama),
            1 => Ok(Backend::Groq),
            2 => Ok(Backend::OpenAI),
            _ => Err(anyhow!("Invalid selection")),
        }
    }

    async fn configure_ollama_url(&self) -> Result<String> {
        let default_url = "http://localhost:11434";

        println!("\nOllama Configuration");
        println!("─────────────────────");

        loop {
            let use_default = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Use default Ollama URL ({})? ", default_url))
                .default(true)
                .interact()?;

            let url = if use_default {
                default_url.to_string()
            } else {
                Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter Ollama URL")
                    .default(default_url.to_string())
                    .interact_text()?
            };

            // Test connection
            println!("\nTesting connection to Ollama...");
            let client = LLMClient::new(Backend::Ollama, None, None, url.clone());

            match client.list_models().await {
                Ok(_) => {
                    println!("✓ Successfully connected to Ollama");
                    return Ok(url);
                }
                Err(e) => {
                    println!("✗ Failed to connect to Ollama: {}", e);

                    if !Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Would you like to try a different URL?")
                        .default(true)
                        .interact()?
                    {
                        return Err(anyhow!("Could not connect to Ollama. Please ensure Ollama is running."));
                    }
                }
            }
        }
    }

    async fn select_ollama_model(&self, ollama_url: &str) -> Result<String> {
        println!("\nFetching available Ollama models...");

        let client = LLMClient::new(Backend::Ollama, None, None, ollama_url.to_string());
        let models = client.list_models().await
            .context("Failed to fetch Ollama models")?;

        if models.is_empty() {
            return Err(anyhow!(
                "No models found in Ollama.\nPlease install a model first using 'ollama pull <model-name>'"
            ));
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a model")
            .items(&models)
            .default(0)
            .interact()?;

        Ok(models[selection].clone())
    }

    fn select_cloud_model(&self, backend: Backend) -> Result<String> {
        let models = backend.available_models();
        let default_model = backend.default_model();

        let default_index = models.iter()
            .position(|m| m == default_model)
            .unwrap_or(0);

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a model")
            .items(&models)
            .default(default_index)
            .interact()?;

        Ok(models[selection].clone())
    }

    fn configure_api_key(&self, provider: &str) -> Result<Option<String>> {
        println!("\n{} API Key Configuration", provider);
        println!("─────────────────────────────");
        println!("API keys will be stored securely in your system keyring.");
        println!("Alternatively, you can set it later via environment variable: {}_API_KEY\n", provider.to_uppercase());

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to enter your API key now?")
            .default(false)
            .interact()?
        {
            let api_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your API key")
                .interact_text()?;

            if api_key.trim().is_empty() {
                return Ok(None);
            }

            // Try to store in keyring
            let key_name = format!("{}_api_key", provider.to_lowercase());
            if is_keyring_available() {
                match store_credential("ez-term", &key_name, &api_key) {
                    Ok(_) => {
                        println!("✅ API key stored securely in system keyring");
                        // Don't return the key - it's in keyring now
                        Ok(None)
                    }
                    Err(e) => {
                        eprintln!("⚠️  Could not store in keyring: {}", e);
                        eprintln!("   Falling back to config file (less secure)");
                        Ok(Some(api_key))
                    }
                }
            } else {
                eprintln!("⚠️  System keyring not available");
                eprintln!("   Storing in config file (less secure)");
                eprintln!("   Consider using environment variables instead");
                Ok(Some(api_key))
            }
        } else {
            Ok(None)
        }
    }

    fn display_summary(&self) {
        println!("\n┌─────────────────────────────────────────┐");
        println!("│         Configuration Summary           │");
        println!("└─────────────────────────────────────────┘\n");

        if let Some(backend) = &self.backend {
            println!("Backend:  {}", match backend {
                Backend::Ollama => "ollama (local)",
                Backend::Groq => "groq (cloud)",
                Backend::OpenAI => "openai (cloud)",
            });
        }

        if let Some(model) = &self.model {
            println!("Model:    {}", model);
        }

        if let Some(url) = &self.ollama_url {
            println!("Ollama URL: {}", url);
        }

        if let Some(key) = &self.api_key {
            println!("API Key:  {}...", &key.chars().take(8).collect::<String>());
        } else if matches!(self.backend, Some(Backend::Groq) | Some(Backend::OpenAI)) {
            println!("API Key:  (will use environment variable)");
        }

        println!();
    }

    fn save_config(&self) -> Result<()> {
        let backend_str = match self.backend.as_ref().unwrap() {
            Backend::Ollama => "ollama",
            Backend::Groq => "groq",
            Backend::OpenAI => "openai",
        };

        let config = Config {
            backend: Some(backend_str.to_string()),
            model: self.model.clone(),
            ollama_url: self.ollama_url.clone(),
            groq_api_key: if matches!(self.backend, Some(Backend::Groq)) {
                self.api_key.clone()
            } else {
                None
            },
            openai_api_key: if matches!(self.backend, Some(Backend::OpenAI)) {
                self.api_key.clone()
            } else {
                None
            },
            migration_completed: None,
            migration_declined: None,
        };

        config.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_wizard_creation() {
        let wizard = SetupWizard::new();

        // Verify initial state
        assert!(wizard.backend.is_none());
        assert!(wizard.model.is_none());
        assert!(wizard.ollama_url.is_none());
        assert!(wizard.api_key.is_none());
    }

    #[test]
    fn test_backend_available_models() {
        // Test that cloud backends return non-empty model lists
        let groq_models = Backend::Groq.available_models();
        assert!(!groq_models.is_empty());
        assert!(groq_models.contains(&"llama-3.3-70b-versatile".to_string()));

        let openai_models = Backend::OpenAI.available_models();
        assert!(!openai_models.is_empty());
        assert!(openai_models.contains(&"gpt-3.5-turbo".to_string()));

        // Ollama returns empty list (dynamic from API)
        let ollama_models = Backend::Ollama.available_models();
        assert!(ollama_models.is_empty());
    }

    #[test]
    fn test_backend_default_models() {
        assert_eq!(Backend::Ollama.default_model(), "llama2");
        assert_eq!(Backend::Groq.default_model(), "llama-3.3-70b-versatile");
        assert_eq!(Backend::OpenAI.default_model(), "gpt-3.5-turbo");
    }
}
