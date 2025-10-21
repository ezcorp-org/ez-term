# Design Document: Secure Downloads and Storage

## Overview
This document outlines the architectural decisions, trade-offs, and design patterns for implementing cryptographic download verification and secure credential storage.

---

## Architecture

### System Components

```
┌─────────────────────────────────────────────────────────────┐
│                        ez-term CLI                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐      ┌─────────────────┐                │
│  │   main.rs    │─────▶│  credentials.rs │                │
│  │              │      │  (new module)   │                │
│  │  - Startup   │      │                 │                │
│  │  - Migration │      │  ┌───────────┐  │                │
│  │    check     │      │  │ Keyring   │  │                │
│  └──────────────┘      │  │  Store    │  │                │
│         │              │  └───────────┘  │                │
│         │              │  ┌───────────┐  │                │
│         │              │  │  Env Var  │  │                │
│         │              │  │  Fallback │  │                │
│         │              │  └───────────┘  │                │
│         │              │  ┌───────────┐  │                │
│         │              │  │  Config   │  │                │
│         │              │  │ Fallback  │  │                │
│         │              │  │(deprecated)│  │                │
│         │              │  └───────────┘  │                │
│         │              └─────────────────┘                │
│         │                                                  │
│         ▼                                                  │
│  ┌──────────────┐      ┌─────────────────┐                │
│  │  config.rs   │      │ verification.rs │                │
│  │              │      │  (new module)   │                │
│  │  - Load/Save │      │                 │                │
│  │  - No longer │      │  ┌───────────┐  │                │
│  │    stores    │      │  │  SHA256   │  │                │
│  │    API keys  │      │  │Calculation│  │                │
│  └──────────────┘      │  └───────────┘  │                │
│         │              │  ┌───────────┐  │                │
│         │              │  │ Checksum  │  │                │
│         │              │  │  Parsing  │  │                │
│         │              │  └───────────┘  │                │
│         │              └─────────────────┘                │
│         │                       ▲                          │
│         ▼                       │                          │
│  ┌──────────────┐               │                          │
│  │  update.rs   │───────────────┘                          │
│  │              │                                          │
│  │  - Download  │                                          │
│  │  - Verify    │                                          │
│  │  - Install   │                                          │
│  └──────────────┘                                          │
│         │                                                  │
│         ▼                                                  │
│  ┌──────────────┐                                          │
│  │  setup.rs    │                                          │
│  │              │                                          │
│  │  - Uses      │                                          │
│  │    keyring   │                                          │
│  │    for keys  │                                          │
│  └──────────────┘                                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘
         │                            │
         ▼                            ▼
┌──────────────────┐        ┌────────────────────┐
│   OS Keyring     │        │  GitHub Releases   │
│                  │        │                    │
│  macOS: Keychain │        │  - Binary archives │
│  Linux: SecretSvc│        │  - checksums.txt   │
│  Win: CredMgr    │        │  - *.asc (optional)│
└──────────────────┘        └────────────────────┘
```

---

## Design Decisions

### 1. Credential Storage Strategy

#### Decision: OS Keyring with Layered Fallback

**Rationale**:
- **Primary (Keyring)**: Most secure, OS-managed, encrypted at rest
- **Secondary (Env Vars)**: Standard for CI/CD, temporary use
- **Tertiary (Config File)**: Deprecated, backward compatibility only

**Alternatives Considered**:
1. **Environment Variables Only**
   - ❌ Poor UX: requires manual shell config
   - ❌ Credentials visible in process list
   - ❌ Risk of exposure in shell history

2. **Encrypted Config File Only**
   - ❌ Requires master password (UX friction)
   - ❌ Key derivation complexity
   - ❌ Still a file that could be copied/leaked

3. **Remote Secrets Manager**
   - ❌ Contradicts "local-only" design principle
   - ❌ Adds external dependency
   - ❌ Network latency

