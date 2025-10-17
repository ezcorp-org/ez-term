#!/bin/zsh
# ez shell wrapper - automatically executes generated commands
# Source this file in your ~/.zshrc

ez() {
    # Check if OLLAMA_HOST is set
    if [[ -z "$OLLAMA_HOST" ]]; then
        export OLLAMA_HOST="http://192.168.0.199:11434"
    fi

    # If no arguments, run interactive mode normally
    if [[ $# -eq 0 ]]; then
        ~/.local/bin/ez
        return $?
    fi

    # For special flags, pass through directly
    case "$1" in
        --help|-h|--version|-V|--list-models|--list-backends|--set-model|--set-backend)
            ~/.local/bin/ez "$@"
            return $?
            ;;
    esac

    # Generate command and capture output
    echo "🤖 Generating command..."
    local cmd_output
    cmd_output=$(~/.local/bin/ez "$@" 2>&1)

    # Check if command generation failed
    if [[ $? -ne 0 ]]; then
        echo "$cmd_output"
        return 1
    fi

    # Parse output: description (💡 line) and command (last line)
    local description
    local generated_cmd

    # Extract description (line starting with 💡)
    description=$(echo "$cmd_output" | grep "^💡" | sed 's/^💡 //')

    # Extract command (last line that's not "Gathering system context" and not the description)
    generated_cmd=$(echo "$cmd_output" | grep -v "Gathering system context" | grep -v "^💡" | tail -n 1 | sed 's/^[[:space:]]*//')

    # Check if command is empty
    if [[ -z "$generated_cmd" ]]; then
        echo "❌ No command generated"
        return 1
    fi

    # Display the description if available
    if [[ -n "$description" ]]; then
        echo ""
        echo "💡 $description"
    fi

    # Display the generated command
    echo ""
    echo "📋 Generated command:"
    echo "   $generated_cmd"
    echo ""

    # Check if it's a refusal
    if echo "$generated_cmd" | grep -q "Refusing: unsafe operation"; then
        echo "⚠️  Command refused for safety"
        return 0
    fi

    # Inject command into zsh line buffer
    print -z "$generated_cmd"
}

# Auto-completion for ez
_ez_completion() {
    local -a completions
    completions=(
        '--help:Show help'
        '--version:Show version'
        '--list-models:List available models'
        '--list-backends:List available backends'
        '--set-model:Set default model'
        '--set-backend:Set default backend'
        '-h:Show help'
        '-V:Show version'
        '-b:Backend to use'
        '-m:Model to use'
    )

    _describe 'ez' completions
}

compdef _ez_completion ez

echo "✅ ez command generator loaded!"
echo "   Usage: ez \"your natural language request\""
echo "   Example: ez \"find large files\""
