use super::themes::Theme;
use chrono::Local;
use mangofetch_core::core::manager::queue::DownloadQueue;
use mangofetch_core::core::registry::PlatformRegistry;
use mangofetch_core::models::queue::{QueueItemInfo, QueueStatus};
use mangofetch_core::models::settings::AppSettings;
use std::sync::Arc;
use tokio::sync::Mutex;

// ── Enumerations ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Queue,
    History,
    Logs,
    Settings,
}

impl Tab {
    pub const ALL: &'static [Tab] = &[Tab::Queue, Tab::History, Tab::Logs, Tab::Settings];

    pub fn label(self, nf: bool) -> &'static str {
        match self {
            Tab::Queue => {
                if nf {
                    " 󰄖 Queue "
                } else {
                    " ⬇ Queue "
                }
            }
            Tab::History => {
                if nf {
                    " 󰄗 History "
                } else {
                    " 📜 History "
                }
            }
            Tab::Logs => {
                if nf {
                    "  Logs "
                } else {
                    " 📋 Logs "
                }
            }
            Tab::Settings => {
                if nf {
                    " 󰒓 Settings "
                } else {
                    " ⚙ Settings "
                }
            }
        }
    }

    pub fn index(self) -> usize {
        Tab::ALL.iter().position(|&t| t == self).unwrap_or(0)
    }
}

pub enum AppState {
    Splash,
    Running,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    /// Vim-style `:command` mode
    Command,
    /// Modal to add a new download URL
    AddUrl,
    /// Confirmation to delete selected item
    ConfirmDelete,
}

// ── App struct ────────────────────────────────────────────────────────────────

pub struct App {
    pub state: AppState,
    pub mode: Mode,
    pub active_tab: Tab,
    pub running: bool,

    /// Items shown in the current view (filtered from full queue state)
    pub items: Vec<QueueItemInfo>,
    pub table_state: ratatui::widgets::TableState,

    pub queue: Arc<Mutex<DownloadQueue>>,
    pub registry: Arc<PlatformRegistry>,
    pub theme: Theme,

    pub version: String,
    pub current_time: String,
    pub use_nerd_fonts: bool,

    /// Vim-style `:cmd` buffer
    pub command_buffer: String,
    /// Buffer for the Add URL modal
    pub url_input: String,
    /// Optional quality override when adding URL (e.g. "720p")
    pub quality_input: String,
    /// Which input field is focused in AddUrl modal (0 = url, 1 = quality)
    pub add_modal_field: usize,

    /// Settings tab cursor
    pub settings_index: usize,
    pub settings_count: usize,

    pub show_help: bool,

    pub status_message: Option<String>,
    pub status_is_error: bool,
    pub message_time: Option<std::time::Instant>,

    /// Ring-buffer for live log lines displayed on the Logs tab
    pub log_lines: Vec<String>,
    pub log_scroll: usize,

    /// Aggregate stats refreshed each tick
    pub total_speed: f64,
    pub active_count: usize,
    pub queued_count: usize,
}

// ── App impl ──────────────────────────────────────────────────────────────────

impl App {
    pub fn new(queue: Arc<Mutex<DownloadQueue>>, registry: Arc<PlatformRegistry>) -> Self {
        let settings = AppSettings::load_from_disk();
        let theme = Self::make_theme(&settings.appearance.tui_theme);

        Self {
            state: AppState::Splash,
            mode: Mode::Normal,
            active_tab: Tab::Queue,
            running: true,
            items: Vec::new(),
            table_state: ratatui::widgets::TableState::default(),
            queue,
            registry,
            theme,
            version: env!("CARGO_PKG_VERSION").to_string(),
            current_time: Local::now().format("%H:%M").to_string(),
            use_nerd_fonts: settings.appearance.use_nerd_fonts,
            command_buffer: String::new(),
            url_input: String::new(),
            quality_input: String::new(),
            add_modal_field: 0,
            settings_index: 0,
            settings_count: 4, // Theme, Nerd Fonts, Max Downloads, Quality
            show_help: false,
            status_message: None,
            status_is_error: false,
            message_time: None,
            log_lines: Vec::new(),
            log_scroll: 0,
            total_speed: 0.0,
            active_count: 0,
            queued_count: 0,
        }
    }

