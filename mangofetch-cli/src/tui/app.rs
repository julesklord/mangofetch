use mangofetch_core::core::manager::queue::DownloadQueue;
use mangofetch_core::core::registry::PlatformRegistry;
use mangofetch_core::models::queue::QueueItemInfo;
use mangofetch_core::models::settings::AppSettings;
use std::sync::Arc;
use tokio::sync::Mutex;
use super::themes::Theme;
use chrono::Local;

#[derive(Debug, PartialEq, Eq)]
pub enum Tab {
    Queue,
    History,
    Settings,
}

pub enum AppState {
    Splash,
    Running,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Command,
}

pub struct App {
    pub state: AppState,
    pub mode: Mode,
    pub active_tab: Tab,
    pub running: bool,
    pub items: Vec<QueueItemInfo>,
    pub table_state: ratatui::widgets::TableState,
    pub queue: Arc<Mutex<DownloadQueue>>,
    pub registry: Arc<PlatformRegistry>,
    pub theme: Theme,
    pub version: String,
    pub current_time: String,
    pub use_nerd_fonts: bool,
    pub command_buffer: String,
    pub settings_index: usize,
    pub settings_count: usize,
    pub show_help: bool,
    pub status_message: Option<String>,
    pub message_time: Option<std::time::Instant>,
}

impl App {
    pub fn new(queue: Arc<Mutex<DownloadQueue>>, registry: Arc<PlatformRegistry>) -> Self {
        let settings = AppSettings::load_from_disk();
        let theme = match settings.appearance.tui_theme.as_str() {
            "pitaya" => Theme::pitaya(),
            "coconut" => Theme::coconut(),
            _ => Theme::mango(),
        };

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
            current_time: Local::now().format("%H:%M:%S").to_string(),
            use_nerd_fonts: settings.appearance.use_nerd_fonts,
            command_buffer: String::new(),
            settings_index: 0,
            settings_count: 2, // Theme and Nerd Fonts
            show_help: false,
            status_message: None,
            message_time: None,
        }
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
        self.message_time = Some(std::time::Instant::now());
    }

    pub fn clear_status_if_needed(&mut self) {
        if let Some(time) = self.message_time {
            if time.elapsed().as_secs() > 3 {
                self.status_message = None;
                self.message_time = None;
            }
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub async fn pause_selected(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            if let Some(item) = self.items.get(idx) {
                let id = item.id;
                let mut q = self.queue.lock().await;
                q.pause(id);
            }
        }
    }

    pub async fn resume_selected(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            if let Some(item) = self.items.get(idx) {
                let id = item.id;
                let mut q = self.queue.lock().await;
                q.resume(id);
                drop(q);
                mangofetch_core::core::manager::queue::try_start_next(self.queue.clone()).await;
            }
        }
    }

    pub async fn remove_selected(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            if let Some(item) = self.items.get(idx) {
                let id = item.id;
                let mut q = self.queue.lock().await;
                q.remove(id);
            }
        }
    }

    pub fn start(&mut self) {
        self.state = AppState::Running;
    }

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
            0 => { // Theme
                let next_theme = match settings.appearance.tui_theme.as_str() {
                    "mango" => "pitaya",
                    "pitaya" => "coconut",
                    _ => "mango",
                };
                settings.appearance.tui_theme = next_theme.to_string();
                self.theme = match next_theme {
                    "pitaya" => Theme::pitaya(),
                    "coconut" => Theme::coconut(),
                    _ => Theme::mango(),
                };
            }
            1 => { // Nerd Fonts
                settings.appearance.use_nerd_fonts = !settings.appearance.use_nerd_fonts;
                self.use_nerd_fonts = settings.appearance.use_nerd_fonts;
            }
            _ => {}
        }
        let _ = settings.save_to_disk();
    }

    pub fn refresh_data(&mut self) {
        self.current_time = Local::now().format("%H:%M:%S").to_string();

        if matches!(self.state, AppState::Splash) { return; }

        // Try to lock the queue without blocking the UI too much
        if let Ok(q) = self.queue.try_lock() {
            self.items = q.get_state();

            if self.active_tab == Tab::Queue {
                self.items.retain(|i| !matches!(i.status, mangofetch_core::models::queue::QueueStatus::Complete { .. }));
            } else if self.active_tab == Tab::History {
                self.items.retain(|i| matches!(i.status, mangofetch_core::models::queue::QueueStatus::Complete { .. } | mangofetch_core::models::queue::QueueStatus::Error { .. }));
            }
        }
        // Ensure selection is valid after refresh
        if self.items.is_empty() {
            self.table_state.select(None);
        } else if self.table_state.selected().is_none() {
            self.table_state.select(Some(0));
        } else if self.table_state.selected().unwrap() >= self.items.len() {
            self.table_state.select(Some(self.items.len() - 1));
        }
    }
    pub fn next_item(&mut self) {
        if self.items.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn prev_item(&mut self) {
        if self.items.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Queue => Tab::History,
            Tab::History => Tab::Settings,
            Tab::Settings => Tab::Queue,
        };
        self.table_state.select(None); // Reset selection when switching tabs
        self.refresh_data();
    }

    pub fn prev_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Queue => Tab::Settings,
            Tab::History => Tab::Queue,
            Tab::Settings => Tab::History,
        };
        self.table_state.select(None);
        self.refresh_data();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
