# Download Verification Capability

## Overview
Cryptographic verification of downloaded binaries to ensure authenticity and integrity during the update process.

## ADDED Requirements

### Requirement: SHA256 Checksum Generation
The release CI/CD pipeline MUST generate SHA256 checksums for all release artifacts.

#### Scenario: Generate checksums for all platforms
- **WHEN** the release workflow builds binaries for all platforms
- **THEN** a checksums.txt file MUST be generated containing one line per artifact with format: `<sha256>  <filename>`
- **AND** the file MUST cover all .tar.gz and .zip files
- **AND** the checksums file MUST be uploaded as a release asset

#### Scenario: Checksums file is published
- **WHEN** a new release is created
- **THEN** the checksums.txt file MUST be available at: `https://github.com/{repo}/releases/download/{version}/checksums.txt`

---

### Requirement: Download Checksum Verification
The update command MUST verify SHA256 checksums of downloaded archives before extraction.

#### Scenario: Successful checksum verification
- **WHEN** a user runs `ez --update` and the binary archive is downloaded
- **THEN** the update process MUST download the checksums.txt file
- **AND** extract the expected checksum for the platform
- **AND** calculate SHA256 of the downloaded archive
- **AND** compare calculated vs expected checksum
- **AND** proceed with extraction only if checksums match

#### Scenario: Checksum mismatch detected
- **WHEN** a user runs `ez --update` and the downloaded archive's checksum does NOT match the published checksum
- **THEN** the update process MUST display error: "❌ Checksum verification failed! Downloaded file may be corrupted or tampered with."
- **AND** delete the downloaded archive
- **AND** NOT proceed with extraction or installation
- **AND** exit with non-zero status code
- **AND** suggest manual download and verification

#### Scenario: Checksums file unavailable
- **WHEN** a user runs `ez --update` and the checksums.txt file cannot be downloaded
- **THEN** the update process MUST display error: "❌ Could not download checksums for verification"
- **AND** abort the update process
- **AND** NOT proceed with installation
- **AND** suggest checking internet connection or GitHub status

---

### Requirement: Size Validation
The update command MUST validate file size before processing.

#### Scenario: File size validation
- **WHEN** a binary archive is being downloaded and the download completes
- **THEN** the update process MUST check that file size is within reasonable bounds (1MB - 50MB)
- **AND** reject files that are 0 bytes
- **AND** reject files larger than 50MB
- **AND** display clear error for size issues

---

### Requirement: GPG Signature Verification
The update command SHALL support optional GPG signature verification for enhanced security.

#### Scenario: GPG signature available and verification enabled
- **WHEN** the release includes a checksums.txt.asc GPG signature and the user has gpg installed with the release signing key imported
- **THEN** the update process MAY download checksums.txt.asc
- **AND** verify the signature against checksums.txt
- **AND** warn if signature verification fails
- **AND** allow opt-in to require valid signatures

#### Scenario: GPG not available
- **WHEN** GPG signature verification is requested but gpg is not installed or signing key not imported
- **THEN** the update process MUST display informational message about GPG verification
- **AND** fall back to checksum-only verification
- **AND** NOT block the update process

---

### Requirement: Verification Progress Feedback
The update process MUST provide clear feedback during verification.

#### Scenario: Verification status messages
- **WHEN** a user runs `ez --update` and verification steps are performed
- **THEN** the update process MUST display "📥 Downloading checksums..." when fetching checksums.txt
- **AND** display "🔐 Verifying integrity..." when calculating checksums
- **AND** display "✅ Verification successful" when checksums match
- **AND** show progress indicators for long-running operations

---

### Requirement: Temporary File Cleanup
The update process MUST clean up temporary files on verification failure.

#### Scenario: Cleanup after verification failure
- **WHEN** checksum verification fails and the update process aborts
- **THEN** the update process MUST delete the downloaded archive
- **AND** delete the checksums.txt file
- **AND** remove the temporary directory
- **AND** leave no partial downloads on disk

#### Scenario: Cleanup after successful update
- **WHEN** the update completes successfully and the binary has been replaced
- **THEN** the update process MUST delete the downloaded archive
- **AND** delete the checksums.txt file
- **AND** delete the extracted binary
- **AND** remove the temporary directory

---

## Testing Requirements

### Unit Tests
- SHA256 calculation for test files
- Checksum parsing from checksums.txt format
- Size validation logic
- Error handling for network failures

### Integration Tests
- Full update flow with valid checksums
- Update flow with invalid checksums (should fail)
- Update flow with missing checksums file (should fail)
- Update flow with corrupted download (should fail)

### Manual Tests
- Test update on each platform (Linux, macOS, Windows)
- Test with slow network connection
- Test with interrupted download
- Verify error messages are clear and actionable

---

## Security Considerations

### Threat Model
- **MITM Attack**: Checksums verified over HTTPS, attacker cannot modify both binary and checksum without certificate compromise
- **Compromised Mirror**: N/A, downloads are from GitHub only
- **DNS Poisoning**: HTTPS with certificate validation mitigates
- **Compromised Release Infrastructure**: Checksums help detect, GPG signatures provide stronger protection

### Defense in Depth
1. **HTTPS**: All downloads over encrypted connections
2. **Checksums**: Detect tampering or corruption
3. **GPG Signatures**: Optional cryptographic proof of authenticity
4. **Size Validation**: Detect obviously wrong downloads early

---

## Performance Considerations
- Checksum calculation adds ~100-500ms for typical binary sizes (5-20MB)
- Checksums file is small (<1KB), negligible download time
- Overall impact on update time: <5%
- Trade-off acceptable for security benefit

---

## Dependencies
- `sha2` crate for SHA256 calculation
- Optional: `gpgme` or similar for GPG verification
- Existing: `reqwest` for downloads
- Existing: `anyhow` for error handling