    fn make_theme(name: &str) -> Theme {
        match name {
            "pitaya" => Theme::pitaya(),
            "coconut" => Theme::coconut(),
            "dracula" => Theme::dracula(),
            _ => Theme::mango(),
        }
    }

    // ── Status helpers ────────────────────────────────────────────────────────

    pub fn set_status(&mut self, message: String) {
        self.status_is_error = false;
        self.status_message = Some(message);
        self.message_time = Some(std::time::Instant::now());
    }

    pub fn set_error(&mut self, message: String) {
        self.status_is_error = true;
        self.status_message = Some(message);
        self.message_time = Some(std::time::Instant::now());
    }

    pub fn clear_status_if_needed(&mut self) {
        if let Some(time) = self.message_time {
            if time.elapsed().as_secs() > 4 {
                self.status_message = None;
                self.message_time = None;
            }
        }
    }

    pub fn push_log(&mut self, line: String) {
        self.log_lines.push(line);
        // Keep only the last 500 lines to avoid unbounded memory
        if self.log_lines.len() > 500 {
            self.log_lines.drain(0..100);
        }
        // Auto-scroll to bottom
        self.log_scroll = self.log_lines.len().saturating_sub(1);
    }

    // ── Help toggle ───────────────────────────────────────────────────────────

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    // ── Queue actions ─────────────────────────────────────────────────────────

    pub async fn pause_selected(&mut self) {
        if let Some(item) = self.selected_item() {
            let id = item.id;
            let title = truncate(&item.title, 40);
            let mut q = self.queue.lock().await;
            q.pause(id);
            drop(q);
            self.set_status(format!("Paused: {}", title));
        }
    }

    pub async fn resume_selected(&mut self) {
        if let Some(item) = self.selected_item() {
            let id = item.id;
            {
                let mut q = self.queue.lock().await;
                q.resume(id);
            }
            mangofetch_core::core::manager::queue::try_start_next(self.queue.clone()).await;
            self.set_status(format!("Resumed: {}", truncate(&item.title, 40)));
        }
    }

    pub async fn remove_selected(&mut self) {
        if let Some(item) = self.selected_item() {
            let id = item.id;
            let mut q = self.queue.lock().await;
            q.remove(id);
            drop(q);
        }
        self.set_status("Removed from queue".to_string());
        self.mode = Mode::Normal;
    }

    fn selected_item(&self) -> Option<QueueItemInfo> {
        self.table_state
            .selected()
            .and_then(|idx| self.items.get(idx).cloned())
    }

    // ── Splash ────────────────────────────────────────────────────────────────

    pub fn start(&mut self) {
        self.state = AppState::Running;
    }

    // ── Settings actions ──────────────────────────────────────────────────────

    pub fn next_setting(&mut self) {
        self.settings_index = (self.settings_index + 1) % self.settings_count;
    }

    pub fn prev_setting(&mut self) {
        if self.settings_index == 0 {
            self.settings_index = self.settings_count - 1;
        } else {
            self.settings_index -= 1;
        }
    }

    pub fn toggle_setting(&mut self) {
        let mut settings = AppSettings::load_from_disk();
        match self.settings_index {
            0 => {
                // Cycle theme
                let next = match settings.appearance.tui_theme.as_str() {
                    "mango" => "pitaya",
                    "pitaya" => "coconut",
                    "coconut" => "dracula",
                    _ => "mango",
                };
                settings.appearance.tui_theme = next.to_string();
                self.theme = Self::make_theme(next);
                self.set_status(format!("Theme: {}", next));
            }
            1 => {
                settings.appearance.use_nerd_fonts = !settings.appearance.use_nerd_fonts;
                self.use_nerd_fonts = settings.appearance.use_nerd_fonts;
                self.set_status(format!(
                    "Nerd Fonts: {}",
                    if self.use_nerd_fonts { "ON" } else { "OFF" }
                ));
            }
            2 => {
                // Cycle max concurrent downloads: 1 → 2 → 3 → 5
                settings.advanced.max_concurrent_downloads =
                    match settings.advanced.max_concurrent_downloads {
                        1 => 2,
                        2 => 3,
                        3 => 5,
                        _ => 1,
                    };
                self.set_status(format!(
                    "Max downloads: {}",
                    settings.advanced.max_concurrent_downloads
                ));
            }
            3 => {
                // Cycle video quality
                settings.download.video_quality = match settings.download.video_quality.as_str() {
                    "best" => "1080p",
                    "1080p" => "720p",
                    "720p" => "480p",
                    "480p" => "360p",
                    _ => "best",
                }
                .to_string();
                self.set_status(format!("Quality: {}", settings.download.video_quality));
            }
            _ => {}
        }
        let _ = settings.save_to_disk();
    }

