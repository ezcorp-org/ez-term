mod cli;
mod config;
mod llm_client;
mod system_info;
mod tool_detection;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::Cli;
use config::Config;
use llm_client::{Backend, LLMClient};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use system_info::SystemInfo;
use tool_detection::ToolDetection;

#[derive(Debug, Serialize, Deserialize)]
struct CommandResponse {
    command: String,
    description: String,
}

fn history_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME environment variable not set")?;
    Ok(PathBuf::from(home).join(".config/ez-term/history.txt"))
}

async fn process_query(
    query: &str,
    client: &LLMClient,
    context: &str,
) -> Result<()> {
    let response = client.generate_and_collect(query, context).await?;

    // Try to parse as JSON
    match serde_json::from_str::<CommandResponse>(&response) {
        Ok(cmd_response) => {
            // Print description first
            println!("💡 {}\n", cmd_response.description);
            // Then print just the command (for shell wrapper to parse)
            println!("{}", cmd_response.command);
            Ok(())
        }
        Err(_) => {
            // Fallback: if not valid JSON, try to extract JSON from response
            if let Some(json_start) = response.find('{') {
                if let Some(json_end) = response.rfind('}') {
                    let json_str = &response[json_start..=json_end];
                    if let Ok(cmd_response) = serde_json::from_str::<CommandResponse>(json_str) {
                        println!("💡 {}\n", cmd_response.description);
                        println!("{}", cmd_response.command);
                        return Ok(());
                    }
                }
            }

            // Final fallback: just print the raw response
            println!("{}", response);
            Err(anyhow!("Failed to parse JSON response"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Load configuration
    let mut config = Config::load()?;

    // Handle list backends
    if args.list_backends {
        println!("Available backends:");
        println!("  - ollama (local)");
        println!("  - groq (cloud, requires API key)");
        println!("  - openai (cloud, requires API key)");
        return Ok(());
    }

    // Handle set backend
    if let Some(backend) = args.set_backend {
        config.backend = Some(backend.clone());
        config.save()?;
        println!("Default backend set to: {}", backend);
        return Ok(());
    }

    // Handle set model
    if let Some(model) = args.set_model {
        config.model = Some(model.clone());
        config.save()?;
        println!("Default model set to: {}", model);
        return Ok(());
    }

    // Determine backend
    let backend_str = args
        .backend
        .or(config.backend.clone())
        .unwrap_or_else(|| "groq".to_string());

    let backend = Backend::from_str(&backend_str)?;

    // Get API key based on backend
    let api_key = match backend {
        Backend::Ollama => None,
        Backend::Groq => config.get_groq_api_key(),
        Backend::OpenAI => config.get_openai_api_key(),
    };

    // Create client
    let client = LLMClient::new(
        backend.clone(),
        args.model.or(config.model.clone()),
        api_key,
        config.get_ollama_url(),
    );

    // Handle list models
    if args.list_models {
        println!("Fetching available models for {}...", backend_str);
        match client.list_models().await {
            Ok(models) => {
                if models.is_empty() {
                    println!("No models available");
                } else {
                    println!("Available models:");
                    for model in models {
                        println!("  - {}", model);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error listing models: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Detect system information and tools once
    let system_info = SystemInfo::detect();
    let tool_detection = ToolDetection::detect();

    // Build context
    let system_context = format!(
        "System: {}\n{}",
        system_info.format_context(),
        tool_detection.format_context()
    );

    let context = format!(
        "You are an expert CLI command generator. Your job is to generate REAL, EXECUTABLE shell commands that accomplish the user's goal.\n\n\
        CRITICAL RULES:\n\
        1. ALWAYS generate an actual command - NEVER use echo as a placeholder\n\
        2. The command MUST be directly executable and accomplish the requested task\n\
        3. Use real tools available on the system (shown below)\n\
        4. Generate commands that actually DO something, not commands that just print text\n\n\
        Output format:\n\
        - Output valid JSON only, on one line\n\
        - JSON schema: {{\"command\": \"actual executable command\", \"description\": \"what this command does\"}}\n\
        - NO prose before/after JSON, NO code fences, NO comments, NO trailing spaces\n\
        - Command MUST be POSIX-compatible (bash/sh) and work on Linux/macOS\n\
        - Description: 1-2 sentences explaining the command's purpose and key flags\n\n\
        Command generation guidelines:\n\
        - Use the ACTUAL tools installed on the user's system (see below)\n\
        - Generate COMPLETE commands with all necessary flags and arguments\n\
        - Default to safe, non-destructive operations (use -print, --dry-run, list-only modes)\n\
        - Quote paths/globs properly (use quotes around paths with spaces)\n\
        - For file operations: prefer find, ls, stat, du, grep (read-only)\n\
        - For git: use status, log, diff, show (read-only by default)\n\
        - For archives: use tar -tzf (list), zip -sf (show), unzip -l (list)\n\
        - For system info: use df, du, free, top, ps, uname (safe read-only)\n\
        - Combine steps with && only when ALL steps are safe/non-destructive\n\n\
        Safety rules (STRICT):\n\
        - NEVER generate: sudo, rm -rf, dd, mkfs, fdisk, chmod 777, curl | sh, eval, commands with > /dev/*\n\
        - For destructive operations: provide SAFE PREVIEW version instead\n\
          Example: User asks \"delete tmp files\" → generate: find . -name '*.tmp' -type f -print\n\
        - If NO safe preview exists: {{\"command\": \"echo 'Refusing: unsafe operation'\", \"description\": \"This operation is unsafe and has no safe preview.\"}}\n\
        - Never expose/print credentials, API keys, passwords, tokens\n\n\
        Example good responses:\n\
        - User: \"find large files\" → {{\"command\": \"find . -type f -size +100M -exec ls -lh {{}} \\\\;\", \"description\": \"Finds files larger than 100MB in current directory and shows their size in human-readable format.\"}}\n\
        - User: \"compress folder\" → {{\"command\": \"tar -czf archive.tar.gz .\", \"description\": \"Creates a gzip-compressed tar archive of the current directory.\"}}\n\
        - User: \"show git changes\" → {{\"command\": \"git diff --stat\", \"description\": \"Shows a summary of changed files and the number of additions/deletions.\"}}\n\
        - User: \"list processes\" → {{\"command\": \"ps aux --sort=-%mem | head -20\", \"description\": \"Lists top 20 processes sorted by memory usage.\"}}\n\n\
        Example bad responses (NEVER DO THIS):\n\
        - {{\"command\": \"echo 'list files'\", ...}} ← WRONG: this just echoes text, doesn't list files\n\
        - {{\"command\": \"echo 'You can use ls'\", ...}} ← WRONG: this is advice, not a command\n\
        - {{\"command\": \"# ls -la\", ...}} ← WRONG: this is a comment, not executable\n\n\
        User's system information:\n{}\n\n\
        Now generate a REAL, EXECUTABLE command as JSON for this request:",
        system_context
    );

    // If query provided as argument, process it and exit (non-interactive mode)
    if let Some(query) = args.query {
        println!("Gathering system context...\n");
        return process_query(&query, &client, &context).await;
    }

    // Check if stdin has data (piped input)
    if !atty::is(atty::Stream::Stdin) {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            println!("Gathering system context...\n");
            return process_query(buffer.trim(), &client, &context).await;
        }
    }

    // Interactive mode
    println!("ez - terminal assistant (interactive mode)");
    println!("Backend: {} | Use Ctrl+D or 'exit' to quit, Ctrl+C to cancel", backend_str);
    println!();

    let mut rl = DefaultEditor::new()?;
    let history_file = history_path()?;

    // Create history directory if it doesn't exist
    if let Some(parent) = history_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Load history
    if history_file.exists() {
        let _ = rl.load_history(&history_file);
    }

    loop {
        let readline = rl.readline("ez> ");

        match readline {
            Ok(line) => {
                let query = line.trim();

                // Skip empty lines
                if query.is_empty() {
                    continue;
                }

                // Check for exit command
                if query.eq_ignore_ascii_case("exit")
                    || query.eq_ignore_ascii_case("quit")
                    || query.eq_ignore_ascii_case("q")
                {
                    println!("Goodbye!");
                    break;
                }

                // Add to history
                let _ = rl.add_history_entry(query);

                println!();

                // Process query
                let _ = process_query(query, &client, &context).await;

                println!();
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    let _ = rl.save_history(&history_file);

    Ok(())
}