**Trade-offs**:
- ✅ Best security for most users
- ✅ Graceful degradation when keyring unavailable
- ⚠️ Adds `keyring` dependency (~100KB)
- ⚠️ Linux requires Secret Service (gnome-keyring or kwallet)

---

### 2. Checksum Verification Approach

#### Decision: SHA256 with Optional GPG

**Rationale**:
- **SHA256**: Industry standard, collision-resistant, fast
- **GPG Optional**: Stronger security but requires user setup
- **HTTPS**: Protects checksums file from MITM

**Alternatives Considered**:
1. **MD5/SHA1 Only**
   - ❌ Cryptographically broken
   - ❌ Not acceptable for security

2. **GPG Required**
   - ❌ High barrier to entry
   - ❌ Requires key management
   - ❌ May not be installed

3. **No Verification**
   - ❌ Current vulnerability
   - ❌ Unacceptable for security

**Trade-offs**:
- ✅ Balance of security and usability
- ✅ SHA256 sufficient for most threat models
- ✅ GPG available for high-security environments
- ⚠️ Adds `sha2` dependency (~50KB)

---

### 3. Migration Strategy

#### Decision: Automatic Detection with User Consent

**Rationale**:
- **Automatic**: Detect plaintext keys on startup
- **User Consent**: Prompt before moving keys
- **One-time**: Remember user choice
- **Safe**: Never delete keys without successful migration

**Alternatives Considered**:
1. **Forced Migration**
   - ❌ Could break existing workflows
   - ❌ Poor user experience

2. **Manual Migration Only**
   - ❌ Most users won't migrate
   - ❌ Leaves vulnerability in place

3. **Silent Migration**
   - ❌ Unexpected behavior
   - ❌ Could confuse users

**Trade-offs**:
- ✅ Balances security and UX
- ✅ User maintains control
- ✅ Clear upgrade path
- ⚠️ Some users may decline (show warning)

---

### 4. Credential Priority Order

#### Decision: Env > Keyring > Config

**Rationale**:
1. **Environment Variables**: Highest priority for CI/CD and power users
2. **OS Keyring**: Default for interactive users
3. **Config File**: Deprecated, backward compatibility only

**Alternatives Considered**:
1. **Keyring > Env > Config**
   - ❌ Breaks CI/CD workflows expecting env vars

2. **Config > Keyring > Env**
   - ❌ Prioritizes least secure method

**Trade-offs**:
- ✅ Respects existing conventions
- ✅ Supports automation use cases
- ✅ Allows temporary overrides
- ✅ Clear precedence rules

---

### 5. Error Handling Philosophy

#### Decision: Fail Secure with Clear Guidance

**Rationale**:
- **Checksum Mismatch**: Abort update, delete file, explain
- **Keyring Unavailable**: Offer alternatives, don't block
- **Migration Failure**: Keep existing keys, don't lose data

**Principles**:
1. Never silently downgrade security
2. Provide actionable error messages
3. Suggest alternatives/workarounds
4. Fail safely (no data loss)

**Example Error Messages**:
```
❌ Checksum verification failed!
   Downloaded file may be corrupted or tampered with.

   Expected: a1b2c3d4...
   Actual:   e5f6g7h8...

   Suggestions:
   - Check your internet connection
   - Try again later
   - Verify GitHub is not having issues
   - Report if problem persists
```

---

## Implementation Patterns

### 1. Credential Module API

