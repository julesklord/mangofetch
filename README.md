<p align="center">
  <img src="static/loop.png" alt="OmniGet CLI" width="100" />
</p>

<h1 align="center">omniget-cli</h1>

<p align="center">
  <strong>Command-line companion to OmniGet.</strong><br>
  Download media, courses, and files from 1000+ sites — directly from your terminal.
</p>

<p align="center">
  <a href="https://github.com/julesklord/omniget-cli/releases/latest"><img src="https://img.shields.io/github/v/release/julesklord/omniget-cli?style=for-the-badge&label=release" alt="Latest Release" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-green?style=for-the-badge" alt="License GPL-3.0" /></a>
  <a href="https://github.com/julesklord/omniget-cli/stargazers"><img src="https://img.shields.io/github/stars/julesklord/omniget-cli?style=for-the-badge" alt="Stars" /></a>
  <img src="https://img.shields.io/badge/rust-1.70%2B-orange?style=for-the-badge&logo=rust" alt="Rust 1.70+" />
</p>

<p align="center">
  <img src="demo.gif" alt="omniget-cli demo" width="720" />
</p>

---

## Overview

`omniget-cli` is a terminal-first download manager built on the same engine as the [OmniGet desktop application](https://github.com/tonhowtf/omniget). It supports video, audio, course content, torrents, and direct file downloads across 1000+ websites powered by [yt-dlp](https://github.com/yt-dlp/yt-dlp).

This repository contains the CLI binary and a shared core library (`omniget-core`) that provides the download queue, dependency management, and platform detection logic.

## Features

| Feature | Description |
|---------|-------------|
| **Multi-platform downloads** | YouTube, Instagram, TikTok, Twitter/X, Reddit, Twitch, Pinterest, Vimeo, Bluesky, Bilibili, and 1000+ more via yt-dlp |
| **Batch processing** | Download multiple URLs from a text file in one command |
| **Auto dependencies** | yt-dlp, FFmpeg, aria2c, and Deno are downloaded and configured automatically |
| **Progress tracking** | Real-time progress bars with speed, percentage, and download phase |
| **Queue management** | Concurrent downloads with configurable limits and session persistence |
| **Settings system** | Read, write, and list configuration via JSON path notation |
| **Activity logs** | Tail application logs directly from the terminal |
| **Audio extraction** | Download audio-only streams with the `--audio-only` flag |

## Installation

### 🚀 Quick Start (Precompiled Binaries)
1. Go to the [Releases page](https://github.com/julesklord/omniget-cli/releases/latest).
2. Download the binary matching your operating system (`.exe` for Windows, binary for Linux/macOS).
3. (Optional) Rename it to `omniget-cli` and move it to a directory in your system's PATH to run it from anywhere.

### 📦 Using with Winget (Windows)
*Coming soon.* We are preparing the manifest for Microsoft Winget. Once submitted, you will be able to install OmniGet CLI simply by running:
```powershell
winget install omniget
```

### 🔨 From source
```bash
git clone https://github.com/julesklord/omniget-cli.git
cd omniget-cli
cargo build --release
```
The binary is output to `target/release/omniget-cli` (or `omniget-cli.exe` on Windows).

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+ (stable toolchain)
- Git

All runtime dependencies (yt-dlp, FFmpeg) are managed automatically on first use.

## Usage

```
omniget-cli [OPTIONS] <COMMAND>
```

### Global options

| Option | Description |
|--------|-------------|
| `-v`, `--verbose` | Enable verbose logging output |
| `-h`, `--help` | Print help information |

---

### `download` — Download media from a URL

```bash
omniget-cli download <URL> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-o`, `--output <DIR>` | Output directory (default: system Downloads folder) |
| `-q`, `--quality <QUALITY>` | Target quality, e.g. `1080p`, `720p`, `480p` |
| `-a`, `--audio-only` | Extract audio only |

**Examples:**

```bash
# Download a YouTube video at 1080p
omniget-cli download "https://www.youtube.com/watch?v=dQw4w9WgXcQ" -q 1080p

# Download audio only to a specific directory
omniget-cli download "https://www.youtube.com/watch?v=dQw4w9WgXcQ" --audio-only -o ~/Music

# Download from Instagram
omniget-cli download "https://www.instagram.com/reel/ABC123/"

# Download from TikTok
omniget-cli download "https://www.tiktok.com/@user/video/1234567890"
```

---

### `download-multiple` — Batch download from a file

```bash
omniget-cli download-multiple <FILE> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-o`, `--output <DIR>` | Output directory for all downloads |

The file should contain one URL per line. Empty lines are ignored.

**Example:**

```bash
# urls.txt contains one URL per line
omniget-cli download-multiple urls.txt -o ~/Videos
```

---

### `info` — Inspect media metadata

```bash
omniget-cli info <URL>
```

Fetches and displays metadata without downloading. Returns title, author, platform, duration, and media type.

**Example:**

```bash
omniget-cli info "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

**Output:**

```
Fetching media info for: https://www.youtube.com/watch?v=dQw4w9WgXcQ
--- Media Info ---
Title:    Rick Astley - Never Gonna Give You Up
Author:   Rick Astley
Platform: YouTube
Duration: 212.0 seconds
Type:     Video
------------------
```

---

### `list` — List downloads in the queue

```bash
omniget-cli list [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--active` | Show only active downloads |
| `--queued` | Show only queued downloads |
| `--completed` | Show only completed downloads |
| `--failed` | Show only failed downloads |

---

### `clean` — Clear download history

```bash
omniget-cli clean [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--finished` | Remove only finished downloads |
| `--failed` | Remove only failed downloads |

Without flags, clears the entire queue history.

---

### `config` — Manage settings

#### `config list`

Display all current settings as JSON.

```bash
omniget-cli config list
```

#### `config get <KEY>`

Read a specific setting using dot-notation path.

```bash
omniget-cli config get download.output_dir
omniget-cli config get download.max_concurrent
```

#### `config set <KEY> <VALUE>`

Write a setting value. Accepts strings, numbers, and booleans.

```bash
omniget-cli config set download.output_dir "/home/user/Videos"
omniget-cli config set download.max_concurrent 5
```

---

### `check` — Verify system dependencies

```bash
omniget-cli check
```

Scans for required binaries (yt-dlp, FFmpeg) and reports their paths. Downloads missing dependencies automatically.

**Output:**

```
Checking system dependencies...
✅ yt-dlp: Found at "/home/user/.local/share/omniget/yt-dlp"
✅ FFmpeg: Found at "/usr/bin/ffmpeg"
```

---

### `update` — Update internal dependencies

```bash
omniget-cli update
```

Forces a re-download of yt-dlp and FFmpeg to their latest versions.

---

### `logs` — View activity logs

```bash
omniget-cli logs [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--tail <N>` | Number of lines to show (default: 20) |

**Example:**

```bash
omniget-cli logs --tail 50
```

---

### `about` — Application information

```bash
omniget-cli about [TOPIC]
```

| Topic | Description |
|-------|-------------|
| `version` | Current version and build edition (default) |
| `roadmap` | Planned features and milestones |
| `changelog` | Release history |
| `terms` | License and usage terms |

---

## Supported platforms

### Media platforms

| Platform | Content |
|----------|---------|
| YouTube | Videos, Shorts, Playlists |
| Instagram | Posts, Reels, Stories |
| TikTok | Videos, Photos |
| Twitter / X | Videos, GIFs |
| Reddit | Videos, Images |
| Twitch | Clips |
| Pinterest | Images, Videos |
| Vimeo | Videos |
| Bluesky | Images, Videos |
| Bilibili (哔哩哔哩) | Videos, Series |

### Additional support

- **Torrent / Magnet**: Any `.torrent` file or magnet link via built-in client
- **Generic yt-dlp**: Any of the [1000+ sites supported by yt-dlp](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md)
- **P2P file sharing**: Send files between devices (planned)

## Architecture

The OmniGet monorepo is structured around a shared core library, `omniget-core`, which provides a platform-agnostic download engine. This allows both the GUI and CLI to share the same underlying logic for downloads, dependency management, and platform integration.

```
omniget/
└── src-tauri/
    ├── omniget-cli/            # CLI binary (clap + indicatif)
    │   ├── src/
    │   │   ├── main.rs         # Command definitions and dispatch
    │   │   └── reporter.rs     # Terminal progress bar UI
    │
    ├── omniget-core/           # SHARED CORE LIBRARY
    │   └── src/
    │       ├── core/
    │       │   ├── manager/    # Download queue, recovery, logging
    │       │   ├── traits.rs   # DownloadReporter & PlatformDownloader traits
    │       │   └── ...         # (dependencies, http_client, ytdlp)
    │       ├── platforms/      # All platform implementations (YouTube, etc.)
    │       └── models/         # Data structs (QueueItem, AppSettings)
    │
    └── omniget/ (GUI)          # Main Tauri application crate
        └── src/
            ├── commands/       # IPC command handlers
            └── core/
                └── reporters.rs  # GUI event emitter
```

| Crate | Role |
|---|---|
| `omniget-cli` | Standalone CLI binary. Uses `omniget-core`. |
| `omniget` (GUI) | Tauri desktop application. Uses `omniget-core`. |
| `omniget-core` | **The Engine**. Contains all business logic: download queue, dependency management, and all platform-specific downloaders. It is UI-agnostic. |

This architecture ensures that new features or bug fixes applied to `omniget-core` are instantly available to both the CLI and the GUI, preventing logic drift and reducing maintenance overhead.

## Configuration

Settings are stored in the OS application data directory:

| OS | Path |
|----|------|
| Windows | `%APPDATA%\omniget\` |
| macOS | `~/Library/Application Support/omniget/` |
| Linux | `~/.local/share/omniget/` |

Queue state is persisted in `recovery.json` within the same directory.

## Dependencies

All runtime dependencies are managed automatically. On first run, `omniget-cli` will download:

| Dependency | Purpose |
|------------|---------|
| **yt-dlp** | Media extraction from 1000+ sites |
| **FFmpeg** | Audio/video merging and conversion |
| **aria2c** | Accelerated HTTP downloads (optional) |

No manual installation is required.

## Roadmap

| Version | Milestone |
|---------|-----------|
| **v0.1.0** | CLI release with full command set ✅ |
| **v0.2.0** | Interactive TUI mode (`ratatui`) |
| **v0.3.0** | Plugin management commands |
| **v0.4.0** | P2P file sharing integration |

## Related projects

- [OmniGet Desktop](https://github.com/tonhowtf/omniget) — The original GUI application (Tauri + SvelteKit)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) — The media extraction engine

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions, project structure, and guidelines.

## License

[GPL-3.0](LICENSE)
