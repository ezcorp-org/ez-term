#!/usr/bin/env zsh
# ez shell wrapper for zsh
# This wrapper provides auto-execution with confirmation for generated commands

ez() {
    # Get the path to the ez binary
    local EZ_BIN="${EZ_BIN:-$HOME/.local/bin/ez}"

    # Fallback to checking in common locations
    if [[ ! -x "$EZ_BIN" ]]; then
        if [[ -x "$HOME/work/ez-term/target/release/ez" ]]; then
            EZ_BIN="$HOME/work/ez-term/target/release/ez"
        elif (( $+commands[ez] )); then
            EZ_BIN="${commands[ez]}"
        else
            echo "❌ ez binary not found. Please install ez or set EZ_BIN environment variable."
            return 1
        fi
    fi

    # If no arguments or flags are passed, run in interactive mode (no wrapper)
    if [[ $# -eq 0 ]]; then
        "$EZ_BIN"
        return $?
    fi

    # If any flags are passed, bypass wrapper and call binary directly
    case "$1" in
        --*|init)
            "$EZ_BIN" "$@"
            return $?
            ;;
    esac

    # Generate command
    echo "🤖 Generating command..."
    echo ""

    local output
    output=$("$EZ_BIN" "$@" 2>&1)
    local exit_code=$?

    if [[ $exit_code -ne 0 ]]; then
        echo "❌ Error generating command:"
        echo "$output"
        return $exit_code
    fi

    # Extract the command (last line of output after the description)
    # The output format is: "💡 Description\n\nCommand"
    local description=$(echo "$output" | grep "^💡" | sed 's/^💡 //')

    # Check for critical warnings BEFORE extracting command (format: "critical: message")
    if echo "$output" | grep -qi "^critical:"; then
        local critical_msg=$(echo "$output" | grep -i "^critical:" | sed 's/critical: //I')
        local actual_command=$(echo "$output" | grep -v "^💡\|^Gathering\|^critical:\|^$" | tail -n 1)

        echo ""
        echo -e "\033[1;31m════════════════════════════════════════════════════════════════\033[0m"
        echo -e "\033[1;31m🚨 CRITICAL COMMAND BLOCKED\033[0m"
        echo -e "\033[1;31m════════════════════════════════════════════════════════════════\033[0m"
        echo ""
        echo -e "\033[1;31m$critical_msg\033[0m"
        echo ""
        echo -e "\033[1;33mGenerated command (NOT inserted):\033[0m"
        echo -e "\033[1;31m$actual_command\033[0m"
        echo ""
        echo -e "\033[1;33mIf you really need to run this, type it manually.\033[0m"
        echo ""

        # Do NOT insert critical commands - user must type manually
        return 1
    fi

    # Check for risk warnings (format: "warning: message")
    if echo "$output" | grep -qi "^warning:"; then
        local warning=$(echo "$output" | grep -i "^warning:" | sed 's/warning: //I')
        local actual_command=$(echo "$output" | grep -v "^💡\|^Gathering\|^warning:\|^$" | tail -n 1)

        echo ""
        echo -e "\033[1;33m$warning\033[0m"
        echo ""
        echo -e "Command: \033[1;33m$actual_command\033[0m"
        echo ""

        # Still insert the command but user saw the warning
        print -z "$actual_command"
        return 0
    fi

    # Safe command - extract and display normally
    local command=$(echo "$output" | grep -v "^💡\|^Gathering\|^$" | tail -n 1)

    if [[ -z "$command" ]]; then
        echo "❌ No command generated"
        echo "$output"
        return 1
    fi

    # Display the generated command
    if [[ -n "$description" ]]; then
        echo "💡 $description"
        echo ""
    fi

    # Auto-insert the command into the terminal input line using print -z
    # This pushes the command onto the input stack, making it appear
    # in the terminal ready for the user to press Enter (or edit first)
    print -z "$command"

    # Return success - the command is now in the input line
    return 0
}

# Enable zsh completion for ez if available
compdef _command ez 2>/dev/null || true
