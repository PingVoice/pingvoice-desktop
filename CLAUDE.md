# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Development with hot-reload
cargo tauri dev

# Production build for current platform
cargo tauri build

# Platform-specific builds
cargo tauri build --target x86_64-pc-windows-msvc    # Windows
cargo tauri build --target aarch64-apple-darwin      # macOS ARM
cargo tauri build --target x86_64-apple-darwin       # macOS Intel
```

Requires: Rust toolchain and `cargo install tauri-cli`

## Architecture

This is a Tauri 2.0 desktop application that wraps a configurable web URL with native desktop features (system tray, autostart). The Rust backend handles window management and system integration while the frontend is loaded from an external URL.

### Module Structure

- **`src/lib.rs`** - Application initialization, window setup, autostart configuration, and close-to-tray behavior. Supports `--minimized` flag for autostart scenarios.
- **`src/config.rs`** - JSON configuration management with platform-specific paths. Config stored at:
  - Windows: `%APPDATA%\com.notification-to-speech.app\config.json`
  - macOS: `~/Library/Application Support/com.notification-to-speech.app/config.json`
  - Linux: `~/.config/com.notification-to-speech.app/config.json`
- **`src/tray.rs`** - System tray menu (Show/Hide/Quit) with left-click toggle and right-click menu.
- **`capabilities/default.json`** - Tauri permission declarations for window management and autostart plugin.

### Configuration Schema

```json
{
  "url": "https://your-app-url.com",
  "autostart": true,
  "start_minimized": true
}
```

### Key Behaviors

- Close button hides to tray instead of quitting
- Window visibility toggled via tray icon left-click
- Autostart managed through `tauri-plugin-autostart`
