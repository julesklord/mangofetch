use super::themes::Theme;
use super::tui_reporter::LogSink;
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
    Home,
    Queue,
    History,
    Settings,
    About,
    Logs,
}

impl Tab {
    pub fn label(self, nf: bool) -> &'static str {
        match self {
            Tab::Home => {
                if nf {
                    " 󰋜 Home "
                } else {
                    " 🏠 Home "
                }
            }
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
            Tab::Settings => {
                if nf {
                    " 󰒓 Settings "
                } else {
                    " ⚙ Settings "
                }
            }
            Tab::About => {
                if nf {
                    " 󰋽 About "
                } else {
                    " ℹ About "
                }
            }
            Tab::Logs => {
                if nf {
                    " 󰋚 Logs "
                } else {
                    " 📋 Logs "
                }
            }
        }
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Home => 0,
            Tab::Queue => 1,
            Tab::History => 2,
            Tab::Settings => 3,
            Tab::About => 4,
            Tab::Logs => 5,
        }
    }

    pub const ALL: &'static [Tab] = &[
        Tab::Home,
        Tab::Queue,
        Tab::History,
        Tab::Settings,
        Tab::About,
        Tab::Logs,
    ];
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DownloadsCategory {
    All,
    Active,
    Queued,
    Completed,
    Failed,
}

impl DownloadsCategory {
    pub const ALL: &'static [DownloadsCategory] = &[
        DownloadsCategory::All,
        DownloadsCategory::Active,
        DownloadsCategory::Queued,
        DownloadsCategory::Completed,
        DownloadsCategory::Failed,
    ];

    pub fn label(self, nf: bool) -> String {
        match self {
            DownloadsCategory::All => if nf { "󰄗 All" } else { "All" }.into(),
            DownloadsCategory::Active => if nf { "󰄖 Active" } else { "Active" }.into(),
            DownloadsCategory::Queued => if nf { "󰄗 Queued" } else { "Queued" }.into(),
            DownloadsCategory::Completed => if nf { "󰄬 Completed" } else { "Completed" }.into(),
            DownloadsCategory::Failed => if nf { "󰅖 Failed" } else { "Failed" }.into(),
        }
    }
}

pub enum AppMsg {
    PreviewFetched(Result<mangofetch_core::models::queue::QueueItemInfo, String>),
}

pub enum AppState {
    Splash,
    Running,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Normal,
    /// Vim-style `:command` mode
    Command,
    /// Modal to add a new download URL
    AddUrl,
    /// Pre-download confirmation modal
    AddConfirm,
    /// Confirmation to delete selected item
    ConfirmDelete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingKind {
    TuiTheme,
    UseNerdFonts,
    MaxDownloads,
    VideoQuality,
    OrganizeByPlatform,
    SkipExisting,
    DownloadSubtitles,
    DownloadAttachments,
    DownloadDescriptions,
    SponsorBlock,
    SplitByChapters,
    EmbedMetadata,
    EmbedThumbnail,
    MaxConcurrentSegments,
    MaxRetries,
    ConcurrentFragments,
    StaggerDelay,
    ClipboardDetection,
    ProxyEnabled,
    PortableMode,
}

impl SettingKind {
    pub const ALL: &'static [SettingKind] = &[
        SettingKind::TuiTheme,
        SettingKind::UseNerdFonts,
        SettingKind::MaxDownloads,
        SettingKind::VideoQuality,
        SettingKind::OrganizeByPlatform,
        SettingKind::SkipExisting,
        SettingKind::DownloadSubtitles,
        SettingKind::DownloadAttachments,
        SettingKind::DownloadDescriptions,
        SettingKind::SponsorBlock,
        SettingKind::SplitByChapters,
        SettingKind::EmbedMetadata,
        SettingKind::EmbedThumbnail,
        SettingKind::MaxConcurrentSegments,
        SettingKind::MaxRetries,
        SettingKind::ConcurrentFragments,
        SettingKind::StaggerDelay,
        SettingKind::ClipboardDetection,
        SettingKind::ProxyEnabled,
        SettingKind::PortableMode,
    ];

