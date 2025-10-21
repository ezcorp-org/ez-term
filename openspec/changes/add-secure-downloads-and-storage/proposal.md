# Proposal: Add Secure Downloads and Storage

## Change ID
`add-secure-downloads-and-storage`

## Summary
Add cryptographic verification for network downloads (checksums and optional GPG signatures) and implement secure API key storage using OS-native credential managers (keyring/keychain) instead of plaintext storage in config files.

## Why
The security audit (SECURITY_AUDIT_REPORT.md) identified two critical vulnerabilities that expose users to significant risk:

1. **Unvalidated Network Downloads (HIGH Priority, Finding #3)**: The `ez --update` command downloads binaries from GitHub with zero integrity checks. An attacker performing a man-in-the-middle attack, DNS poisoning, or compromising the release infrastructure could serve malicious binaries that would be installed and executed without verification. This creates a direct path to remote code execution.

2. **Plaintext API Key Storage (MEDIUM Priority, Finding #4)**: API keys for cloud LLM providers (Groq, OpenAI) are stored in plaintext in `~/.config/ez-term/config.toml`. These keys can be stolen through filesystem access, accidental backup/sharing, or malware. Once stolen, attackers can abuse the API keys for unauthorized access, incurring costs or accessing user data.

These vulnerabilities contradict the application's privacy-first and security-conscious design principles. Fixing them will improve the overall security score from 4.3/10 (MEDIUM risk) to an estimated 7+/10 (LOW risk), making ez-term suitable for broader adoption including security-aware users and enterprise environments.

The proposed changes align with industry best practices:
- Checksum verification is standard for software distribution (apt, brew, cargo)
- OS keyring storage is the recommended approach for desktop applications (Chrome, VS Code, Git)
- The migration path ensures existing users can upgrade safely without breaking workflows

## Problem Statement
The current implementation has two critical security vulnerabilities identified in the security audit:

1. **Unvalidated Network Downloads (HIGH)**: The update mechanism downloads binaries from GitHub with zero integrity verification. This leaves users vulnerable to:
   - Man-in-the-middle attacks
   - DNS poisoning
   - Compromised release infrastructure
   - Downloaded corrupted or malicious binaries

2. **Plaintext API Key Storage (MEDIUM)**: API keys for Groq and OpenAI are stored in plaintext in `~/.config/ez-term/config.toml`, making them vulnerable to:
   - Filesystem compromise
   - Accidental disclosure (config backup, version control)
   - Malware scanning for common config paths
   - Local privilege escalation attacks

## Proposed Solution

### 1. Download Verification
Implement a multi-layered verification system for binary downloads:

- **SHA256 Checksums** (Required): Verify all downloaded archives against published checksums
- **GPG Signatures** (Optional): Allow verification of signed releases for maximum security
- **Size Validation**: Verify file size matches expected before extraction
- **HTTPS Enforcement**: Already implemented, but maintain strict HTTPS-only policy

### 2. Secure Credential Storage
Implement OS-native credential storage with graceful fallback:

- **Primary**: Use OS keyring/keychain for API key storage
  - macOS: Keychain
  - Linux: Secret Service API (GNOME Keyring, KDE Wallet)
  - Windows: Windows Credential Manager
- **Fallback**: Encrypted config file with user-provided master password
- **Migration**: Automatically migrate plaintext keys on first run with new version
- **Environment Variables**: Continue to support env vars as override

## Affected Components

### Source Code
- `src/update.rs` - Add checksum and signature verification
- `src/config.rs` - Add secure credential storage backend
- `src/setup.rs` - Update wizard to use secure storage
- `Cargo.toml` - Add dependencies: `sha2`, `keyring`, optional `gpgme`

### CI/CD
- `.github/workflows/release.yml` - Generate SHA256 checksums and publish
- `.github/workflows/release.yml` - Optional GPG signing of releases

### Documentation
- Update installation docs with verification instructions
- Add security best practices guide
- Document credential storage behavior

## User Impact

### Positive
- **Security**: Protection against malware and stolen credentials
- **Trust**: Cryptographic proof of authenticity
- **Compliance**: Better alignment with enterprise security policies
- **Privacy**: API keys no longer visible in filesystem

### Breaking Changes
- **Config File Format**: API keys will be removed from `config.toml` after migration
- **First Run**: Users will be prompted to migrate existing plaintext keys
- **Dependencies**: New system requirements for keyring libraries (optional on Linux)

### Migration Path
1. On first run with new version, detect plaintext keys in config
2. Prompt user: "Migrate to secure storage? [Y/n]"
3. If yes: Move keys to keyring and remove from config file
4. If no: Warn about security risk, continue with plaintext (deprecated)
5. Set migration flag to avoid repeated prompts

## Implementation Phases

### Phase 1: Download Verification (P1 - High Priority)
- Add SHA256 checksum generation to CI
- Implement checksum verification in update.rs
- Add tests for verification logic
- Update docs

### Phase 2: Secure Storage (P1 - High Priority)
- Add keyring dependency
- Implement credential storage abstraction
- Add migration logic
- Update setup wizard
- Add tests

### Phase 3: Optional Enhancements (P2 - Medium Priority)
- GPG signature support
- Encrypted config file fallback
- Audit logging for credential access
- Config file permission enforcement (0600)

## Success Criteria
- [ ] All downloads verified with SHA256 checksums
- [ ] Invalid checksums rejected with clear error message
- [ ] API keys stored in OS keyring by default
- [ ] Existing keys migrated automatically
- [ ] No regression in setup wizard UX
- [ ] 100% test coverage for verification logic
- [ ] Security audit score improved from 4.3/10 to 7+/10
- [ ] Documentation updated with security best practices

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Keyring unavailable on some Linux distros | Medium | Provide encrypted file fallback |
| Migration breaks existing workflows | Low | Make migration optional with warnings |
| Checksum file compromised | Medium | Add optional GPG signature verification |
| CI fails to generate checksums | High | Add validation step before release |
| Cross-platform keyring issues | Medium | Extensive testing, graceful degradation |

## Alternatives Considered

### 1. Environment Variables Only
- **Pro**: Simple, no storage needed
- **Con**: Poor UX, credentials in shell history/process lists

### 2. Encrypted Config File Only
- **Pro**: Portable, no OS dependencies
- **Con**: Requires master password, key derivation complexity

### 3. No Verification (Status Quo)
- **Pro**: Zero effort
- **Con**: Leaves critical security vulnerability unfixed

## Open Questions
- Should GPG signing be required or optional in Phase 1?
- Should we support multiple keyring backends or use `keyring` crate's abstraction?
- Should migration be mandatory or optional with deprecation warning?
- Should we maintain backward compatibility with plaintext keys (discouraged)?

## Related Changes
- Addresses HIGH priority finding #3 from security audit
- Addresses MEDIUM priority finding #4 from security audit
- Improves overall security score from 4.3/10 to estimated 7+/10
- Complements existing HTTPS-only networking

## Timeline
- **Phase 1 (Download Verification)**: 1 week
- **Phase 2 (Secure Storage)**: 1 week
- **Phase 3 (Optional Enhancements)**: 2 weeks
- **Total**: ~4 weeks for full implementation
