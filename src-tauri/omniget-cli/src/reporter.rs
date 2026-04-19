use async_trait::async_trait;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use omniget_core::core::events::QueueItemProgress;
use omniget_core::core::traits::DownloadReporter;
use omniget_core::models::queue::QueueItemInfo;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

// ============================================================================
// THEME SYSTEM — Make it easy to add more themes later
// ============================================================================

pub trait CliTheme: Send + Sync {
    /// Color codes — return ANSI escape codes or empty string for no color
    fn color_success(&self) -> String;
    fn color_error(&self) -> String;
    fn color_warning(&self) -> String;
    fn color_info(&self) -> String;
    fn color_platform(&self, platform: &str) -> String;
    fn color_accent(&self) -> String;
    fn color_reset(&self) -> String;

    /// Format a progress bar template
    fn progress_template(&self) -> String;

    /// Format phase change messages
    fn format_phase(&self, download_id: u64, phase: &str) -> String;

    /// Format completion message
    fn format_complete(&self, title: &str, file_path: Option<&str>, size_bytes: Option<u64>) -> String;

    /// Format error message
    fn format_error(&self, download_id: u64, error: &str) -> String;

    /// Format platform name with styling
    fn format_platform(&self, platform: &str) -> String;

    /// Check if theme supports Unicode (for graceful fallback)
    fn supports_unicode(&self) -> bool {
        true
    }
}

// ============================================================================
// BRUTALIST THEME (The Recommended Direction)
// ============================================================================

pub struct BrutalistTheme {
    supports_unicode: bool,
}

impl BrutalistTheme {
    pub fn new(supports_unicode: bool) -> Self {
        Self { supports_unicode }
    }

    fn _unicode_or_ascii<'a>(&self, unicode: &'a str, ascii: &'a str) -> &'a str {
        if self.supports_unicode {
            unicode
        } else {
            ascii
        }
    }
}

impl CliTheme for BrutalistTheme {
    fn color_success(&self) -> String {
        "\x1b[1;32m".to_string() // Bright Green
    }

    fn color_error(&self) -> String {
        "\x1b[1;31m".to_string() // Bright Red
    }

    fn color_warning(&self) -> String {
        "\x1b[1;33m".to_string() // Bright Yellow
    }

    fn color_info(&self) -> String {
        "\x1b[1;36m".to_string() // Bright Cyan
    }

    fn color_accent(&self) -> String {
        "\x1b[1;36m".to_string() // Bright Cyan
    }

    fn color_platform(&self, platform: &str) -> String {
        match platform.to_lowercase().as_str() {
            "youtube" => "\x1b[1;31m".to_string(), // Red
            "instagram" => "\x1b[1;35m".to_string(), // Magenta
            "tiktok" => "\x1b[1;36m".to_string(), // Cyan
            "twitter" | "x" => "\x1b[1;30m".to_string(), // Dark Gray
            "reddit" => "\x1b[1;33m".to_string(), // Orange/Yellow
            "twitch" => "\x1b[1;35m".to_string(), // Purple/Magenta
            "pinterest" => "\x1b[1;31m".to_string(), // Red
            "vimeo" => "\x1b[1;36m".to_string(), // Cyan
            "bluesky" => "\x1b[1;34m".to_string(), // Blue
            "bilibili" => "\x1b[1;36m".to_string(), // Cyan
            _ => "\x1b[1;37m".to_string(), // White
        }
    }

    fn color_reset(&self) -> String {
        "\x1b[0m".to_string()
    }

    fn progress_template(&self) -> String {
        "{prefix:.bold.cyan} {spinner:.cyan} [{bar:30.cyan/blue}] {pos:>3}% {msg:.dim}".to_string()
    }

    fn format_phase(&self, _download_id: u64, phase: &str) -> String {
        let icon = match phase.to_lowercase().as_str() {
            p if p.contains("fetch") || p.contains("info") => "🔍",
            p if p.contains("download") => "⬇️ ",
            p if p.contains("process") || p.contains("merge") => "🔧",
            p if p.contains("done") || p.contains("complete") => "✓ ",
            _ => "⏳",
        };

        format!(
            "{} {} Phase: {}",
            self.color_info(),
            icon,
            phase,
        )
    }

