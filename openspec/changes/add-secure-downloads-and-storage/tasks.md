# Implementation Tasks

## Phase 1: Download Verification (P0/P1 - High Priority)

### CI/CD Updates

- [x] **Task 1.1**: Update `.github/workflows/release.yml` to generate SHA256 checksums
  - Add step after all binaries are built
  - Generate checksums.txt with format: `<sha256>  <filename>`
  - Include all .tar.gz and .zip artifacts
  - Upload checksums.txt as release asset
  - **Validation**: Manually trigger workflow, verify checksums file in release assets
  - **Dependencies**: None
  - **Estimated effort**: 1 hour

- [x] **Task 1.2**: Add checksum verification step to release workflow
  - Verify generated checksums.txt is valid before release
  - Ensure all expected artifacts have checksums
  - Fail release if checksum generation fails
  - **Validation**: Test with intentionally missing artifact
  - **Dependencies**: Task 1.1
  - **Estimated effort**: 30 minutes

### Code Implementation

- [x] **Task 1.3**: Add `sha2` dependency to Cargo.toml
  - Add: `sha2 = "0.10"`
  - Run `cargo build` to verify compilation
  - **Validation**: Build succeeds
  - **Dependencies**: None
  - **Estimated effort**: 5 minutes

- [x] **Task 1.4**: Create checksum verification module in `src/verification.rs`
  - Define `verify_checksum(file_path: &Path, expected: &str) -> Result<bool>`
  - Implement SHA256 calculation using `sha2` crate
  - Parse checksums.txt format
  - Add helper function to extract checksum for specific filename
  - **Validation**: Unit tests with test files
  - **Dependencies**: Task 1.3
  - **Estimated effort**: 2 hours

- [x] **Task 1.5**: Add checksum download to `src/update.rs`
  - Download checksums.txt from GitHub release
  - Store in temp directory alongside binary
  - Handle download errors gracefully
  - **Validation**: Unit test with mocked HTTP
  - **Dependencies**: Task 1.4
  - **Estimated effort**: 1 hour

- [x] **Task 1.6**: Integrate checksum verification into update flow
  - Add verification step after binary download
  - Extract expected checksum from checksums.txt
  - Calculate actual checksum of downloaded file
  - Compare and abort if mismatch
  - Display clear error messages
  - **Validation**: Integration test with valid/invalid checksums
  - **Dependencies**: Task 1.4, Task 1.5
  - **Estimated effort**: 2 hours

- [x] **Task 1.7**: Add size validation to download process
  - Check file size before processing
  - Reject 0-byte files
  - Reject files > 50MB
  - Display size-related errors
  - **Validation**: Test with edge cases
  - **Dependencies**: None (can parallelize)
  - **Estimated effort**: 30 minutes

- [x] **Task 1.8**: Improve error messages and user feedback
  - Add progress indicator for checksum download
  - Display "🔐 Verifying integrity..." during verification
  - Show "✅ Verification successful" on success
  - Clear error messages for each failure mode
  - **Validation**: Manual testing of all error paths
  - **Dependencies**: Task 1.6
  - **Estimated effort**: 1 hour

### Testing

- [x] **Task 1.9**: Add unit tests for checksum calculation
  - Test SHA256 calculation against known values
  - Test checksums.txt parsing
  - Test extraction of specific file checksums
  - Test error handling for malformed checksums
  - **Validation**: `cargo test verification` passes
  - **Dependencies**: Task 1.4
  - **Estimated effort**: 1 hour

- [x] **Task 1.10**: Add integration tests for update verification
  - Mock GitHub API responses
  - Test successful update with valid checksum
  - Test failed update with invalid checksum
  - Test failed update with missing checksums file
  - Test cleanup after verification failure
  - **Validation**: `cargo test update` passes
  - **Dependencies**: Task 1.6
  - **Estimated effort**: 2 hours

