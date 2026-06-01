# Technical Architecture

MangoFetch uses a modular, crate-based architecture implemented in **Rust** to manage performance and memory safety.

## Project Structure

The repository uses a workspace with three primary crates:

1.  **`mangofetch-core`**: The engine. A UI-agnostic library containing:
    *   **Download Manager**: Handles queueing, task scheduling, and state persistence.
    *   **Platform Registry**: Manages registered "Extractors" for different platforms.
    *   **Dependency Engine**: Verifies availability of FFmpeg and yt-dlp.
2.  **`mangofetch-cli`**: The binary. Interfaces with the core to provide:
    *   The `clap`-based command line interface.
    *   The `ratatui`-based Terminal User Interface.
3.  **`mangofetch-plugin-sdk`**: Provides traits and tools for building 3rd-party platform extractors.

---

## Download Lifecycle

The process for converting a URL into a local file includes the following phases:

1.  **Extraction**: The `PlatformRegistry` identifies the required downloader (e.g., `YouTubeDownloader`) and fetches metadata without downloading media.
2.  **Queueing**: The item enters the `DownloadQueue` and transitions to an `Active` state when a slot is available.
3.  **Execution**:
    *   **Videos**: The system invokes **yt-dlp** with performance-optimized flags.
    *   **Direct Links**: The system uses a Rust-native HTTP downloader with multi-segment support.
    *   **P2P**: Specialized crates manage swarm connectivity for torrents.
4.  **Post-Processing**: **FFmpeg** merges streams (e.g., VP9 video and Opus audio) or embeds metadata and thumbnails.
5.  **Finalization**: The system moves the file to the target directory and saves the session to `recovery.json`.

---

## Security

*   **Subprocesses**: Input sanitization precedes data transfer to external tools like FFmpeg.
*   **Cookie Management**: The engine handles authentication cookies securely without logging.
*   **Local Processing**: The application does not include telemetry; all data remains local.

---

## Performance

*   **Concurrency**: `tokio` manages network events to maintain low CPU usage.
*   **I/O Efficiency**: Zero-copy buffering minimizes memory allocation during direct file downloads.
*   **Rate Limiting**: A staggering algorithm prevents rate-limit triggers during batch downloads.
