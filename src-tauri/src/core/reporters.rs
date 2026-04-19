use omniget_core::core::events::QueueItemProgress;
use omniget_core::core::traits::DownloadReporter;
use omniget_core::models::queue::QueueItemInfo;
use tauri::{AppHandle, Emitter};

pub struct TauriReporter {
    app: AppHandle,
}

impl TauriReporter {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl DownloadReporter for TauriReporter {
    fn on_progress(&self, _id: u64, info: QueueItemProgress) {
        let _ = self.app.emit("queue-item-progress", info);
    }

    fn on_complete(&self, _id: u64, file_path: Option<String>, file_size_bytes: Option<u64>) {
        #[derive(serde::Serialize, Clone)]
        struct CompletePayload {
            file_path: Option<String>,
            file_size_bytes: Option<u64>,
        }
        let _ = self.app.emit(
            "download-complete",
            CompletePayload {
                file_path,
                file_size_bytes,
            },
        );
    }

    fn on_error(&self, _id: u64, error_message: String) {
        let _ = self.app.emit("download-error", error_message);
    }

    fn on_retry(&self, _id: u64, attempt: u32, delay_ms: u64) {
        #[derive(serde::Serialize, Clone)]
        struct RetryPayload {
            attempt: u32,
            delay_ms: u64,
        }
        let _ = self
            .app
            .emit("download-retry", RetryPayload { attempt, delay_ms });
    }

    fn on_phase_change(&self, _id: u64, phase: String) {
        let _ = self.app.emit("download-phase-change", phase);
    }

    fn on_media_preview(
        &self,
        url: String,
        title: String,
        author: String,
        thumbnail_url: Option<String>,
        duration_seconds: Option<f64>,
    ) {
        #[derive(serde::Serialize, Clone)]
        struct PreviewPayload {
            url: String,
            title: String,
            author: String,
            thumbnail_url: Option<String>,
            duration_seconds: Option<f64>,
        }
        let _ = self.app.emit(
            "media-info-preview",
            PreviewPayload {
                url,
                title,
                author,
                thumbnail_url,
                duration_seconds,
            },
        );
    }

    fn on_queue_update(&self, state: Vec<QueueItemInfo>) {
        use omniget_core::models::queue::QueueStatus;
        use tauri::Manager;

        let _ = self.app.emit("queue-state-update", &state);
        let total = crate::tray::compute_total_active(&self.app);
        crate::tray::update_active_count(&self.app, total);

        let active_items: Vec<_> = state
            .iter()
            .filter(|i| i.status == QueueStatus::Active)
            .collect();
        let avg_percent = if !active_items.is_empty() {
            let sum: f64 = active_items.iter().map(|i| i.percent).sum();
            sum / active_items.len() as f64 / 100.0
        } else {
            0.0
        };
        crate::tray::update_taskbar_badge(&self.app, total, avg_percent);

        if let Some(window) = self.app.get_webview_window("main") {
            let title = if total > 0 {
                format!("({}) omniget", total)
            } else {
                "omniget".into()
            };
            let _ = window.set_title(&title);
        }
    }

    fn on_system_progress(&self, title: &str, percent: f32, message: &str) {
        #[derive(serde::Serialize, Clone)]
        struct SystemProgressPayload {
            title: String,
            percent: f32,
            message: String,
        }
        let _ = self.app.emit(
            "system-progress",
            SystemProgressPayload {
                title: title.to_string(),
                percent,
                message: message.to_string(),
            },
        );
    }
}