    fn format_complete(
        &self,
        title: &str,
        file_path: Option<&str>,
        size_bytes: Option<u64>,
    ) -> String {
        let size_str = size_bytes
            .map(|b| format_bytes(b))
            .unwrap_or_else(|| "unknown size".to_string());

        match file_path {
            Some(path) => format!(
                "{}✓ COMPLETE{} {} [{}] → {}",
                self.color_success(),
                self.color_reset(),
                title,
                size_str,
                path
            ),
            None => format!(
                "{}✓ COMPLETE{} {} [{}]",
                self.color_success(),
                self.color_reset(),
                title,
                size_str
            ),
        }
    }

    fn format_error(&self, download_id: u64, error: &str) -> String {
        format!(
            "{}✗ ERROR (ID:{}){} {}",
            self.color_error(),
            download_id,
            self.color_reset(),
            error
        )
    }

    fn format_platform(&self, platform: &str) -> String {
        format!(
            "{}{}{}",
            self.color_platform(platform),
            platform,
            self.color_reset()
        )
    }

    fn supports_unicode(&self) -> bool {
        self.supports_unicode
    }
}

// ============================================================================
// REFACTORED CLI REPORTER
// ============================================================================

pub struct CLIReporter {
    multi_progress: MultiProgress,
    bars: Arc<Mutex<HashMap<u64, ProgressBar>>>,
    theme: Arc<dyn CliTheme>,
}

impl CLIReporter {
    /// Create a new reporter with the default (Brutalist) theme
    pub fn new() -> Self {
        Self::with_theme(Arc::new(BrutalistTheme::new(true)))
    }

    /// Create a reporter with a custom theme
    pub fn with_theme(theme: Arc<dyn CliTheme>) -> Self {
        Self {
            multi_progress: MultiProgress::new(),
            bars: Arc::new(Mutex::new(HashMap::new())),
            theme,
        }
    }

    /// Create an ASCII-only reporter (for limited terminals)
    pub fn _ascii_only() -> Self {
        Self::with_theme(Arc::new(BrutalistTheme::new(false)))
    }

    fn create_progress_bar(&self, title: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(100));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&self.theme.progress_template())
                .unwrap()
                .progress_chars("█>░"),
        );
        pb.set_prefix(title.to_string());
        pb
    }
}

impl Default for CLIReporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DownloadReporter for CLIReporter {
    fn on_progress(&self, download_id: u64, info: QueueItemProgress) {
        let mut bars = self.bars.lock().unwrap();
        let pb = bars
            .entry(download_id)
            .or_insert_with(|| self.create_progress_bar(&format!("DL#{}", download_id)));

        pb.set_position(info.percent as u64);

        // Format speed and ETA inline
        let speed_mbs = (info.speed_bytes_per_sec / 1_048_576.0 * 10.0).round() / 10.0;
        
        let eta_str = if info.speed_bytes_per_sec > 0.0 && info.total_bytes.is_some() {
            let total = info.total_bytes.unwrap();
            let remaining = total.saturating_sub(info.downloaded_bytes);
            let eta_seconds = (remaining as f64 / info.speed_bytes_per_sec) as u64;
            format_duration(eta_seconds)
        } else {
            "calculating...".to_string()
        };

        pb.set_message(format!(
            "{}{} MB/s | ETA: {}{}",
            self.theme.color_accent(),
            speed_mbs,
            eta_str,
            self.theme.color_reset()
        ));
    }

    fn on_complete(
        &self,
        download_id: u64,
        file_path: Option<String>,
        file_size_bytes: Option<u64>,
    ) {
        let mut bars = self.bars.lock().unwrap();
        if let Some(pb) = bars.remove(&download_id) {
            let message = self.theme.format_complete(
                "Download",
                file_path.as_deref(),
                file_size_bytes,
            );
            pb.finish_with_message(message);
        }
    }