    pub fn label(self) -> &'static str {
        match self {
            SettingKind::TuiTheme => "TUI Theme",
            SettingKind::UseNerdFonts => "Nerd Fonts",
            SettingKind::MaxDownloads => "Max Downloads",
            SettingKind::VideoQuality => "Default Quality",
            SettingKind::OrganizeByPlatform => "Organize Platforms",
            SettingKind::SkipExisting => "Skip Existing",
            SettingKind::DownloadSubtitles => "Download Subtitles",
            SettingKind::DownloadAttachments => "Download Attachments",
            SettingKind::DownloadDescriptions => "Download Descriptions",
            SettingKind::SponsorBlock => "SponsorBlock",
            SettingKind::SplitByChapters => "Split Chapters",
            SettingKind::EmbedMetadata => "Embed Metadata",
            SettingKind::EmbedThumbnail => "Embed Thumbnail",
            SettingKind::MaxConcurrentSegments => "Max Segments",
            SettingKind::MaxRetries => "Max Retries",
            SettingKind::ConcurrentFragments => "Concurrent Fragments",
            SettingKind::StaggerDelay => "Stagger Delay",
            SettingKind::ClipboardDetection => "Clipboard Detect",
            SettingKind::ProxyEnabled => "Use Proxy",
            SettingKind::PortableMode => "Portable Mode",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            SettingKind::TuiTheme => "mango │ pitaya │ coconut │ dracula",
            SettingKind::UseNerdFonts => "Enables icons (requires patched terminal)",
            SettingKind::MaxDownloads => "Max simultaneous downloads",
            SettingKind::VideoQuality => "best │ 1080p │ 720p │ 480p │ 360p",
            SettingKind::OrganizeByPlatform => "Organize files into platform folders",
            SettingKind::SkipExisting => "Do not re-download existing files",
            SettingKind::DownloadSubtitles => "Attempt to download subtitles",
            SettingKind::DownloadAttachments => "Download extra media attachments",
            SettingKind::DownloadDescriptions => "Save .description files",
            SettingKind::SponsorBlock => "Skip sponsors on YouTube",
            SettingKind::SplitByChapters => "Split video into chapter files",
            SettingKind::EmbedMetadata => "Write metadata to file",
            SettingKind::EmbedThumbnail => "Embed thumbnail into video",
            SettingKind::MaxConcurrentSegments => "Max segments for direct downloads",
            SettingKind::MaxRetries => "Number of retry attempts",
            SettingKind::ConcurrentFragments => "Fragments per download (yt-dlp)",
            SettingKind::StaggerDelay => "Delay between downloads (ms)",
            SettingKind::ClipboardDetection => "Scan clipboard for URLs",
            SettingKind::ProxyEnabled => "Use configured proxy",
            SettingKind::PortableMode => "Save data in local folder",
        }
    }
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

    /// Preview info for confirmation
    pub preview_info: Option<mangofetch_core::models::queue::QueueItemInfo>,
    /// Whether we are currently fetching info for preview
    pub is_fetching_preview: bool,
    /// Error message during preview fetch
    pub preview_error: Option<String>,

    pub msg_tx: tokio::sync::mpsc::UnboundedSender<AppMsg>,
    pub msg_rx: tokio::sync::mpsc::UnboundedReceiver<AppMsg>,

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

    /// Ring-buffer for the bottom output panel (reporter events)
    pub output_lines: Vec<String>,
    pub output_scroll: usize,

    /// Shared log sink from the TuiReporter
    log_sink: LogSink,

    /// Current category for the Queue/History tabs
    pub download_category: DownloadsCategory,

    /// Selected action in Home tab
    pub home_index: usize,

    /// Selected sub-section in About tab
    pub about_index: usize,

    /// Aggregate stats refreshed each tick
    pub total_speed: f64,
    pub active_count: usize,
    pub queued_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
}

// ── App impl ──────────────────────────────────────────────────────────────────