```rust
pub trait CredentialStore {
    fn store(&self, key: &str, value: &str) -> Result<()>;
    fn retrieve(&self, key: &str) -> Result<Option<String>>;
    fn delete(&self, key: &str) -> Result<()>;
}

pub struct KeyringStore {
    service: String,
}

impl KeyringStore {
    pub fn new(service: &str) -> Self {
        Self { service: service.to_string() }
    }
}

impl CredentialStore for KeyringStore {
    fn store(&self, key: &str, value: &str) -> Result<()> {
        let entry = keyring::Entry::new(&self.service, key)?;
        entry.set_password(value)?;
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Option<String>> {
        let entry = keyring::Entry::new(&self.service, key)?;
        match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn delete(&self, key: &str) -> Result<()> {
        let entry = keyring::Entry::new(&self.service, key)?;
        entry.delete_password()?;
        Ok(())
    }
}

pub fn get_credential(key: &str) -> Result<Option<String>> {
    // Priority 1: Environment variable
    if let Ok(value) = std::env::var(key.to_uppercase()) {
        return Ok(Some(value));
    }

    // Priority 2: Keyring
    let store = KeyringStore::new("ez-term");
    if let Some(value) = store.retrieve(key)? {
        return Ok(Some(value));
    }

    // Priority 3: Config file (deprecated, with warning)
    let config = Config::load()?;
    match key {
        "groq_api_key" => {
            if let Some(key) = config.groq_api_key {
                eprintln!("⚠️  Warning: API key stored in plaintext config (deprecated)");
                eprintln!("   Run 'ez init' to migrate to secure storage");
                return Ok(Some(key));
            }
        }
        "openai_api_key" => {
            if let Some(key) = config.openai_api_key {
                eprintln!("⚠️  Warning: API key stored in plaintext config (deprecated)");
                eprintln!("   Run 'ez init' to migrate to secure storage");
                return Ok(Some(key));
            }
        }
        _ => {}
    }

    Ok(None)
}
```

### 2. Checksum Verification Pattern

```rust
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn calculate_sha256(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

pub fn parse_checksums_file(content: &str) -> HashMap<String, String> {
    let mut checksums = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let checksum = parts[0];
            let filename = parts[1];
            checksums.insert(filename.to_string(), checksum.to_string());
        }
    }

    checksums
}

pub fn verify_checksum(file_path: &Path, expected: &str) -> Result<bool> {
    let actual = calculate_sha256(file_path)?;
    Ok(actual.eq_ignore_ascii_case(expected))
}
```

### 3. Migration Pattern

```rust
pub fn check_and_migrate() -> Result<()> {
    let config = Config::load()?;

    // Already migrated or declined
    if config.migration_completed || config.migration_declined {
        return Ok(());
    }

    // No keys to migrate
    if config.groq_api_key.is_none() && config.openai_api_key.is_none() {
        return Ok(());
    }

    // Prompt user
    println!("\n🔒 API keys found in plaintext config file");
    println!("   Migrate to secure system keyring?");

    let migrate = Confirm::new()
        .with_prompt("Migrate to secure storage?")
        .default(true)
        .interact()?;

    if !migrate {
        // User declined, remember choice
        let mut config = config;
        config.migration_declined = true;
        config.save()?;

        eprintln!("\n⚠️  Warning: API keys stored in plaintext (insecure)");
        eprintln!("   Consider using environment variables instead");
        return Ok(());
    }

    // Perform migration
    let store = KeyringStore::new("ez-term");
    let mut migrated_keys = Vec::new();

    if let Some(key) = &config.groq_api_key {
        store.store("groq_api_key", key)?;
        migrated_keys.push("Groq");
    }

    if let Some(key) = &config.openai_api_key {
        store.store("openai_api_key", key)?;
        migrated_keys.push("OpenAI");
    }

    // Verify storage before removing from config
    // (fail-safe: if verification fails, keys stay in config)
    if !migrated_keys.is_empty() {
        for key in ["groq_api_key", "openai_api_key"] {
            if store.retrieve(key)?.is_some() {
                // Successfully stored, remove from config
                let mut config = Config::load()?;
                match key {
                    "groq_api_key" => config.groq_api_key = None,
                    "openai_api_key" => config.openai_api_key = None,
                    _ => {}
                }
                config.migration_completed = true;
                config.save()?;
            }
        }

        println!("✅ Migrated {} API key(s) to secure storage", migrated_keys.len());
    }

    Ok(())
}
```

