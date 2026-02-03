# Notarized Distribution Guide

This guide covers how to distribute PingVoice Desktop outside the Mac App Store with Apple notarization. Notarized apps open without Gatekeeper warnings, providing a smooth user experience.

## Overview

| Distribution Method | Pros | Cons |
|---------------------|------|------|
| **Mac App Store** | Easy discovery, automatic updates, trusted | Apple review, IAP requirements, 15-30% cut |
| **Notarized (this guide)** | No review process, no IAP rules, direct distribution | Requires Developer ID, manual update mechanism |
| **Unsigned** | No Apple account needed | Users must right-click → Open, security warnings |

## Prerequisites

1. **Apple Developer Program membership** ($99/year)
2. **Xcode installed** - For `codesign`, `notarytool`, and `stapler` commands
3. **Developer ID Application certificate** - Different from Mac App Store certificates

## Step 1: Create Developer ID Certificate

### In Apple Developer Portal

1. Go to [developer.apple.com](https://developer.apple.com) → Certificates, Identifiers & Profiles
2. Click **Certificates** → **+** (Create new)
3. Select **Developer ID Application** (not "Mac App Distribution")
4. Follow the prompts to create a Certificate Signing Request (CSR) from Keychain Access
5. Download and double-click to install in Keychain Access

### Export as .p12 (for CI/CD)

1. Open **Keychain Access**
2. Find your "Developer ID Application" certificate
3. Right-click → **Export**
4. Save as `.p12` format with a strong password
5. Keep this file secure — it can sign apps as you

## Step 2: Create App-Specific Password

Apple's notary service requires authentication:

1. Go to [appleid.apple.com](https://appleid.apple.com)
2. Sign in → **Security** → **App-Specific Passwords**
3. Click **Generate an app-specific password**
4. Name it "PingVoice Notarization" (or similar)
5. Save the generated password securely

## Step 3: Find Your Team ID

Your Team ID is a 10-character identifier:

1. Go to [developer.apple.com](https://developer.apple.com) → Account
2. Find **Team ID** in the membership details
3. Or run: `security find-identity -v -p codesigning` and look at certificate names

## Step 4: Configure GitHub Secrets

Add these secrets to your repository (Settings → Secrets and variables → Actions):

| Secret Name | Value | How to Get It |
|-------------|-------|---------------|
| `APPLE_CERTIFICATE` | Base64-encoded .p12 file | `base64 -i certificate.p12 \| pbcopy` |
| `APPLE_CERTIFICATE_PASSWORD` | Password for .p12 file | The password you set during export |
| `APPLE_ID` | Your Apple ID email | Your developer account email |
| `APPLE_PASSWORD` | App-specific password | From Step 2 |
| `APPLE_TEAM_ID` | 10-character Team ID | From Step 3 |

### Encoding the Certificate

```bash
# Encode your .p12 certificate to base64
base64 -i /path/to/certificate.p12 | pbcopy

# The base64 string is now in your clipboard
# Paste it as the APPLE_CERTIFICATE secret value
```

## Step 5: Update tauri.conf.json

Change the signing identity from ad-hoc to your Developer ID:

```json
"macOS": {
  "minimumSystemVersion": "14.0",
  "entitlements": "entitlements.plist",
  "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
  "hardenedRuntime": true
}
```

Replace `Your Name (TEAM_ID)` with your actual certificate name. Find it with:

```bash
security find-identity -v -p codesigning | grep "Developer ID Application"
```

## Step 6: Update CI/CD Workflow

Add these steps to `.github/workflows/build.yml` for the macOS build job:

### Import Certificate Step

Add before the build step:

```yaml
- name: Import Apple certificate
  if: matrix.os == 'macos-latest'
  env:
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
  run: |
    # Decode certificate
    echo "$APPLE_CERTIFICATE" | base64 --decode > certificate.p12

    # Create temporary keychain
    security create-keychain -p "temppassword" build.keychain
    security default-keychain -s build.keychain
    security unlock-keychain -p "temppassword" build.keychain

    # Import certificate
    security import certificate.p12 -k build.keychain \
      -P "$APPLE_CERTIFICATE_PASSWORD" -T /usr/bin/codesign

    # Allow codesign to access keychain
    security set-key-partition-list -S apple-tool:,apple:,codesign: \
      -s -k "temppassword" build.keychain

    # Clean up certificate file
    rm certificate.p12
```

### Notarization Step

Add after the build step:

```yaml
- name: Notarize macOS app
  if: matrix.os == 'macos-latest'
  env:
    APPLE_ID: ${{ secrets.APPLE_ID }}
    APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
    APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
  run: |
    # Find the DMG
    DMG_PATH=$(find target -name "*.dmg" | head -1)
    echo "Notarizing: $DMG_PATH"

    # Submit for notarization and wait
    xcrun notarytool submit "$DMG_PATH" \
      --apple-id "$APPLE_ID" \
      --password "$APPLE_PASSWORD" \
      --team-id "$APPLE_TEAM_ID" \
      --wait

    # Staple the notarization ticket to the DMG
    xcrun stapler staple "$DMG_PATH"

    echo "Notarization complete!"
```

## Step 7: Local Notarization (Manual)

For testing or one-off releases without CI/CD:

### Sign the App

```bash
# Build the app
cargo tauri build --target aarch64-apple-darwin

# Verify it's signed with Developer ID
codesign -dv --verbose=4 target/release/bundle/macos/pingvoice.app
```

### Submit for Notarization

```bash
# For DMG
xcrun notarytool submit target/release/bundle/dmg/pingvoice_*.dmg \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait

# For .app directly (zip it first)
ditto -c -k --keepParent target/release/bundle/macos/pingvoice.app pingvoice.zip
xcrun notarytool submit pingvoice.zip \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait
```

### Staple the Ticket

```bash
# Staple to DMG
xcrun stapler staple target/release/bundle/dmg/pingvoice_*.dmg

# Or staple to .app
xcrun stapler staple target/release/bundle/macos/pingvoice.app
```

### Verify Notarization

```bash
# Check if notarization ticket is stapled
xcrun stapler validate target/release/bundle/dmg/pingvoice_*.dmg

# Check Gatekeeper acceptance
spctl --assess --type open --context context:primary-signature -v \
  target/release/bundle/macos/pingvoice.app
```

## Troubleshooting

### "The signature is invalid"

The app may not be properly signed. Verify:

```bash
codesign --verify --deep --strict target/release/bundle/macos/pingvoice.app
```

### Notarization fails with "Invalid Signature"

Ensure hardened runtime is enabled in `tauri.conf.json`:

```json
"hardenedRuntime": true
```

### "Unable to find credentials"

Check that all environment variables are set:
- `APPLE_ID` - Your Apple ID email
- `APPLE_PASSWORD` - App-specific password (not your account password)
- `APPLE_TEAM_ID` - 10-character team identifier

### Check notarization status

```bash
xcrun notarytool log <submission-id> \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID"
```

### Certificate not found in CI

Ensure the certificate is correctly base64 encoded without line breaks:

```bash
# Encode without line breaks
base64 -i certificate.p12 | tr -d '\n' | pbcopy
```

## Distribution

Once notarized, you can distribute the DMG via:

- **GitHub Releases** - Automated via the existing workflow
- **Direct download** - Host on your website
- **Homebrew Cask** - Create a cask formula

Users can download and open the app normally — no Gatekeeper warnings.

## Comparison: App Store vs Notarized

| Aspect | App Store | Notarized |
|--------|-----------|-----------|
| Apple review | Required (1-7 days) | None |
| In-App Purchase rules | Required for subscriptions | Not applicable |
| Distribution | App Store only | Anywhere |
| Auto-updates | Built-in | Must implement yourself |
| User trust | Highest | High (Apple verified) |
| Revenue share | 15-30% to Apple | None |

## Configuration Reference

| File | Purpose |
|------|---------|
| `tauri.conf.json` | Signing identity, hardened runtime |
| `entitlements.plist` | App capabilities (sandbox, network) |
| `.github/workflows/build.yml` | CI/CD with signing and notarization |
