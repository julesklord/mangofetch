
<table border="0">
  <tr>
    <td width="200" align="center" valign="top">
      <img src="docs/assets/logo.svg" width="180" alt="MangoFetch logo">
    </td>
    <td valign="top">
      <h1>mangofetch</h1>
      <p><strong>Fast, Tropical, Pure Rust.</strong><br/>
      <em>Headless download engine SDK with GUI, TUI, and CLI frontends.</em></p>
      <p>
        <a href="https://crates.io/crates/mangofetch-cli"><img src="https://img.shields.io/crates/v/mangofetch-cli?style=plastic&color=orange" alt="Crates.io"></a>
        <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPL--3.0-blue?style=plastic" alt="License GPL-3.0"></a>
        <img src="https://img.shields.io/badge/Built%20With-Rust-red?style=plastic&logo=rust" alt="Built with Rust">
        <img src="https://img.shields.io/badge/Architecture-Asynchronous-brightgreen?style=plastic" alt="Async">
        <img src="https://img.shields.io/badge/Concurrency-Tokio-purple?style=plastic" alt="Tokio">
      </p>
    </td>
  </tr>
</table>

---

<p align="center">
  <img src="docs/assets/shot.png" alt="MangoFetch in action"  />
</p>

___

<!--toc:start-->
- [Overview](#overview)
- [Cross-Platform Compatibility](#cross-platform-compatibility)
- [Using as a Rust Library (mangofetch-core)](#using-as-a-rust-library-mangofetch-core)
- [Installation](#installation)
  - [Via Cargo (Recommended)](#via-cargo-recommended)
  - [From Source](#from-source)
  - [Run](#run)
- [Technical Architecture](#technical-architecture)
  - [Core Components](#core-components)
- [How the Engine Works](#how-the-engine-works)
  - [Key Features under the hood](#key-features-under-the-hood)
- [Command Reference](#command-reference)
- [Acknowledgments](#acknowledgments)
- [Contributing](#contributing)
- [License](#license)
<!--toc:end-->

## Overview

MangoFetch downloads media quickly across multiple platforms. It provides granular control over video and audio formats without complex configuration.

The **`mangofetch-core`** engine uses Tokio and Reqwest to handle YouTube, Torrents, SoundCloud, and Instagram. It wraps `yt-dlp` and `ffmpeg` to support over 1000 platforms.

The suite includes three frontends:
1. **`mangofetch-gui`**: A hardware-accelerated desktop application powered by `egui`. It uses the MonolithUI design system and provides persistent logs and telemetry.
2. **`mangofetch tui`**: A terminal dashboard built with `ratatui`. It supports mouse interaction, modal dialogs, and multiple color palettes.
3. **`mangofetch cli`**: A scriptable command-line interface for batch processing.

## Cross-Platform Compatibility

MangoFetch runs natively on multiple architectures and operating systems. 

- **Operating Systems:** Windows (10/11), macOS (Intel and Apple Silicon), GNU/Linux, and BSD.
- **Architectures:** AMD64, ARM64, and ARMv7. It operates on desktops, MacBooks, and Raspberry Pi.
- **Dependency Management:** The engine detects missing binaries like `yt-dlp`, `ffmpeg`, or `aria2c`. It automatically downloads the correct standalone version for your OS.

___

<p align="center">
  <img src="docs/assets/mangofetch_demo.webp" alt="MangoFetch demo" />
</p>

---

## Using as a Rust Library (mangofetch-core)

Integrate `mangofetch-core` directly into Discord bots, web servers, or custom applications.

Add it to `Cargo.toml`:
```toml
[dependencies]
mangofetch-core = { git = "https://github.com/julesklord/mangofetch" }
```

**Engine Capabilities:**
* **Simple Telemetry:** Track progress via `tokio::sync::mpsc` channels.
* **Unified Traits:** Use the `PlatformDownloader` trait for links, torrents, and videos.
* **Dependency Automation:** The engine manages `yt-dlp` and `ffmpeg` binaries.
* **Recovery:** The download manager handles retries and network interruptions.

---

## CLI/TUI Installation

### Via Cargo (Recommended)

Install the CLI to your system path:

```zsh
cargo install mangofetch
```

### From Source

```zsh
git clone https://github.com/julesklord/mangofetch.git
cd mangofetch
cargo build --release
```

### Run
```zsh
mangofetch <command> <link>
```

Set `MANGOFETCH_OFFLINE=1` to run tests without downloading external tools:

```bash
export MANGOFETCH_OFFLINE=1
cargo test -p mangofetch-core
```

```zsh
mangofetch tui
```

---

## Technical Architecture

The modular design separates the core engine from the user interfaces.

```mermaid
graph TD
    User([Terminal / App User]) -->|CLI / Events| Frontend(CLI / Bot / Custom GUI)

    subgraph MangoFetch Workspace
        Frontend -->|Dispatch & Read MPSC| Core(mangofetch-core)

        subgraph Core Engine
            Core --> Manager[Manager: Queue & Recovery]
            Core --> Registry[Platform Registry]
            Manager --> IO[Tokio Async I/O]
            Registry --> Ext_Native[Native Extractors]
            Registry --> Ext_Generic[Generic Extractor]
        end

        Frontend -.->|Dynamic Linking| SDK(mangofetch-plugin-sdk)
    end

    Ext_Generic -->|Wraps| YTDLP[yt-dlp Binary]
    Ext_Native -->|Muxes Audio/Video| FFmpeg[FFmpeg Binary]
    Ext_Native -->|BitTorrent / P2P| RQBit[librqbit]
    YTDLP -.-> Network((Internet))
    IO -.-> Network
    IO --> Disk[(Local Storage)]
```

### Core Components

- **`mangofetch-core`**: Manages the download queue and platform extractors. It handles binary dependencies automatically.
- **`mangofetch`**: A terminal frontend using `clap` and `ratatui`.
- **`mangofetch-plugin-sdk`**: A toolkit for extending features at runtime.

---

## How the Engine Works

The queue processes multiple items concurrently. It isolates failures and triggers automatic retries.

```mermaid
stateDiagram-v2
    [*] --> Queued : Submit URL
    Queued --> FetchingMetadata : Worker Thread Picks Item
    FetchingMetadata --> Active : Media Info Resolved
    FetchingMetadata --> Error : Network/Parse Failure

    state Active {
        [*] --> Allocating
        Allocating --> Downloading : Progress Stream via MPSC
        Downloading --> Muxing : Audio+Video Merge (FFmpeg)
        Muxing --> [*]
    }

    Active --> Paused : User Pauses
    Paused --> Active : User Resumes
    Active --> Seeding : BitTorrent Complete
    Seeding --> Complete : Seed Goal Reached
    Active --> Complete : Success
    Active --> Error : Interruption / Connection Drop
    Error --> Queued : Retry Logic Triggered (Auto-Recovery)
    Complete --> [*]
```

### Internal Features

- **Progress Reporting:** Background channels provide updates without blocking the UI.
- **Binary Provisioning:** Locates and configures `ffmpeg` and `yt-dlp`.
- **Smart Parsing:** Prioritizes native extraction over external wrappers.

---

## Command Reference

View the **[Official Wiki](docs/wiki/Home.md)** for details.

*   **[Installation Guide](docs/wiki/Installation.md)**
*   **[CLI Command Reference](docs/wiki/CLI-Guide.md)**
*   **[TUI Interactive Guide](docs/wiki/TUI-Experience.md)**
*   **[Technical Architecture](docs/wiki/Architecture.md)**

| Command                          | Alias | Description                                             |
| :------------------------------- | :---- | :------------------------------------------------------ |
| `mangofetch download <url>`      | `mango d` | Download a single link.                                 |
| `mangofetch download-multiple`   | `mango dm`| Process links from a file.                              |
| `mangofetch info <url>`          | `mango i` | View metadata without downloading.                      |
| `mangofetch list`                | `mango ls`| View current queue.                                     |
| `mangofetch clean`               | `mango c` | Clear history and cache.                                |
| `mangofetch config`              | `mango cfg`| Modify limits and paths.                                |
| `mangofetch check`               | `mango ch`| Verify external tool status.                            |
| `mangofetch update`              | `mango up`| Update external binaries.                               |
| `mangofetch logs`                | `mango log`| View debug logs.                                        |
| `mangofetch about`               | `mango a` | Show version and license.                               |

---

## Acknowledgments

- **[OmniGet](https://github.com/tonhowtf/omniget)** — Inspired the original engine logic.
- **[yt-dlp](https://github.com/yt-dlp/yt-dlp)** — Handles core extraction.

## Contributing

Submit pull requests for features or fixes. Open an issue first for major changes. See `CONTRIBUTING.md` for details.

## License

<p align="center">
  Built by <a href="https://github.com/julesklord">Jules</a> and Claude.<br>
  Released under the GPL-3.0 License.
</p>
