# 🛠️ CLI Reference Guide

The MangoFetch CLI is designed to be the fastest way to download media. It follows a modular subcommand structure, making it intuitive for both simple and complex operations.

## 📥 Media Downloads

### `download` (alias: `d`)
Download a single media item from a URL.
*   **Usage**: `mangofetch download <URL> [OPTIONS]`
*   **Options**:
    *   `-o, --output <PATH>`: Specify a custom output directory.
    *   `-q, --quality <LABEL>`: Override default quality (e.g., `1080p`, `480p`, `best`).
    *   `-a, --audio-only`: Download only the audio track (extracts to `.mp3`/`.m4a`).

### `download-multiple`
Batch download a list of URLs from a text file.
*   **Usage**: `mangofetch download-multiple <FILE_PATH> [OPTIONS]`
*   **File Format**: One URL per line. Lines starting with `#` are ignored.

---

## 🔍 Discovery & Inspection

### `info`
Fetch metadata for a URL without starting the download.
*   **Usage**: `mangofetch info <URL>`
*   **Output**: Shows title, author, platform, duration, and available qualities.

> [!SCREENSHOT_PLACEHOLDER: CLI Info Card for a YouTube URL]
> *Captura de la salida del comando 'mangofetch info' mostrando una tarjeta de información elegante.*

---

## 🚦 Queue Management

### `list` (alias: `ls`)
View the current status of downloads in the queue.
*   **Usage**: `mangofetch list [FILTERS]`
*   **Filters**: `--active`, `--queued`, `--completed`, `--failed`.

### `clean`
Remove finished or failed items from the session history.
*   **Usage**: `mangofetch clean [OPTIONS]`
*   **Options**: `--finished`, `--failed`.

---

## ⚙️ Configuration

### `config` (alias: `cfg`)
Interact with the application settings directly from the terminal.
*   **Subcommands**:
    *   `list`: Show all current settings in JSON format.
    *   `get <KEY>`: Retrieve a specific setting value.
    *   `set <KEY> <VALUE>`: Update a setting (e.g., `mangofetch config set appearance.tui_theme pitaya`).

---

## 📋 System & Maintenance

### `check`
Verify that all system dependencies (FFmpeg, yt-dlp) are present and functional.

### `update`
Check for and download internal updates for dependencies like `yt-dlp`.

### `logs`
View the latest activity logs for debugging.
*   **Usage**: `mangofetch logs [--tail <LINES>]`

### `about`
Show version information, credits, and project links.

---

## 🖥️ Interactive Mode

### `tui`
Launch the full-screen Terminal User Interface.
*   **Usage**: `mangofetch tui`
*   **Learn more**: See the [TUI Experience](TUI-Experience.md) guide.
