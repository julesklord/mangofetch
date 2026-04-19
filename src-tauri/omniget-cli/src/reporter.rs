use async_trait::async_trait;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use omniget_core::core::events::QueueItemProgress;
use omniget_core::core::traits::DownloadReporter;
use omniget_core::models::queue::QueueItemInfo;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct CLIReporter {
    multi_progress: MultiProgress,
    bars: Arc<Mutex<HashMap<u64, ProgressBar>>>,
}

impl CLIReporter {
    pub fn new() -> Self {
        Self {
            multi_progress: MultiProgress::new(),
            bars: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn create_progress_bar(&self, title: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(100));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{prefix:.bold.dim} {spinner:.green} [{bar:40.cyan/blue}] {pos:>3}% {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_prefix(title.to_string());
        pb
    }
}

#[async_trait]
impl DownloadReporter for CLIReporter {
    fn on_progress(&self, download_id: u64, info: QueueItemProgress) {
        let mut bars = self.bars.lock().unwrap();
        let pb = bars
            .entry(download_id)
            .or_insert_with(|| self.create_progress_bar("Downloading..."));

        pb.set_position(info.percent as u64);
        pb.set_message(format!(
            "{} MB/s",
            (info.speed_bytes_per_sec / 1_048_576.0 * 10.0).round() / 10.0
        ));
    }

    fn on_complete(
        &self,
        download_id: u64,
        _file_path: Option<String>,
        _file_size_bytes: Option<u64>,
    ) {
        let mut bars = self.bars.lock().unwrap();
        if let Some(pb) = bars.remove(&download_id) {
            pb.finish_with_message("Done");
        }
    }

    fn on_error(&self, download_id: u64, error_message: String) {
        let mut bars = self.bars.lock().unwrap();
        if let Some(pb) = bars.remove(&download_id) {
            pb.abandon_with_message(format!("Error: {}", error_message));
        } else {
            eprintln!("Download {} error: {}", download_id, error_message);
        }
    }

    fn on_retry(&self, _download_id: u64, attempt: u32, delay_ms: u64) {
        // Log to multi_progress or just stderr for now
        self.multi_progress
            .println(format!(
                "Retrying (attempt {}), waiting {}ms...",
                attempt, delay_ms
            ))
            .unwrap_or_default();
    }

    fn on_phase_change(&self, download_id: u64, phase: String) {
        let bars = self.bars.lock().unwrap();
        if let Some(pb) = bars.get(&download_id) {
            pb.set_message(phase);
        }
    }

    fn on_media_preview(
        &self,
        _url: String,
        title: String,
        author: String,
        _thumbnail_url: Option<String>,
        _duration_seconds: Option<f64>,
    ) {
        self.multi_progress
            .println(format!("Found: {} by {}", title, author))
            .unwrap_or_default();
    }

    fn on_queue_update(&self, _state: Vec<QueueItemInfo>) {
        // Not used much in CLI yet
    }

    fn on_system_progress(&self, title: &str, percent: f32, message: &str) {
        let mut bars = self.bars.lock().unwrap();
        // Use a high ID (e.g., u64::MAX) for system tasks
        let pb = bars
            .entry(u64::MAX)
            .or_insert_with(|| self.create_progress_bar(title));

        pb.set_prefix(title.to_string());
        pb.set_position(percent as u64);
        pb.set_message(message.to_string());
        if percent >= 100.0 {
            pb.finish_with_message("Done");
            bars.remove(&u64::MAX);
        }
    }
}