- [x] **Task 1.11**: Manual cross-platform testing
  - Test update on Linux (x86_64 and aarch64)
  - Test update on macOS (Intel and Apple Silicon)
  - Test update on Windows
  - Verify checksum verification works on all platforms
  - **Validation**: Checklist of platform tests completed
  - **Dependencies**: Task 1.6
  - **Estimated effort**: 2 hours

### Documentation

- [x] **Task 1.12**: Update UPDATE.md documentation
  - Document checksum verification process
  - Add manual verification instructions
  - Explain error messages
  - Add security best practices
  - **Validation**: Docs review
  - **Dependencies**: Task 1.6
  - **Estimated effort**: 1 hour

---

## Phase 2: Secure Credential Storage (P0/P1 - High Priority)

### Dependencies

- [x] **Task 2.1**: Add `keyring` dependency to Cargo.toml
  - Add: `keyring = "2.3"`
  - Research platform-specific requirements (Linux Secret Service)
  - Document optional dependencies
  - **Validation**: Build succeeds on all platforms
  - **Dependencies**: None
  - **Estimated effort**: 30 minutes

### Core Credential Module

- [x] **Task 2.2**: Create credential storage abstraction in `src/credentials.rs`
  - Define trait `CredentialStore` with methods: `store()`, `retrieve()`, `delete()`
  - Implement `KeyringStore` using `keyring` crate
  - Define service name: "ez-term"
  - Define account names: "groq_api_key", "openai_api_key"
  - **Validation**: Compiles, type-checks correctly
  - **Dependencies**: Task 2.1
  - **Estimated effort**: 2 hours

- [x] **Task 2.3**: Implement keyring storage operations
  - Implement `KeyringStore::store(key: &str, value: &str) -> Result<()>`
  - Implement `KeyringStore::retrieve(key: &str) -> Result<Option<String>>`
  - Implement `KeyringStore::delete(key: &str) -> Result<()>`
  - Handle platform-specific errors gracefully
  - **Validation**: Unit tests (may require mocking on CI)
  - **Dependencies**: Task 2.2
  - **Estimated effort**: 3 hours

- [x] **Task 2.4**: Implement credential priority logic
  - Create `get_credential(key: &str) -> Result<Option<String>>`
  - Check environment variables first
  - Check keyring second
  - Check config file third (with deprecation warning)
  - **Validation**: Unit tests with different priority scenarios
  - **Dependencies**: Task 2.3
  - **Estimated effort**: 1 hour

- [x] **Task 2.5**: Add fallback handling for missing keyring
  - Detect when keyring is unavailable (e.g., no Secret Service on Linux)
  - Return clear error with platform-specific suggestions
  - Provide instructions for environment variable setup
  - **Validation**: Test on system without keyring
  - **Dependencies**: Task 2.3
  - **Estimated effort**: 1 hour

### Migration Logic

- [x] **Task 2.6**: Implement plaintext key detection in `src/migration.rs`
  - Create `detect_plaintext_keys(config: &Config) -> Vec<String>`
  - Check for groq_api_key field
  - Check for openai_api_key field
  - Return list of keys found
  - **Validation**: Unit tests with different config states
  - **Dependencies**: None (can parallelize)
  - **Estimated effort**: 30 minutes

- [x] **Task 2.7**: Implement migration prompt and execution
  - Create `migrate_to_keyring(config: &mut Config) -> Result<()>`
  - Prompt user: "🔒 Migrate API keys to secure storage? [Y/n]"
  - On yes: move keys to keyring, remove from config, save config
  - On no: set migration_declined flag in config
  - Handle errors without data loss
  - **Validation**: Integration test with mock config
  - **Dependencies**: Task 2.3, Task 2.6
  - **Estimated effort**: 2 hours

- [x] **Task 2.8**: Add migration check to main.rs startup
  - Call migration check before command execution
  - Only prompt once (check for migration_declined flag)
  - Don't prompt if no plaintext keys found
  - Don't interrupt `ez init` or `ez --update` flows
  - **Validation**: Test various startup scenarios
  - **Dependencies**: Task 2.7
  - **Estimated effort**: 1 hour

