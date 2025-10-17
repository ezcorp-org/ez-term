use anyhow::{anyhow, Context, Result};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum Backend {
    Ollama,
    Groq,
    OpenAI,
}

impl Backend {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "ollama" => Ok(Self::Ollama),
            "groq" => Ok(Self::Groq),
            "openai" => Ok(Self::OpenAI),
            _ => Err(anyhow!("Unknown backend: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Ollama => "ollama",
            Self::Groq => "groq",
            Self::OpenAI => "openai",
        }
    }

    pub fn default_model(&self) -> &str {
        match self {
            Self::Ollama => "llama2",
            Self::Groq => "llama-3.3-70b-versatile",
            Self::OpenAI => "gpt-3.5-turbo",
        }
    }

    pub fn available_models(&self) -> Vec<String> {
        match self {
            Self::Ollama => vec![], // Dynamic from API
            Self::Groq => vec![
                "llama-3.3-70b-versatile".to_string(),
                "llama-3.1-70b-versatile".to_string(),
                "mixtral-8x7b-32768".to_string(),
                "gemma2-9b-it".to_string(),
            ],
            Self::OpenAI => vec![
                "gpt-4".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Option<ChatMessage>,
    delta: Option<ChatMessage>,
}

pub struct LLMClient {
    client: Client,
    backend: Backend,
    model: String,
    api_key: Option<String>,
    ollama_url: String,
}

impl LLMClient {
    pub fn new(
        backend: Backend,
        model: Option<String>,
        api_key: Option<String>,
        ollama_url: String,
    ) -> Self {
        let model = model.unwrap_or_else(|| backend.default_model().to_string());

        Self {
            client: Client::new(),
            backend,
            model,
            api_key,
            ollama_url,
        }
    }

    pub async fn generate(&self, prompt: &str, system_context: &str) -> Result<()> {
        match self.backend {
            Backend::Ollama => self.generate_ollama(prompt, system_context).await,
            Backend::Groq => self.generate_groq(prompt, system_context).await,
            Backend::OpenAI => self.generate_openai(prompt, system_context).await,
        }
    }

    pub async fn generate_and_collect(&self, prompt: &str, system_context: &str) -> Result<String> {
        match self.backend {
            Backend::Ollama => self.generate_and_collect_ollama(prompt, system_context).await,
            Backend::Groq => self.generate_and_collect_groq(prompt, system_context).await,
            Backend::OpenAI => self.generate_and_collect_openai(prompt, system_context).await,
        }
    }

    async fn generate_ollama(&self, prompt: &str, system_context: &str) -> Result<()> {
        let url = format!("{}/api/generate", self.ollama_url);
        let full_prompt = format!("{}\n\n{}", system_context, prompt);

        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt: full_prompt,
            stream: true,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Ollama request failed with status: {}",
                response.status()
            ));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if let Ok(resp) = serde_json::from_str::<OllamaGenerateResponse>(line) {
                    print!("{}", resp.response);
                    use std::io::Write;
                    std::io::stdout().flush()?;
                }
            }
        }

        println!(); // Newline at the end
        Ok(())
    }

    async fn generate_groq(&self, prompt: &str, system_context: &str) -> Result<()> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow!("Groq API key not configured. Set GROQ_API_KEY environment variable or use --set-backend"))?;

        let url = "https://api.groq.com/openai/v1/chat/completions";

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_context.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            stream: true,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Groq API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Groq API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(resp) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(choices) = resp["choices"].as_array() {
                            if let Some(choice) = choices.first() {
                                if let Some(content) = choice["delta"]["content"].as_str() {
                                    print!("{}", content);
                                    use std::io::Write;
                                    std::io::stdout().flush()?;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(); // Newline at the end
        Ok(())
    }

    async fn generate_openai(&self, prompt: &str, system_context: &str) -> Result<()> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow!("OpenAI API key not configured. Set OPENAI_API_KEY environment variable"))?;

        let url = "https://api.openai.com/v1/chat/completions";

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_context.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            stream: true,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to OpenAI API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "OpenAI API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(resp) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(choices) = resp["choices"].as_array() {
                            if let Some(choice) = choices.first() {
                                if let Some(content) = choice["delta"]["content"].as_str() {
                                    print!("{}", content);
                                    use std::io::Write;
                                    std::io::stdout().flush()?;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(); // Newline at the end
        Ok(())
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        match self.backend {
            Backend::Ollama => self.list_ollama_models().await,
            Backend::Groq | Backend::OpenAI => Ok(self.backend.available_models()),
        }
    }

    async fn list_ollama_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.ollama_url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to list Ollama models: {}",
                response.status()
            ));
        }

        let data: serde_json::Value = response.json().await?;
        let models = data["models"]
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(models)
    }

    async fn generate_and_collect_ollama(&self, prompt: &str, system_context: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.ollama_url);
        let full_prompt = format!("{}\n\n{}", system_context, prompt);

        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt: full_prompt,
            stream: true,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Ollama request failed with status: {}",
                response.status()
            ));
        }

        let mut stream = response.bytes_stream();
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if let Ok(resp) = serde_json::from_str::<OllamaGenerateResponse>(line) {
                    full_response.push_str(&resp.response);
                }
            }
        }

        Ok(full_response.trim().to_string())
    }

    async fn generate_and_collect_groq(&self, prompt: &str, system_context: &str) -> Result<String> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow!("Groq API key not configured. Set GROQ_API_KEY environment variable or use --set-backend"))?;

        let url = "https://api.groq.com/openai/v1/chat/completions";

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_context.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            stream: true,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Groq API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "Groq API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let mut stream = response.bytes_stream();
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(resp) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(choices) = resp["choices"].as_array() {
                            if let Some(choice) = choices.first() {
                                if let Some(content) = choice["delta"]["content"].as_str() {
                                    full_response.push_str(content);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(full_response.trim().to_string())
    }

    async fn generate_and_collect_openai(&self, prompt: &str, system_context: &str) -> Result<String> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| anyhow!("OpenAI API key not configured. Set OPENAI_API_KEY environment variable"))?;

        let url = "https://api.openai.com/v1/chat/completions";

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_context.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            stream: true,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to OpenAI API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "OpenAI API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let mut stream = response.bytes_stream();
        let mut full_response = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(resp) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(choices) = resp["choices"].as_array() {
                            if let Some(choice) = choices.first() {
                                if let Some(content) = choice["delta"]["content"].as_str() {
                                    full_response.push_str(content);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(full_response.trim().to_string())
    }
}