impl App {
    pub fn new(
        queue: Arc<Mutex<DownloadQueue>>,
        registry: Arc<PlatformRegistry>,
        log_sink: LogSink,
    ) -> Self {
        let settings = AppSettings::load_from_disk();
        let theme = Self::make_theme(&settings.appearance.tui_theme);
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

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
            settings_count: 20, // Expanded list
            show_help: false,
            preview_info: None,
            is_fetching_preview: false,
            preview_error: None,
            msg_tx: tx,
            msg_rx: rx,
            status_message: None,
            status_is_error: false,
            message_time: None,
            log_lines: Vec::new(),
            log_scroll: 0,
            output_lines: Vec::new(),
            output_scroll: 0,
            log_sink,
            download_category: DownloadsCategory::All,
            home_index: 0,
            about_index: 0,
            total_speed: 0.0,
            active_count: 0,
            queued_count: 0,
            completed_count: 0,
            failed_count: 0,
        }
    }

    fn make_theme(name: &str) -> Theme {
        match name {
            "pitaya" => Theme::pitaya(),
            "coconut" => Theme::coconut(),
            "guava" => Theme::guava(),
            "papaya" => Theme::papaya(),
            "passionfruit" => Theme::passionfruit(),
            "lychee" => Theme::lychee(),
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

    pub fn log_scroll_up(&mut self) {
        if self.log_scroll > 0 {
            self.log_scroll -= 1;
        }
    }

    pub fn log_scroll_down(&mut self) {
        if self.log_scroll < self.log_lines.len().saturating_sub(1) {
            self.log_scroll += 1;
        }
    }

    /// Drain any pending lines from the TuiReporter into output_lines.
    pub fn drain_reporter_logs(&mut self) {
        if let Ok(mut sink) = self.log_sink.lock() {
            for line in sink.drain(..) {
                let cleaned = strip_ansi_and_clean(&line);
                if cleaned.is_empty() {
                    continue;
                }
                // Mirror into log_lines for the Logs tab too
                self.log_lines.push(cleaned.clone());
                self.output_lines.push(cleaned);
            }
        }
        // Cap sizes
        if self.log_lines.len() > 1000 {
            self.log_lines.drain(0..200);
        }
        if self.output_lines.len() > 500 {
            self.output_lines.drain(0..100);
        }
        // Auto-scroll output to bottom
        self.output_scroll = self.output_lines.len().saturating_sub(1);
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
                    "coconut" => "guava",
                    "guava" => "papaya",
                    "papaya" => "passionfruit",
                    "passionfruit" => "lychee",
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
            4 => {
                settings.download.organize_by_platform = !settings.download.organize_by_platform;
                self.set_status(format!(
                    "Organize by Platform: {}",
                    if settings.download.organize_by_platform {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            5 => {
                settings.download.skip_existing = !settings.download.skip_existing;
                self.set_status(format!(
                    "Skip existing: {}",
                    if settings.download.skip_existing {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            6 => {
                settings.download.download_subtitles = !settings.download.download_subtitles;
                self.set_status(format!(
                    "Download Subtitles: {}",
                    if settings.download.download_subtitles {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            7 => {
                settings.download.download_attachments = !settings.download.download_attachments;
                self.set_status(format!(
                    "Download Attachments: {}",
                    if settings.download.download_attachments {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            8 => {
                settings.download.download_descriptions = !settings.download.download_descriptions;
                self.set_status(format!(
                    "Download Descriptions: {}",
                    if settings.download.download_descriptions {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            9 => {
                settings.download.youtube_sponsorblock = !settings.download.youtube_sponsorblock;
                self.set_status(format!(
                    "SponsorBlock: {}",
                    if settings.download.youtube_sponsorblock {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            10 => {
                settings.download.split_by_chapters = !settings.download.split_by_chapters;
                self.set_status(format!(
                    "Split by chapters: {}",
                    if settings.download.split_by_chapters {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            11 => {
                settings.download.embed_metadata = !settings.download.embed_metadata;
                self.set_status(format!(
                    "Embed Metadata: {}",
                    if settings.download.embed_metadata {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            12 => {
                settings.download.embed_thumbnail = !settings.download.embed_thumbnail;
                self.set_status(format!(
                    "Embed Thumbnail: {}",
                    if settings.download.embed_thumbnail {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            13 => {
                settings.advanced.max_concurrent_segments =
                    match settings.advanced.max_concurrent_segments {
                        8 => 16,
                        16 => 32,
                        32 => 64,
                        _ => 8,
                    };
                self.set_status(format!(
                    "Max Segments: {}",
                    settings.advanced.max_concurrent_segments
                ));
            }
            14 => {
                settings.advanced.concurrent_fragments =
                    match settings.advanced.concurrent_fragments {
                        4 => 8,
                        8 => 16,
                        _ => 4,
                    };
                self.set_status(format!(
                    "Max Fragments: {}",
                    settings.advanced.concurrent_fragments
                ));
            }
            15 => {
                settings.advanced.stagger_delay_ms = match settings.advanced.stagger_delay_ms {
                    0 => 100,
                    100 => 250,
                    250 => 500,
                    _ => 0,
                };
                self.set_status(format!(
                    "Stagger Delay: {}ms",
                    settings.advanced.stagger_delay_ms
                ));
            }
            16 => {
                settings.advanced.torrent_listen_port = match settings.advanced.torrent_listen_port
                {
                    6881 => 8881,
                    8881 => 9881,
                    _ => 6881,
                };
                self.set_status(format!(
                    "Torrent Port: {}",
                    settings.advanced.torrent_listen_port
                ));
            }
            17 => {
                settings.proxy.enabled = !settings.proxy.enabled;
                self.set_status(format!(
                    "Proxy: {}",
                    if settings.proxy.enabled { "ON" } else { "OFF" }
                ));
            }
            18 => {
                settings.telegram.fix_file_extensions = !settings.telegram.fix_file_extensions;
                self.set_status(format!(
                    "Fix TG extensions: {}",
                    if settings.telegram.fix_file_extensions {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            19 => {
                settings.start_with_windows = !settings.start_with_windows;
                self.set_status(format!(
                    "Start with Windows: {}",
                    if settings.start_with_windows {
                        "ON"
                    } else {
                        "OFF"
                    }
                ));
            }
            _ => {}
        }
        let _ = settings.save_to_disk();
    }

    // ── Message processing ───────────────────────────────────────────────────

    pub fn process_messages(&mut self) {
        while let Ok(msg) = self.msg_rx.try_recv() {
            match msg {
                AppMsg::PreviewFetched(result) => {
                    self.is_fetching_preview = false;
                    match result {
                        Ok(info) => {
                            self.preview_info = Some(info);
                            self.preview_error = None;
                        }
                        Err(e) => {
                            self.preview_error = Some(e);
                            self.preview_info = None;
                        }
                    }
                }
            }
        }
    }

    // ── Data refresh ──────────────────────────────────────────────────────────

    pub fn refresh_data(&mut self) {
        self.process_messages();

        // Update clock only once per second to avoid UI churn
        let now = Local::now();
        let time_str = now.format("%H:%M").to_string();
        if self.current_time != time_str {
            self.current_time = time_str;
        }

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
            self.completed_count = all
                .iter()
                .filter(|i| matches!(i.status, QueueStatus::Complete { .. }))
                .count();
            self.failed_count = all
                .iter()
                .filter(|i| matches!(i.status, QueueStatus::Error { .. }))
                .count();

            self.total_speed = all
                .iter()
                .filter(|i| matches!(i.status, QueueStatus::Active))
                .map(|i| i.speed_bytes_per_sec)
                .sum();

            // Filter per tab and category
            self.items = match self.active_tab {
                Tab::Queue | Tab::History => {
                    all.into_iter()
                        .filter(|i| match self.download_category {
                            DownloadsCategory::All => true,
                            DownloadsCategory::Active => matches!(i.status, QueueStatus::Active),
                            DownloadsCategory::Queued => matches!(i.status, QueueStatus::Queued),
                            DownloadsCategory::Completed => {
                                matches!(i.status, QueueStatus::Complete { .. })
                            }
                            DownloadsCategory::Failed => {
                                matches!(i.status, QueueStatus::Error { .. })
                            }
                        })
                        .filter(|i| {
                            // Secondary filter to keep Queue/History separation if desired,
                            // but usually categories are enough.
                            match self.active_tab {
                                Tab::Queue => !matches!(
                                    i.status,
                                    QueueStatus::Complete { .. } | QueueStatus::Error { .. }
                                ),
                                Tab::History => matches!(
                                    i.status,
                                    QueueStatus::Complete { .. } | QueueStatus::Error { .. }
                                ),
                                _ => true,
                            }
                        })
                        .collect()
                }
                _ => Vec::new(),
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
        let all = Tab::ALL;
        let current_idx = all.iter().position(|&t| t == self.active_tab).unwrap_or(0);
        self.active_tab = all[(current_idx + 1) % all.len()];
        self.table_state.select(None);
        self.refresh_data();
    }

    pub fn prev_tab(&mut self) {
        let all = Tab::ALL;
        let current_idx = all.iter().position(|&t| t == self.active_tab).unwrap_or(0);
        if current_idx == 0 {
            self.active_tab = all[all.len() - 1];
        } else {
            self.active_tab = all[current_idx - 1];
        }
        self.table_state.select(None);
        self.refresh_data();
    }

    pub fn next_category(&mut self) {
        let all = DownloadsCategory::ALL;
        let current_idx = all
            .iter()
            .position(|&c| c == self.download_category)
            .unwrap_or(0);
        self.download_category = all[(current_idx + 1) % all.len()];
        self.refresh_data();
    }

    pub async fn execute_home_action(&mut self) {
        match self.home_index {
            0 => self.open_add_modal(),
            1 => self.set_status("Batch Download: Use 'download-multiple' command".to_string()),
            2 => self.set_status("P2P Send: Use 'send' command".to_string()),
            3 => self.set_status("Torrent/Magnet: Paste URL in 'Add URL' modal".to_string()),
            _ => {}
        }
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

fn strip_ansi_and_clean(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == '['
                || c == '('
                || c == ')'
                || c == '#'
                || c == '?'
                || c.is_ascii_digit()
                || c == ';'
            {
                continue;
            }
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if c == '\r' || c == '\n' || c == '\0' {
            continue;
        } else {
            result.push(c);
        }
    }
    result.trim().to_string()
}
