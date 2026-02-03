# macOS Testing Guide

This guide covers how to build and test PingVoice Desktop on macOS before App Store submission.

## Prerequisites

### macOS Version

**Requires macOS 14.0 (Sonoma) or later.** This is needed for the `backgroundThrottling` WebKit feature that keeps audio playing when the app is hidden to the system tray.

### Xcode Command Line Tools

Required for compilers and macOS SDK:

```bash
xcode-select --install
```

### Rust and Cargo

Install Rust via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:

```bash
source ~/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

#### Adding build targets (optional)

To build for both Intel and Apple Silicon:

```bash
rustup target add x86_64-apple-darwin    # Intel
rustup target add aarch64-apple-darwin   # Apple Silicon
```

### Tauri CLI

Install the Tauri command line tool:

```bash
cargo install tauri-cli
```

Verify installation:

```bash
cargo tauri --version
```

### Summary

| Requirement | Details |
|-------------|---------|
| macOS Version | 14.0 (Sonoma) or later |
| Xcode CLI Tools | `xcode-select --install` |
| Rust/Cargo | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Tauri CLI | `cargo install tauri-cli` |

## Building the App

### Development build (with hot-reload)

```bash
cargo tauri dev
```

### Production build

```bash
# Build for current architecture
cargo tauri build

# Build for specific architecture
cargo tauri build --target aarch64-apple-darwin   # Apple Silicon (M1/M2/M3)
cargo tauri build --target x86_64-apple-darwin    # Intel Mac
```

Build output location: `target/release/bundle/macos/pingvoice.app`

## Verifying the Build

### Check entitlements are embedded

```bash
codesign -d --entitlements - target/release/bundle/macos/pingvoice.app
```

Expected output should include:
- `com.apple.security.app-sandbox`
- `com.apple.security.network.client`

### Check code signature

```bash
codesign -dv --verbose=4 target/release/bundle/macos/pingvoice.app
```

### Verify app bundle structure

```bash
ls -la target/release/bundle/macos/pingvoice.app/Contents/
```

## Running the App

### Run directly

```bash
open target/release/bundle/macos/pingvoice.app
```

### Run with logging

```bash
./target/release/bundle/macos/pingvoice.app/Contents/MacOS/pingvoice
```

### Run minimized (autostart mode)

```bash
open target/release/bundle/macos/pingvoice.app --args --minimized
```

## Functional Testing Checklist

### Core functionality

- [ ] App launches and loads the web URL correctly
- [ ] Window displays at correct size (1200x800)
- [ ] Window is resizable with minimum constraints (800x600)

### System tray

- [ ] Tray icon appears in menu bar
- [ ] Left-click toggles window visibility
- [ ] Right-click shows context menu
- [ ] "Show" menu item reveals window
- [ ] "Hide" menu item hides window
- [ ] "Quit" menu item exits the app

### Window behavior

- [ ] Close button hides to tray (doesn't quit)
- [ ] App stays running in background when window closed
- [ ] Window restores to previous position

### Autostart

- [ ] Autostart can be enabled in config
- [ ] App launches on login when enabled
- [ ] `--minimized` flag starts app hidden

### Sandbox testing

- [ ] Network requests work (web URL loads)
- [ ] No sandbox violations in Console.app

## Checking for Sandbox Violations

Open Console.app and filter for "pingvoice" or "sandbox" to check for violations:

```bash
log stream --predicate 'process == "pingvoice" OR message CONTAINS "sandbox"'
```

## Testing the DMG Installer

```bash
# Mount the DMG
open target/release/bundle/macos/pingvoice_1.0.1_aarch64.dmg

# Drag to Applications and test from there
open /Applications/pingvoice.app
```

## Troubleshooting

### App won't launch

Check for crashes in Console.app or run from terminal to see error output.

### Web URL doesn't load

Verify `config.json` has correct URL:
- macOS: `~/Library/Application Support/io.pingvoice/config.json`

### Tray icon missing

Ensure `icons/icon.png` exists and is valid.

### Entitlements not applied

Rebuild with `cargo tauri build` - entitlements are applied during the build process.
