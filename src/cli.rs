use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ez")]
#[command(about = "Lightweight CLI AI assistant powered by local or cloud LLMs", long_about = None)]
#[command(version)]
pub struct Cli {
    /// The query to ask the AI (if not provided, enters interactive mode)
    #[arg(value_name = "QUERY")]
    pub query: Option<String>,

    /// Backend to use (ollama, groq, openai)
    #[arg(short, long)]
    pub backend: Option<String>,

    /// Model to use
    #[arg(short, long)]
    pub model: Option<String>,

    /// List available backends
    #[arg(long)]
    pub list_backends: bool,

    /// List available models for current backend
    #[arg(long)]
    pub list_models: bool,

    /// Set default backend
    #[arg(long)]
    pub set_backend: Option<String>,

    /// Set default model
    #[arg(long)]
    pub set_model: Option<String>,

    /// Update ez to the latest version
    #[arg(long)]
    pub update: bool,

    /// Run interactive setup wizard to configure ez-term (backend, model, API keys)
    #[arg(long)]
    pub init: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run interactive setup wizard to configure ez-term
    Init,
}
