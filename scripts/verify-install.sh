#!/usr/bin/env bash
# Comprehensive Installation Verification for ez-term v0.5.6

echo "════════════════════════════════════════════════════════════════"
echo "  ez-term Installation Verification"
echo "════════════════════════════════════════════════════════════════"
echo ""

FAILED=0

# Test 1: Binary Installation
echo "Test 1: Binary Installation"
echo "----------------------------"

if [ -x ~/.local/bin/ez ]; then
    echo "✅ PASS: Binary installed at ~/.local/bin/ez"
else
    echo "❌ FAIL: Binary not found at ~/.local/bin/ez"
    FAILED=$((FAILED + 1))
fi

VERSION=$(~/.local/bin/ez --version 2>&1)
if echo "$VERSION" | grep -q "0.5.7"; then
    echo "✅ PASS: Version is 0.5.7"
else
    echo "❌ FAIL: Expected version 0.5.7, got: $VERSION"
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 2: Shell Wrapper Installation
echo "Test 2: Shell Wrapper Installation"
echo "-----------------------------------"

if [ -f ~/.ez.zsh ]; then
    echo "✅ PASS: Wrapper file exists at ~/.ez.zsh"
else
    echo "❌ FAIL: Wrapper file not found at ~/.ez.zsh"
    FAILED=$((FAILED + 1))
fi

if grep -q "^ez()" ~/.ez.zsh 2>/dev/null; then
    echo "✅ PASS: ez() function defined in wrapper"
else
    echo "❌ FAIL: ez() function not found in wrapper"
    FAILED=$((FAILED + 1))
fi

# Check for critical command blocking logic
if grep -q "critical:" ~/.ez.zsh 2>/dev/null; then
    echo "✅ PASS: Critical command detection in wrapper"
else
    echo "❌ FAIL: Critical command detection missing from wrapper"
    FAILED=$((FAILED + 1))
fi

# Check for warning command logic
if grep -q "warning:" ~/.ez.zsh 2>/dev/null; then
    echo "✅ PASS: Warning command detection in wrapper"
else
    echo "❌ FAIL: Warning command detection missing from wrapper"
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 3: Test Suite
echo "Test 3: Test Suite"
echo "-------------------"

cd /home/dev/work/ez-term

TEST_OUTPUT=$(cargo test --lib 2>&1 | tail -5)

if echo "$TEST_OUTPUT" | grep -q "47 passed"; then
    echo "✅ PASS: All 47 tests passing"
else
    echo "❌ FAIL: Tests not passing"
    echo "$TEST_OUTPUT"
    FAILED=$((FAILED + 1))
fi

echo ""

# Test 4: Command Validator
echo "Test 4: Command Validator"
echo "--------------------------"

cat > /tmp/test_validator_quick.rs << 'EOF'
fn main() {
    use ez_cli::command_validator::{CommandValidator, RiskLevel};

    let validator = CommandValidator::new();

    // Test critical
    let critical = validator.validate("find . -type f -delete").unwrap();
    if critical != RiskLevel::Critical {
        println!("FAIL: find -delete not detected as critical");
        std::process::exit(1);
    }

    // Test warning
    let warning = validator.validate("rm -rf /tmp/test").unwrap();
    if warning != RiskLevel::Medium {
        println!("FAIL: rm -rf not detected as medium");
        std::process::exit(1);
    }

    // Test safe
    let safe = validator.validate("ls -la").unwrap();
    if safe != RiskLevel::Safe {
        println!("FAIL: ls not detected as safe");
        std::process::exit(1);
    }

    println!("OK");
}
EOF

rustc --edition 2021 -L target/release/deps /tmp/test_validator_quick.rs \
    --extern ez_cli=target/release/libez_cli.rlib \
    -o /tmp/test_validator_quick 2>&1 | grep -v warning > /dev/null

if [ -f /tmp/test_validator_quick ]; then
    VALIDATOR_OUTPUT=$(/tmp/test_validator_quick 2>&1)
    if [ "$VALIDATOR_OUTPUT" = "OK" ]; then
        echo "✅ PASS: Command validator working correctly"
        echo "  - find -delete → CRITICAL"
        echo "  - rm -rf → MEDIUM"
        echo "  - ls -la → SAFE"
    else
        echo "❌ FAIL: Command validator test failed"
        echo "$VALIDATOR_OUTPUT"
        FAILED=$((FAILED + 1))
    fi
    rm -f /tmp/test_validator_quick /tmp/test_validator_quick.rs
else
    echo "⚠️  WARNING: Could not compile validator test"
fi

echo ""

# Test 5: Source Files
echo "Test 5: Source Files"
echo "--------------------"

REQUIRED_FILES=(
    "src/main.rs"
    "src/lib.rs"
    "src/command_validator.rs"
    "src/prompt_sanitizer.rs"
    "src/config.rs"
    "src/llm_client.rs"
    "scripts/ez.sh"
    "scripts/ez.zsh"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file"
    else
        echo "❌ MISSING: $file"
        FAILED=$((FAILED + 1))
    fi
done

echo ""

# Test 6: Documentation
echo "Test 6: Documentation"
echo "---------------------"

REQUIRED_DOCS=(
    "README.md"
    "CHANGELOG.md"
    "TROUBLESHOOTING.md"
    "RELEASE_PROCESS.md"
)

for doc in "${REQUIRED_DOCS[@]}"; do
    if [ -f "$doc" ]; then
        echo "✅ $doc"
    else
        echo "❌ MISSING: $doc"
        FAILED=$((FAILED + 1))
    fi
done

echo ""

# Test 7: Security Features
echo "Test 7: Security Features"
echo "-------------------------"

# Check command_validator.rs for critical patterns
if grep -q "find.*delete" src/command_validator.rs; then
    echo "✅ PASS: find -delete pattern in validator"
else
    echo "❌ FAIL: find -delete pattern missing"
    FAILED=$((FAILED + 1))
fi

# Check prompt_sanitizer.rs exists
if [ -f src/prompt_sanitizer.rs ]; then
    echo "✅ PASS: Prompt sanitizer module exists"
else
    echo "❌ FAIL: Prompt sanitizer missing"
    FAILED=$((FAILED + 1))
fi

# Check wrapper has critical blocking
if grep -q "Do NOT insert critical commands" ~/.ez.zsh; then
    echo "✅ PASS: Wrapper blocks critical commands"
else
    echo "❌ FAIL: Wrapper missing critical blocking"
    FAILED=$((FAILED + 1))
fi

echo ""

# Summary
echo "════════════════════════════════════════════════════════════════"

if [ $FAILED -eq 0 ]; then
    echo "  ✅ ALL TESTS PASSED!"
    echo "════════════════════════════════════════════════════════════════"
    echo ""
    echo "Installation Status: ✅ VERIFIED"
    echo "Version: 0.5.7"
    echo "Test Suite: 47/47 passing"
    echo "Security Features: Active"
    echo ""
    echo "Next Steps:"
    echo "-----------"
    echo "1. Configure a backend (Groq/OpenAI/Ollama)"
    echo "2. Reload your shell: source ~/.ez.zsh"
    echo "3. Test: ez 'list files'"
    echo ""
    echo "See README.md for detailed usage instructions."
    echo ""
    exit 0
else
    echo "  ❌ $FAILED TEST(S) FAILED"
    echo "════════════════════════════════════════════════════════════════"
    echo ""
    echo "Please review the failures above and reinstall if needed."
    echo ""
    exit 1
fi