    // ── Data refresh ──────────────────────────────────────────────────────────

    pub fn refresh_data(&mut self) {
        self.current_time = Local::now().format("%H:%M:%S").to_string();

        if matches!(self.state, AppState::Splash) {
            return;
        }

        if let Ok(q) = self.queue.try_lock() {
            let all = q.get_state();

            // Compute aggregates from full list
            self.active_count = all
                .iter()
                .filter(|i| matches!(i.status, QueueStatus::Active))
                .count();
            self.queued_count = all
                .iter()
                .filter(|i| matches!(i.status, QueueStatus::Queued))
                .count();
            self.total_speed = all
                .iter()
                .filter(|i| matches!(i.status, QueueStatus::Active))
                .map(|i| i.speed_bytes_per_sec)
                .sum();

            // Filter per tab
            self.items = match self.active_tab {
                Tab::Queue => all
                    .into_iter()
                    .filter(|i| {
                        !matches!(
                            i.status,
                            QueueStatus::Complete { .. } | QueueStatus::Error { .. }
                        )
                    })
                    .collect(),
                Tab::History => all
                    .into_iter()
                    .filter(|i| {
                        matches!(
                            i.status,
                            QueueStatus::Complete { .. } | QueueStatus::Error { .. }
                        )
                    })
                    .collect(),
                Tab::Logs | Tab::Settings => Vec::new(),
            };
        }

        // Keep selection in-bounds
        if self.items.is_empty() {
            self.table_state.select(None);
        } else if self.table_state.selected().is_none() {
            self.table_state.select(Some(0));
        } else if let Some(sel) = self.table_state.selected() {
            if sel >= self.items.len() {
                self.table_state.select(Some(self.items.len() - 1));
            }
        }
    }

    // ── Navigation ────────────────────────────────────────────────────────────

    pub fn next_item(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let next = self
            .table_state
            .selected()
            .map(|i| if i >= self.items.len() - 1 { 0 } else { i + 1 })
            .unwrap_or(0);
        self.table_state.select(Some(next));
    }

    pub fn prev_item(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let prev = self
            .table_state
            .selected()
            .map(|i| if i == 0 { self.items.len() - 1 } else { i - 1 })
            .unwrap_or(0);
        self.table_state.select(Some(prev));
    }

    pub fn next_tab(&mut self) {
        let idx = (self.active_tab.index() + 1) % Tab::ALL.len();
        self.active_tab = Tab::ALL[idx];
        self.table_state.select(None);
        self.refresh_data();
    }

    pub fn prev_tab(&mut self) {
        let idx = (self.active_tab.index() + Tab::ALL.len() - 1) % Tab::ALL.len();
        self.active_tab = Tab::ALL[idx];
        self.table_state.select(None);
        self.refresh_data();
    }

    pub fn scroll_logs_down(&mut self) {
        let max = self.log_lines.len().saturating_sub(1);
        if self.log_scroll < max {
            self.log_scroll += 1;
        }
    }

    pub fn scroll_logs_up(&mut self) {
        if self.log_scroll > 0 {
            self.log_scroll -= 1;
        }
    }

    pub fn scroll_logs_bottom(&mut self) {
        self.log_scroll = self.log_lines.len().saturating_sub(1);
    }

    // ── Modal helpers ─────────────────────────────────────────────────────────

    pub fn open_add_modal(&mut self) {
        self.url_input.clear();
        self.quality_input.clear();
        self.add_modal_field = 0;
        self.mode = Mode::AddUrl;
    }

    pub fn close_add_modal(&mut self) {
        self.mode = Mode::Normal;
        self.url_input.clear();
        self.quality_input.clear();
    }

    pub fn add_modal_next_field(&mut self) {
        self.add_modal_field = (self.add_modal_field + 1) % 2;
    }

    // ── Quit ─────────────────────────────────────────────────────────────────

    pub fn quit(&mut self) {
        self.running = false;
    }
}

// ── Utility ───────────────────────────────────────────────────────────────────

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