    fn on_error(&self, download_id: u64, error_message: String) {
        let mut bars = self.bars.lock().unwrap();
        if let Some(pb) = bars.remove(&download_id) {
            let formatted = self.theme.format_error(download_id, &error_message);
            pb.abandon_with_message(formatted);
        } else {
            let formatted = self.theme.format_error(download_id, &error_message);
            self.multi_progress.println(formatted).unwrap_or_default();
        }
    }

    fn on_retry(&self, download_id: u64, attempt: u32, delay_ms: u64) {
        let icon = "🔄";
        let message = format!(
            "{}{}(ID:{}) Retry attempt {} in {}ms{}",
            self.theme.color_warning(),
            icon,
            download_id,
            attempt,
            delay_ms,
            self.theme.color_reset()
        );
        self.multi_progress.println(message).unwrap_or_default();
    }

    fn on_phase_change(&self, download_id: u64, phase: String) {
        let bars = self.bars.lock().unwrap();
        if let Some(pb) = bars.get(&download_id) {
            let formatted = self.theme.format_phase(download_id, &phase);
            pb.set_message(formatted);
        }
    }

    fn on_media_preview(
        &self,
        url: String,
        title: String,
        author: String,
        _thumbnail_url: Option<String>,
        duration_seconds: Option<f64>,
    ) {
        // Extract platform from URL for color coding
        let platform = extract_platform(&url);

        let duration_str = duration_seconds
            .map(|d| format_duration(d as u64))
            .unwrap_or_else(|| "unknown duration".to_string());

        let message = format!(
            "{}📺 Found:{} {} by {} {} [{}]",
            self.theme.color_info(),
            self.theme.color_reset(),
            title,
            author,
            self.theme.format_platform(&platform),
            duration_str
        );

        self.multi_progress.println(message).unwrap_or_default();
    }

    fn on_queue_update(&self, _state: Vec<QueueItemInfo>) {}

    fn on_system_progress(&self, title: &str, percent: f32, message: &str) {
        let mut bars = self.bars.lock().unwrap();
        let pb = bars
            .entry(u64::MAX)
            .or_insert_with(|| self.create_progress_bar(title));

        pb.set_prefix(format!(
            "{}SYS: {}{}",
            self.theme.color_info(),
            title,
            self.theme.color_reset()
        ));
        pb.set_position(percent as u64);
        pb.set_message(message.to_string());

        if percent >= 100.0 {
            let complete_msg = format!(
                "{}✓ {}{}",
                self.theme.color_success(),
                title,
                self.theme.color_reset()
            );
            pb.finish_with_message(complete_msg);
            bars.remove(&u64::MAX);
        }
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Format bytes into human-readable size
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

/// Format seconds into human-readable duration
fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}

/// Extract platform name from URL
fn extract_platform(url: &str) -> String {
    if url.contains("youtube.com") || url.contains("youtu.be") {
        "YouTube".to_string()
    } else if url.contains("instagram.com") {
        "Instagram".to_string()
    } else if url.contains("tiktok.com") {
        "TikTok".to_string()
    } else if url.contains("twitter.com") || url.contains("x.com") {
        "Twitter/X".to_string()
    } else if url.contains("reddit.com") {
        "Reddit".to_string()
    } else if url.contains("twitch.tv") {
        "Twitch".to_string()
    } else if url.contains("pinterest.com") {
        "Pinterest".to_string()
    } else if url.contains("vimeo.com") {
        "Vimeo".to_string()
    } else if url.contains("bluesky.app") {
        "Bluesky".to_string()
    } else if url.contains("bilibili.com") {
        "Bilibili".to_string()
    } else {
        "Generic".to_string()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.0 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m");
    }

    #[test]
    fn test_extract_platform() {
        assert_eq!(extract_platform("https://youtube.com/watch?v=123"), "YouTube");
        assert_eq!(
            extract_platform("https://www.tiktok.com/@user/video/123"),
            "TikTok"
        );
        assert_eq!(
            extract_platform("https://www.instagram.com/reel/123/"),
            "Instagram"
        );
    }

    #[test]
    fn test_brutalist_theme() {
        let theme = BrutalistTheme::new(true);
        assert!(theme.supports_unicode());
        assert!(!theme.color_success().is_empty());
        assert!(!theme.progress_template().is_empty());
    }
}
