# 🏗️ Technical Architecture

MangoFetch is built with a modular, crate-based architecture designed for performance, memory safety, and minimal dependency overhead. It is written entirely in **Rust**.

## 📦 Project Structure

The repository is organized as a workspace with three primary crates:

1.  **`mangofetch-core`**: The engine. UI-agnostic library containing:
    *   **Download Manager**: Queue handling, task scheduling, and state persistence.
    *   **Platform Registry**: A modular system where different "Extractors" are registered.
    *   **Dependency Engine**: Logic to ensure FFmpeg and yt-dlp are available.
2.  **`mangofetch-cli`**: The binary. A thin wrapper over the core that provides:
    *   The `clap`-based command line interface.
    *   The `ratatui`-based Terminal User Interface.
3.  **`mangofetch-plugin-sdk`**: Tools and traits to build 3rd-party platform extractors.

---

## 🚦 The Download Lifecycle

Understanding how a URL becomes a file on your disk:

1.  **Extraction Phase**: The `PlatformRegistry` identifies the correct downloader (e.g., YouTubeDownloader). It fetches metadata (Title, Author, Qualities) without downloading the media.
2.  **Queueing Phase**: The item is added to the `DownloadQueue`. If the slot is available, it transitions to `Active`.
3.  **Execution Phase**:
    *   For standard videos, we invoke **yt-dlp** with specific performance flags.
    *   For direct links, we use a custom **Rust-native HTTP downloader** with multi-segment support.
    *   For P2P/Torrents, we utilize specialized crates for swarm connectivity.
4.  **Post-Processing Phase**: **FFmpeg** is invoked to merge streams (e.g., VP9 video + Opus audio) or to embed metadata and thumbnails.
5.  **Finalization**: The file is moved to the target directory, and the session is saved to `recovery.json`.

---

## 🔒 Security Architecture

MangoFetch takes security seriously:
*   **Sandboxed Subprocesses**: We sanitize all inputs before passing them to external tools like FFmpeg.
*   **Cookie Protection**: User cookies used for authentication (e.g., for age-restricted content) are handled securely and never logged.
*   **No Telemetry**: MangoFetch does not "phone home". All download data remains local to your machine.

---

## 🚀 Performance Optimizations

*   **Async/Await**: The entire engine is powered by `tokio`, allowing it to handle thousands of concurrent network events with negligible CPU usage.
*   **Zero-Copy Buffering**: When downloading direct files, we minimize memory copies to maximize disk I/O throughput.
*   **Staggered Starts**: We implement a staggering algorithm to avoid triggering rate-limiting when starting massive batch downloads.
