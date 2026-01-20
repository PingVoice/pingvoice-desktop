# PingVoice Desktop

The official desktop app for [PingVoice](https://pingvoice.io) — turn API calls into instant spoken notifications.

PingVoice lets your systems speak. Send a POST request to the API, and hear it announced in real-time. Built for developers who want audio alerts for CI/CD pipelines, AI agents, server monitoring, and long-running jobs.

## Features

- System tray with show/hide/quit controls
- Click tray icon to toggle window visibility
- Autostart on system boot (optional)
- Start minimized to tray
- Close to tray instead of quitting

## Configuration

Create a `config.json` in the app's config directory:

**Windows:** `%APPDATA%\io.pingvoice\config.json`
**macOS:** `~/Library/Application Support/io.pingvoice/config.json`
**Linux:** `~/.config/io.pingvoice/config.json`

```json
{
  "url": "https://pingvoice.io",
  "autostart": true,
  "start_minimized": true
}
```

| Option | Default | Description |
|--------|---------|-------------|
| `url` | `https://pingvoice.io` | The web URL to load |
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
