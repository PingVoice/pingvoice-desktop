# Ping Voice Desktop

A lightweight Tauri desktop wrapper that loads a configurable web URL with system tray support.

## Features

- System tray with show/hide/quit controls
- Click tray icon to toggle window visibility
- Autostart on system boot (optional)
- Start minimized to tray
- Close to tray instead of quitting

## Configuration

Create a `config.json` in the app's config directory:

**Windows:** `%APPDATA%\com.notification-to-speech.app\config.json`
**macOS:** `~/Library/Application Support/com.notification-to-speech.app/config.json`
**Linux:** `~/.config/com.notification-to-speech.app/config.json`

```json
{
  "url": "https://your-app-url.com",
  "autostart": true,
  "start_minimized": true
}
```

| Option | Default | Description |
|--------|---------|-------------|
| `url` | `http://localhost` | The web URL to load |
| `autostart` | `false` | Start app on system boot |
| `start_minimized` | `false` | Start hidden in system tray |

## Building

Requires [Rust](https://rustup.rs/) and the Tauri CLI.

```bash
# Install Tauri CLI
cargo install tauri-cli

# Build for current platform
cargo tauri build

# Build for specific targets
cargo tauri build --target x86_64-pc-windows-msvc    # Windows
cargo tauri build --target aarch64-apple-darwin      # macOS ARM
cargo tauri build --target x86_64-apple-darwin       # macOS Intel
```

## Development

```bash
cargo tauri dev
```

## Build Output

- **Windows:** `target/release/bundle/nsis/` - NSIS installer
- **macOS:** `target/release/bundle/dmg/` - DMG disk image
