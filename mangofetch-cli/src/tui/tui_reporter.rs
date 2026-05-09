//! TUI-specific reporter — captures all download events into a shared log buffer
//! instead of writing to stdout (which would corrupt ratatui's alternate screen).

use async_trait::async_trait;
use mangofetch_core::core::events::QueueItemProgress;
use mangofetch_core::core::traits::DownloadReporter;
use mangofetch_core::models::queue::QueueItemInfo;
use std::sync::{Arc, Mutex};

/// Shared log sink for the TUI. The reporter pushes lines here,
/// and the `App` drains them on each tick.
pub type LogSink = Arc<Mutex<Vec<String>>>;

pub fn new_log_sink() -> LogSink {
    Arc::new(Mutex::new(Vec::new()))
}

/// A `DownloadReporter` that never touches stdout.
/// All events are serialized into human-readable log lines and pushed
/// into a `LogSink` that the TUI renders in the output panel.
pub struct TuiReporter {
    sink: LogSink,
}

impl TuiReporter {
    pub fn new(sink: LogSink) -> Self {
        Self { sink }
    }

    fn push(&self, line: String) {
        if let Ok(mut logs) = self.sink.lock() {
            logs.push(line);
            // Cap at 2000 lines to avoid unbounded growth
            if logs.len() > 2000 {
                logs.drain(0..500);
            }
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!(
            "{}…",
            s.chars().take(max.saturating_sub(1)).collect::<String>()
        )
    }
}

fn format_bytes_compact(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1_048_576 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1_073_741_824 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    }
}

#[async_trait]
impl DownloadReporter for TuiReporter {
    fn on_progress(&self, download_id: u64, info: QueueItemProgress) {
        // Only log significant progress (every 10%) to avoid flooding
        let pct = info.percent.round() as u64;
        if pct.is_multiple_of(10) || pct == 100 {
            let speed = if info.speed_bytes_per_sec > 0.0 {
                format!(
                    "{}/s",
                    format_bytes_compact(info.speed_bytes_per_sec as u64)
                )
            } else {
                "--".into()
            };
            self.push(format!(
                "[DL#{:02}] {:.0}%  {}  {}",
                download_id,
                info.percent,
                speed,
                truncate(&info.title, 40),
            ));
        }
    }

    fn on_complete(
        &self,
        download_id: u64,
        file_path: Option<String>,
        file_size_bytes: Option<u64>,
    ) {
        let size = file_size_bytes
            .map(format_bytes_compact)
            .unwrap_or_else(|| "--".into());
        let path = file_path.as_deref().unwrap_or("--");
        self.push(format!(
            "✓ DL#{:02} COMPLETE  [{}]  → {}",
            download_id, size, path
        ));
    }

    fn on_error(&self, download_id: u64, error_message: String) {
        self.push(format!(
            "✗ DL#{:02} ERROR  {}",
            download_id,
            truncate(&error_message, 80)
        ));
    }

    fn on_retry(&self, download_id: u64, attempt: u32, delay_ms: u64) {
        self.push(format!(
            "↻ DL#{:02} RETRY  attempt {} in {}ms",
            download_id, attempt, delay_ms
        ));
    }

    fn on_phase_change(&self, download_id: u64, phase: String) {
        self.push(format!("⟫ DL#{:02} → {}", download_id, phase));
    }

    fn on_media_preview(
        &self,
        _url: String,
        title: String,
        author: String,
        _thumbnail_url: Option<String>,
        duration_seconds: Option<f64>,
    ) {
        let dur = duration_seconds
            .map(|d| {
                let s = d as u64;
                if s >= 3600 {
                    format!("{}h{}m", s / 3600, (s % 3600) / 60)
                } else if s >= 60 {
                    format!("{}m{}s", s / 60, s % 60)
                } else {
                    format!("{}s", s)
                }
            })
            .unwrap_or_else(|| "--".into());
        self.push(format!(
            "◈ FOUND  {}  by {}  [{}]",
            truncate(&title, 40),
            truncate(&author, 20),
            dur,
        ));
    }

    fn on_queue_update(&self, _state: Vec<QueueItemInfo>) {
        // The TUI reads queue state directly; nothing to log here.
    }

    fn on_system_progress(&self, title: &str, percent: f32, message: &str) {
        let pct = percent.round() as u32;
        if pct.is_multiple_of(25) || pct >= 100 {
            let msg = if message.trim().is_empty() {
                "Preparing..."
            } else {
                message
            };
            self.push(format!("⚙ {}  {:.0}%  {}", title, percent, msg));
        }
    }
}
