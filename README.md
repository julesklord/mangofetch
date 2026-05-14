<table border="0">
  <tr>
    <td width="200" align="center" valign="top">
      <img src="docs/assets/logo.svg" width="180" alt="MangoFetch logo">
    </td>
    <td valign="top">
      <h1>mangofetch</h1>
      <p><strong>Fast, Tropical, Pure Rust.</strong><br/>
      <em>A headless, UI-agnostic download engine SDK & TUI/CLI frontend</em></p>
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
  <img src="docs/assets/shot.png" alt="MangoFetch in action" width="1080" />
</p>

___

<!--toc:start-->
- [mangofetch](#mangofetch)
  - [Overview](#overview)
  - [Using as a Rust SDK (mangofetch-core)](#using-as-a-rust-sdk-mangofetch-core)
  - [CLI/TUI Installation](#clitui-installation)
  - [Key Features (v0.5.1)](#key-features-v051)
  - [Technical Architecture](#technical-architecture)
  - [The Core Engine Lifecycle](#the-core-engine-lifecycle)
  - [Command Reference](#command-reference)
  - [Roadmap & Milestones](#roadmap--milestones)
  - [Acknowledgments](#acknowledgments)
  - [Contributing](#contributing)
  - [License](#license)
<!--toc:end-->

## Overview

**MangoFetch** is a fast and simple media downloader built in Rust. It's designed to be powerful but easy to use, keeping the heavy lifting of downloading away from the interface you're using.

At its heart is **`mangofetch-core`**, a lightweight and **headless engine**. Built on **Tokio** and **Reqwest**, it uses a simple API with Rust Traits to handle YouTube, Torrents (Magnet), SoundCloud, Instagram, and over 1000+ other platforms thanks to `yt-dlp` and `ffmpeg`.

For everyone else, MangoFetch comes with **`mangofetch-cli`**. It's a handy **TUI (Terminal User Interface)** that looks great and works even better, featuring easy settings, mouse support, and 11 fun Tropical Fruit color palettes to choose from.

___

<p align="center">
  <img src="docs/assets/mangofetch_demo.webp" alt="MangoFetch demo" width="680" />
</p>

---

## Using as a Rust Library (mangofetch-core)

Unlike big, clunky GUI downloaders, **MangoFetch is built to be part of your project**. If you're making a Discord bot, a web server, or your own custom app, you can just plug `mangofetch-core` right into your Rust code.

Add it to your `Cargo.toml`:
```toml
[dependencies]
mangofetch-core = { git = "https://github.com/julesklord/mangofetch-cli" }
```

**Why use `mangofetch-core`?**
* **Simple Telemetry:** Check progress easily using standard `tokio::sync::mpsc` channels. No UI blocking or complicated setup needed.
* **Unified Traits:** Whether it's a direct link, a torrent, or a TikTok video, you can talk to them all through the same `PlatformDownloader` trait.
* **Easy Dependencies:** The engine takes care of managing and checking external tools like `yt-dlp` and `ffmpeg` in its own space.
* **Reliable Queue:** A smart download manager that handles retries and network hiccups automatically.

---

## CLI/TUI Installation

### Via Cargo (Recommended)

The fastest way to install the CLI directly to your system path:

```zsh
cargo install mangofetch-cli
```

### From Source

For developers who want the absolute bleeding edge:

```zsh
git clone https://github.com/julesklord/mangofetch-cli.git
cd mangofetch-cli
cargo build --release
# The compiled binary will be available at: target/release/mangofetch
```

### Run
```zsh
mangofetch <command> <link>
```

```zsh
mangofetch tui (run the tui interactive dashboard)
```

---


## Key Features (v0.5.2)

*   **Supports 1000+ Sites**: Smooth integration with `yt-dlp` to work with almost any video site out there.
*   **Headless Core**: A clean design that lets you use the download engine as a standalone Rust library.
*   **Interactive TUI**: A responsive dashboard with **11 Tropical Fruit Themes** (Mango, Pitaya, Guayaba, Passionfruit, and more).
*   **Easy Mouse Support**: Use your mouse to scroll through your queue and click on tabs.
*   **Vim-Style Commands**: For the power users, you can use quick `:` commands to get things done fast.
*   **P2P & Torrents**: Works with magnet links and decentralized file sharing.
*   **Smart Downloading**: Uses multi-segment downloads and clever connection handling to keep things fast and avoid rate limits.

---

## Technical Architecture

MangoFetch is well-organized, keeping things clean and modular. This design makes the core engine portable, easy to test, and separate from how it's shown on screen.

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

- **`mangofetch-core`**: The heart of the system. It handles the download queue and works with sites like YouTube, Instagram, and TikTok. It also manages `yt-dlp` and `ffmpeg` for you, even downloading them if they aren't on your system.
- **`mangofetch-cli`**: A simple frontend built with `clap` and `ratatui`. It's fast, looks great, and shows you everything that's happening in real-time.
- **`mangofetch-plugin-sdk`**: A toolkit for adding new features to MangoFetch while it's running.

---

## How the Engine Works

The `mangofetch-core` queue is smart and reliable. It handles many downloads at once, and if something goes wrong with one, it just retries later without stopping everything else.

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

### Key Features under the hood

- **Fast Progress Reporting:** Uses background channels to show progress without slowing down the app. This keeps the interface smooth and responsive.
- **Self-Healing Tools:** Automatically finds and sets up the external tools it needs (`ffmpeg`, `yt-dlp`).
- **Smart Parsing:** Tries to handle links directly first, only using extra tools when it really needs to.

---

## Command Reference

For a full list of commands and how to use the TUI, check out our **[Official Wiki](docs/wiki/Home.md)**.

*   **[Installation Guide](docs/wiki/Installation.md)**
*   **[CLI Command Reference](docs/wiki/CLI-Guide.md)**
*   **[TUI Interactive Guide](docs/wiki/TUI-Experience.md)**
*   **[Technical Architecture](docs/wiki/Architecture.md)**

| Full Command                          | Short Alias _(Upcoming)_ | Description                                             |
| :------------------------------------ | :----------------------- | :------------------------------------------------------ |
| `mangofetch download <url>`           | `mango d <url>`          | Just download a single link.                            |
| `mangofetch download-multiple <file>` | `mango dm <file>`        | Download a whole bunch of links from a file.            |
| `mangofetch info <url>`               | `mango i <url>`          | See info about a link without downloading it.           |
| `mangofetch list`                     | `mango ls`               | See what's currently in your queue.                      |
| `mangofetch clean`                    | `mango c`                | Clear your history and cache.                           |
| `mangofetch config`                   | `mango cfg`              | Change settings like limits and paths.                  |
| `mangofetch check`                    | `mango ch`               | Check if your tools are working correctly.              |
| `mangofetch update`                   | `mango up`               | Update the external tools to their latest versions.     |
| `mangofetch logs`                     | `mango log`              | View app logs if you're curious or debugging.           |
| `mangofetch about`                    | `mango a`                | Show version and license info.                          |

---


## Roadmap & Milestones

| Version    | Status | Milestone                                                  |
| ---------- | ------ | ---------------------------------------------------------- |
| **v0.1.0** | ✅     | First version with basic downloading                       |
| **v0.2.0** | ✅     | Cleaned up the code and made the engine faster             |
| **v0.3.1** | ✅     | New name, better tests, and more docs                      |
| **v0.4.0** | ✅     | **The TUI Release:** A cool new terminal interface         |
| **v0.5.1** | ✅     | **UX & Polish:** Tropical themes and mouse support         |
| **v0.5.2** | ✅     | **Maintenance:** Relaxed tone, security fixes, and cleanup |
| **v0.6.0** | ⏳     | Plugins and extra community sites                          |
| **v0.7.0** | ⏳     | Better P2P and torrent support                             |

---

## Acknowledgments

- **[OmniGet](https://github.com/tonhowtf/omniget)** — A big inspiration for this project. Huge thanks to _tonhowft_ for the original ideas and engine logic.
- **[yt-dlp](https://github.com/yt-dlp/yt-dlp)** — The amazing tool that does the heavy lifting for so many sites.

## Contributing

Pull requests are always welcome. We have a few rules to keep the code clean, so if you're planning a big change, just open an issue first so we can chat about it. Check out `CONTRIBUTING.md` for more info.

## License

<p align="center">
  Built by <a href="https://github.com/julesklord">Jules</a>, gemini-cli assistant and claude.<br>
  Released under the GPL-3.0 License.
</p>