### Config Module Updates

- [x] **Task 2.9**: Update `src/config.rs` to use credential module
  - Modify `get_groq_api_key()` to use new credential priority
  - Modify `get_openai_api_key()` to use new credential priority
  - Remove direct reading of API keys from config struct (deprecated)
  - Add migration_declined field to Config struct
  - **Validation**: Existing tests pass, new tests for credential retrieval
  - **Dependencies**: Task 2.4
  - **Estimated effort**: 2 hours

- [x] **Task 2.10**: Add config file permission enforcement (Unix)
  - Set permissions to 0600 when saving config
  - Check permissions when loading config
  - Warn if permissions are too permissive
  - **Validation**: Test on Linux/macOS
  - **Dependencies**: None (can parallelize)
  - **Estimated effort**: 1 hour

- [x] **Task 2.11**: Update Config::save() to exclude API keys
  - Remove groq_api_key and openai_api_key from serialization
  - Add comment: "# API keys stored securely in system keyring"
  - Ensure backward compatibility for reading old configs
  - **Validation**: Test config load/save cycle
  - **Dependencies**: Task 2.9
  - **Estimated effort**: 1 hour

### Setup Wizard Integration

- [x] **Task 2.12**: Update `src/setup.rs` to use keyring storage
  - Modify `configure_api_key()` to store in keyring
  - Handle keyring failures gracefully
  - Offer fallback to environment variables
  - Display success confirmation
  - **Validation**: Run `ez init` and verify key in keyring
  - **Dependencies**: Task 2.3
  - **Estimated effort**: 2 hours

- [x] **Task 2.13**: Add option to remove/reconfigure keys in setup wizard
  - Add "Reconfigure credentials" option in setup wizard
  - Allow deletion of keys from keyring
  - Confirm deletion to user
  - **Validation**: Manual test of credential removal
  - **Dependencies**: Task 2.12
  - **Estimated effort**: 1 hour

### Testing

- [x] **Task 2.14**: Add unit tests for credential storage
  - Test keyring store/retrieve (mocked)
  - Test credential priority logic
  - Test environment variable override
  - Test graceful degradation when keyring unavailable
  - **Validation**: `cargo test credentials` passes
  - **Dependencies**: Task 2.4
  - **Estimated effort**: 2 hours

- [x] **Task 2.15**: Add integration tests for migration
  - Test detection of plaintext keys
  - Test successful migration
  - Test migration decline
  - Test repeated runs (no re-prompting)
  - **Validation**: `cargo test migration` passes
  - **Dependencies**: Task 2.7
  - **Estimated effort**: 1 hour

- [x] **Task 2.16**: Add platform-specific integration tests
  - Test macOS Keychain integration
  - Test Linux Secret Service integration
  - Test Windows Credential Manager integration
  - Test fallback behavior on each platform
  - **Validation**: Platform CI tests pass
  - **Dependencies**: Task 2.3
  - **Estimated effort**: 3 hours

- [x] **Task 2.17**: Manual cross-platform testing
  - Run setup wizard on macOS, verify Keychain entry
  - Run setup wizard on Linux, verify Secret Service entry
  - Run setup wizard on Windows, verify Credential Manager entry
  - Test migration on each platform
  - Test without keyring available (Linux headless)
  - **Validation**: Checklist of platform tests completed
  - **Dependencies**: Task 2.12
  - **Estimated effort**: 3 hours

### Documentation

- [x] **Task 2.18**: Update README.md with keyring requirements
  - Document system requirements (Secret Service on Linux)
  - Explain credential storage approach
  - Add troubleshooting section
  - Document environment variable usage
  - **Validation**: Docs review
  - **Dependencies**: Task 2.12
  - **Estimated effort**: 1 hour

