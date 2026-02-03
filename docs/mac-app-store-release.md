# Mac App Store Release Guide

This guide covers the process of releasing PingVoice Desktop to the Mac App Store.

## Prerequisites

1. **Apple Developer Program membership** ($99/year) - Required for App Store distribution
2. **Xcode installed** - Needed for code signing tools and Transporter

## Steps

### 1. Create certificates in Apple Developer Portal

- Go to [developer.apple.com](https://developer.apple.com) → Certificates, Identifiers & Profiles
- Create a **Mac App Distribution** certificate
- Create an **App Store Connect** distribution certificate (for uploading)
- Create an **App ID** with bundle identifier `io.pingvoice`

### 2. Create app record in App Store Connect

- Go to [appstoreconnect.apple.com](https://appstoreconnect.apple.com)
- Create a new macOS app with bundle ID `io.pingvoice`
- Fill in metadata: description, screenshots, categories, privacy policy URL, etc.

### 3. Update signing identity

In `tauri.conf.json`, replace the ad-hoc signing:

```json
"signingIdentity": "3rd Party Mac Developer Application: Your Name (TEAM_ID)"
```

### 4. Build the app

```bash
cargo tauri build --target aarch64-apple-darwin   # ARM Mac
cargo tauri build --target x86_64-apple-darwin    # Intel Mac
# Or build universal binary for both
```

### 5. Create installer package

App Store requires a `.pkg` installer, not `.app`:

```bash
productbuild --component "target/release/bundle/macos/pingvoice.app" /Applications \
  --sign "3rd Party Mac Developer Installer: Your Name (TEAM_ID)" \
  pingvoice.pkg
```

### 6. Upload to App Store Connect

Use **Transporter** (free from Mac App Store) or `xcrun altool`:

```bash
xcrun altool --upload-app --type macos --file pingvoice.pkg \
  --apiKey YOUR_KEY --apiIssuer YOUR_ISSUER_ID
```

### 7. Submit for review

In App Store Connect, select the uploaded build and submit for Apple's review.

## Key Considerations

- Apple reviews can take 1-7 days
- You'll need a privacy policy URL
- Screenshots are required (1280x800 or 2560x1600)
- The sandbox entitlements in `entitlements.plist` satisfy App Store requirements

## Configuration Files

| File | Purpose |
|------|---------|
| `entitlements.plist` | Sandbox + network entitlements for App Store |
| `tauri.conf.json` | References entitlements, enables hardened runtime |

## Verification

Before submitting, verify entitlements are embedded:

```bash
codesign -d --entitlements - target/release/bundle/macos/pingvoice.app
```