---

## Security Analysis

### Threat Model

| Threat | Without Fix | With Fix | Mitigation |
|--------|-------------|----------|------------|
| MITM on update | ❌ Vulnerable | ✅ Protected | SHA256 + HTTPS |
| Compromised release | ❌ Vulnerable | ✅ Detected | Checksum mismatch |
| Config file theft | ❌ API keys exposed | ✅ No keys in file | Keyring storage |
| Memory dump | ⚠️ Keys in memory | ⚠️ Keys in memory | Short-lived, OS-protected |
| Keylogger | ❌ Captures plaintext | ✅ One-time entry | OS keyring entry |
| Backup exposure | ❌ Keys in backups | ✅ Keyring excluded | OS keyring design |

### Defense in Depth

**Layer 1: Network Security**
- HTTPS for all downloads
- Certificate validation

**Layer 2: Cryptographic Verification**
- SHA256 checksums
- Optional GPG signatures

**Layer 3: Credential Protection**
- OS keyring encryption
- Environment variable support
- Config file permissions (0600)

**Layer 4: Migration Safety**
- User consent required
- Verify before delete
- No silent downgrades

---

## Performance Impact

### Checksum Verification
- **File Size**: 5-20MB (typical binary)
- **SHA256 Calculation**: ~50-200ms on modern hardware
- **Network Overhead**: <1KB (checksums.txt)
- **Total Impact**: <5% increase in update time
- **Acceptable**: Security benefit far outweighs cost

### Keyring Access
- **Initial Store**: ~10-50ms (one-time)
- **Retrieve**: ~10-50ms per credential
- **Caching**: Keep in memory during session
- **Total Impact**: <100ms added to startup
- **Acceptable**: Negligible compared to LLM latency

---

## Testing Strategy

### Unit Tests
- Checksum calculation (known values)
- Checksums file parsing
- Credential priority logic
- Migration detection

### Integration Tests
- Update with valid checksums
- Update with invalid checksums (should fail)
- Keyring store/retrieve cycle
- Migration flow end-to-end

### Platform Tests
- macOS Keychain
- Linux Secret Service (with/without daemon)
- Windows Credential Manager
- Permission enforcement

### Manual Tests
- Real update on each platform
- Setup wizard with keyring
- Migration from old version
- Error message clarity

---

## Rollout Plan

### Phase 1: Download Verification (Week 1)
- Implement and test checksum verification
- Update CI to generate checksums
- Release v0.5.0-beta1 for testing

### Phase 2: Secure Storage (Week 2)
- Implement keyring integration
- Add migration logic
- Release v0.5.0-beta2 for testing

### Phase 3: Polish & Release (Week 3)
- Address beta feedback
- Complete documentation
- Full platform testing
- Release v0.5.0

### Phase 4: Monitor & Iterate
- Track migration success rate
- Collect feedback on error messages
- Address platform-specific issues
- Consider GPG signatures based on demand

---

## Future Enhancements

### Possible Additions
1. **Audit Logging**: Log credential access for forensics
2. **Multi-Factor**: Require additional auth for sensitive operations
3. **Credential Rotation**: Prompt to rotate old keys
4. **Encrypted Backup**: Secure export/import of credentials
5. **Team Sharing**: Shared keyring for organizational use

### Out of Scope
- Cloud-based secrets management (contradicts local-only)
- Hardware security modules (overkill for CLI tool)
- Blockchain-based verification (unnecessary complexity)

---

## Conclusion

This design provides a pragmatic balance between security and usability:

- **Checksum verification** eliminates the critical update vulnerability
- **Keyring storage** protects API keys without UX friction
- **Graceful fallback** ensures functionality on all platforms
- **Clear migration path** upgrades existing users safely
- **Maintainable architecture** follows Rust best practices

The implementation will significantly improve the security posture (from 4.3/10 to estimated 7+/10) while maintaining the user experience and local-only philosophy of ez-term.