- [x] **Task 2.19**: Create SECURITY.md with best practices
  - Document credential storage security model
  - Explain migration process
  - Platform-specific security notes
  - Threat model and mitigations
  - **Validation**: Security review
  - **Dependencies**: All Phase 2 tasks
  - **Estimated effort**: 2 hours

---

## Phase 3: Optional Enhancements (P2 - Medium Priority)

### GPG Signature Verification

- [ ] **Task 3.1**: Research and select GPG library (optional)
  - Evaluate `gpgme`, `sequoia-openpgp`, or shell `gpg` command
  - Consider cross-platform compatibility
  - Document decision
  - **Validation**: Proof of concept
  - **Dependencies**: None
  - **Estimated effort**: 2 hours

- [ ] **Task 3.2**: Implement GPG signature verification (optional)
  - Add opt-in flag `--verify-signature` to update command
  - Download checksums.txt.asc from GitHub
  - Verify signature against checksums.txt
  - Fall back to checksum-only if GPG unavailable
  - **Validation**: Test with signed and unsigned releases
  - **Dependencies**: Task 3.1, Phase 1 complete
  - **Estimated effort**: 4 hours

### Release Signing

- [ ] **Task 3.3**: Set up GPG signing in CI (optional)
  - Generate or import release signing key
  - Add GPG_PRIVATE_KEY and GPG_PASSPHRASE secrets
  - Sign checksums.txt during release
  - Upload signature as release asset
  - **Validation**: Check release for .asc file
  - **Dependencies**: Task 3.1
  - **Estimated effort**: 2 hours

### Encrypted Config Fallback

- [ ] **Task 3.4**: Implement encrypted config file fallback (optional)
  - Add password prompt for config encryption
  - Use argon2 for key derivation
  - Use chacha20-poly1305 for encryption
  - Store encrypted blob in config
  - **Validation**: Test encryption/decryption cycle
  - **Dependencies**: Phase 2 complete
  - **Estimated effort**: 6 hours

---

## Final Integration & Validation

- [ ] **Task 4.1**: End-to-end integration testing
  - Fresh install with new secure features
  - Update from old version (test migration)
  - Test on all supported platforms
  - Verify all error paths
  - **Validation**: Complete test matrix
  - **Dependencies**: All Phase 1 and Phase 2 tasks
  - **Estimated effort**: 4 hours

- [ ] **Task 4.2**: Update SECURITY_AUDIT_REPORT.md
  - Re-run security audit with new features
  - Update risk scores
  - Mark resolved issues as fixed
  - Document remaining risks
  - **Validation**: Improved security score (target: 7+/10)
  - **Dependencies**: All implementation complete
  - **Estimated effort**: 2 hours

- [ ] **Task 4.3**: Version bump and release preparation
  - Bump version to 0.5.0 in Cargo.toml
  - Update CHANGELOG.md
  - Create migration guide for users
  - Tag release
  - **Validation**: Release checklist
  - **Dependencies**: All tasks complete, tests passing
  - **Estimated effort**: 1 hour

---

## Summary

**Total Tasks**: 47 (38 required, 9 optional)

**Estimated Effort**:
- Phase 1 (Download Verification): ~14 hours
- Phase 2 (Secure Storage): ~25 hours
- Phase 3 (Optional): ~14 hours
- Final Integration: ~7 hours
- **Total**: ~60 hours (~1.5-2 weeks for 1 developer)

**Parallelization Opportunities**:
- Tasks 1.3-1.7 can be worked on concurrently by different developers
- Tasks 2.1-2.6 can be parallelized
- Phase 1 and Phase 2 core modules can be developed in parallel
- Testing tasks can be done concurrently with implementation

**Critical Path**:
1. Task 1.1 → 1.4 → 1.6 (checksum verification)
2. Task 2.1 → 2.3 → 2.9 → 2.12 (secure storage)
3. Task 2.7 → 2.8 (migration)
4. Task 4.1 → 4.2 (integration & validation)
