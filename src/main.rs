mod cli;
mod command_validator;
mod config;
mod context_agent;
mod credentials;
mod llm_client;
mod migration;
mod prompt_sanitizer;
mod setup;
mod system_info;
mod tool_detection;
mod update;
mod verification;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use context_agent::ContextAgent;
use llm_client::{Backend, LLMClient};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use serde::{Deserialize, Serialize};
use setup::SetupWizard;
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
    agent: &ContextAgent,
) -> Result<()> {
    // Step 1: Sanitize user input to prevent prompt injection
    let sanitized_query = prompt_sanitizer::sanitize_user_input(query);

    // Step 2: Get agentic context based on the query
    let agentic_context = agent.get_relevant_context(&sanitized_query);

    // Step 3: Create structured system prompt with clear boundaries
    let system_prompt = prompt_sanitizer::create_system_prompt(context);

    // Combine with agentic context
    let full_context = if !agentic_context.is_empty() {
        format!("{}\n{}", system_prompt, agentic_context)
    } else {
        system_prompt
    };

    // Step 4: Generate command from LLM
    let response = client.generate_and_collect(&sanitized_query, &full_context).await?;

    // Step 5: Parse response
    let (description, command) = match serde_json::from_str::<CommandResponse>(&response) {
        Ok(cmd_response) => (cmd_response.description, cmd_response.command),
        Err(_) => {
            // Fallback: try to extract JSON from response
            if let Some(json_start) = response.find('{') {
                if let Some(json_end) = response.rfind('}') {
                    let json_str = &response[json_start..=json_end];
                    if let Ok(cmd_response) = serde_json::from_str::<CommandResponse>(json_str) {
                        (cmd_response.description, cmd_response.command)
                    } else {
                        // Final fallback: just print the raw response
                        println!("{}", response);
                        return Err(anyhow!("Failed to parse JSON response"));
                    }
                } else {
                    println!("{}", response);
                    return Err(anyhow!("Failed to parse JSON response"));
                }
            } else {
                println!("{}", response);
                return Err(anyhow!("Failed to parse JSON response"));
            }
        }
    };

    // Step 6: Validate command for security risks
    let validator = command_validator::CommandValidator::new();

    match validator.validate(&command) {
        Ok(command_validator::RiskLevel::Safe) => {
            // Safe command - print normally
            println!("💡 {}\n", description);
            println!("{}", command);
        }
        Ok(command_validator::RiskLevel::Medium) => {
            // Risky command - print warning but still output command
            let warning = validator.get_warning_message(&command);
            println!("💡 {}\n", description);
            println!("warning: {}", warning);
            println!("{}", command);
        }
        Ok(command_validator::RiskLevel::Critical) => {
            // Critical command - print strong warning AND command
            // User will see the warning but can still execute if they type it manually
            let critical_msg = validator.get_critical_message(&command);
            println!("💡 {}\n", description);
            println!("critical: {}", critical_msg);
            println!("{}", command);
        }
        Err(e) => {
            // Validation error (shouldn't happen with current implementation)
            println!("error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // Handle update command
    if args.update {
        return update::update();
    }

    // Handle init subcommand or --init flag - run setup wizard
    if args.init || matches!(args.command, Some(Commands::Init)) {
        let mut wizard = SetupWizard::new();
        return wizard.run().await;
    }

    // Check for first-run (no config exists) and trigger setup wizard
    let config_path = Config::config_path()?;
    if !config_path.exists() && !args.init {
        println!("Welcome to ez-term! It looks like this is your first time running the tool.");
        println!("Let's set up your configuration.\n");

        let mut wizard = SetupWizard::new();
        wizard.run().await?;

        // After setup, reload config and continue
    }

    // Load configuration
    let mut config = Config::load()?;

    // Check for migration (skip if running init or update commands)
    if !args.init && !matches!(args.command, Some(Commands::Init)) && !args.update {
        // Non-interactive migration check for regular commands
        if config.groq_api_key.is_some() || config.openai_api_key.is_some() {
            if !config.migration_completed.unwrap_or(false) && !config.migration_declined.unwrap_or(false) {
                // Only run migration for interactive commands
                if args.query.is_none() && atty::is(atty::Stream::Stdin) {
                    let _ = migration::check_and_migrate();
                    // Reload config after potential migration
                    config = Config::load()?;
                }
            }
        }
    }

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
    let context_agent = ContextAgent::new()?;

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
        4. Generate commands that actually DO something, not commands that just print text\n\
        5. IMPORTANT: Generate commands specific to the user's OS (see system info below)\n\n\
        Output format:\n\
        - Output valid JSON only, on one line\n\
        - JSON schema: {{\"command\": \"actual executable command\", \"description\": \"what this command does\"}}\n\
        - NO prose before/after JSON, NO code fences, NO comments, NO trailing spaces\n\
        - Description: 1-2 sentences explaining the command's purpose and key flags\n\n\
        Command generation guidelines:\n\
        - Use the ACTUAL tools installed on the user's system (see below)\n\
        - Generate COMPLETE commands with all necessary flags and arguments\n\
        - **MATCH the user's OS**: Use OS-specific commands and flag syntax (Linux/macOS/Windows)\n\
        - **Linux**: Use GNU-style commands (apt/dnf, systemctl, --long-flags, /proc)\n\
        - **macOS**: Use BSD-style commands (brew, launchctl, single-dash flags, different flag syntax)\n\
        - **Windows**: Use PowerShell commands or cmd.exe built-ins\n\
        - When user intent is ambiguous, prefer safe read-only operations (use -print, --dry-run, list-only modes)\n\
        - When user explicitly requests destructive operations, generate the actual command they're asking for\n\
        - Quote paths/globs properly (use quotes around paths with spaces)\n\
        - For file operations: prefer find, ls, stat, du, grep (read-only)\n\
        - For git: use status, log, diff, show (read-only by default)\n\
        - For archives: use tar -tzf (list), zip -sf (show), unzip -l (list)\n\
        - For system info: use df, du, free, top, ps, uname (safe read-only)\n\
        - Combine steps with && only when ALL steps are safe/non-destructive\n\n\
        Safety rules:\n\
        - Generate the ACTUAL command the user is asking for\n\
        - Be truthful and accurate - don't refuse or substitute commands\n\
        - Command validation and safety checks happen after generation\n\
        - Never expose/print credentials, API keys, passwords, tokens in commands\n\
        - Explain clearly what the command does in the description field\n\n\
        Example good responses:\n\
        - User: \"find large files\" → {{\"command\": \"find . -type f -size +100M -exec ls -lh {{}} \\\\;\", \"description\": \"Finds files larger than 100MB in current directory and shows their size in human-readable format.\"}}\n\
        - User: \"compress folder\" → {{\"command\": \"tar -czf archive.tar.gz .\", \"description\": \"Creates a gzip-compressed tar archive of the current directory.\"}}\n\
        - User: \"show git changes\" → {{\"command\": \"git diff --stat\", \"description\": \"Shows a summary of changed files and the number of additions/deletions.\"}}\n\
        - User: \"list processes\" (Linux) → {{\"command\": \"ps aux --sort=-%mem | head -20\", \"description\": \"Lists top 20 processes sorted by memory usage.\"}}\n\
        - User: \"list processes\" (macOS) → {{\"command\": \"ps aux -m | head -20\", \"description\": \"Lists top 20 processes sorted by memory usage.\"}}\n\n\
        Example bad responses (NEVER DO THIS):\n\
        - {{\"command\": \"echo 'list files'\", ...}} ← WRONG: this just echoes text, doesn't list files\n\
        - {{\"command\": \"echo 'You can use ls'\", ...}} ← WRONG: this is advice, not a command\n\
        - {{\"command\": \"# ls -la\", ...}} ← WRONG: this is a comment, not executable\n\n\
        User's system information:\n{}\n\n\
        IMPORTANT: The user is on {}. Generate commands that work specifically on this OS!\n\n\
        Now generate a REAL, EXECUTABLE command as JSON for this request:",
        system_context,
        system_info.os.to_uppercase()
    );

    // If query provided as argument, process it and exit (non-interactive mode)
    if let Some(query) = args.query {
        println!("Gathering system context...\n");
        return process_query(&query, &client, &context, &context_agent).await;
    }

    // Check if stdin has data (piped input)
    if !atty::is(atty::Stream::Stdin) {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        if !buffer.trim().is_empty() {
            println!("Gathering system context...\n");
            return process_query(buffer.trim(), &client, &context, &context_agent).await;
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
                let _ = process_query(query, &client, &context, &context_agent).await;

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
