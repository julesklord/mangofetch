use omniget_core::core::events::{EventEmitter, QueueItemProgress};
use omniget_core::models::queue::{QueueItemInfo, QueueStatus};
use tauri::Emitter;

#[derive(Clone)]
pub struct TauriEventEmitter {
    app: tauri::AppHandle,
}

impl TauriEventEmitter {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }
}

impl EventEmitter for TauriEventEmitter {
    fn emit_queue_state(&self, items: &[QueueItemInfo]) {
        let _ = self.app.emit("queue-state-update", items);
        let active_count = items
            .iter()
            .filter(|i| i.status == QueueStatus::Active)
            .count() as u32;
        crate::tray::update_active_count(&self.app, active_count);
    }

    fn emit_progress(&self, progress: &QueueItemProgress) {
        let _ = self.app.emit("queue-item-progress", progress);
    }
}
