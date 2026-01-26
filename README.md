# PingVoice Desktop

The official desktop app for [PingVoice](https://pingvoice.io) — turn API calls into instant spoken notifications.

PingVoice lets your systems speak. Send a POST request to the API, and hear it announced in real-time. Built for developers who want audio alerts for CI/CD pipelines, AI agents, server monitoring, and long-running jobs.

## What is PingVoice?

PingVoice is a text-to-speech API notification service. Send a POST request to the API, and the message is converted to speech and delivered instantly via WebSocket to your browser or this desktop app.

**Why audio?** Sound reaches you when your eyes are elsewhere. No more watching terminals or refreshing dashboards.

### Use Cases

- **CI/CD Pipelines** — Know when builds and deploys finish
- **AI Coding Assistants** — Get notified when Claude Code, Cursor, or Copilot need input
- **Long-Running Jobs** — Step away while scripts, ETL jobs, or data processing runs
- **Server Monitoring** — Hear critical alerts without dashboard fatigue

### Platform Availability

| Platform | Status |
|----------|--------|
| Windows | Available |
| macOS | Coming soon |

Learn more at [pingvoice.io](https://pingvoice.io)

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
