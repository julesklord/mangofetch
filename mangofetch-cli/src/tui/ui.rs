use super::app::{App, AppState, DownloadsCategory, Mode, SettingKind, Tab};
use super::assets::{BLOCK_TITLE, MANGO_BODY, MANGO_STEM};
use crate::formatting::{format_bytes, format_duration};
use mangofetch_core::models::queue::QueueStatus;
use mangofetch_core::models::settings::AppSettings;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, Gauge, List, ListItem, Paragraph, Row, Table, Wrap},
};
use std::sync::Arc;

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn render(f: &mut Frame, app: &mut App) {
    // Fill background
    f.render_widget(Block::default().bg(app.theme.background), f.area());

    if matches!(app.state, AppState::Splash) {
        render_splash(f, app);
        return;
    }

    let area = f.area();
    let chunks = Layout::vertical([
        Constraint::Length(3), // header / tabs
        Constraint::Min(8),    // main content (table, settings, etc.)
        Constraint::Length(5), // detail panel + progress gauge
        Constraint::Length(8), // output panel (reporter events)
        Constraint::Length(1), // status bar
    ])
    .split(area);

    render_tabs(f, app, chunks[0]);
    render_main(f, app, chunks[1]);
    render_detail(f, app, chunks[2]);
    render_output(f, app, chunks[3]);
    render_statusbar(f, app, chunks[4]);

    // Overlays
    if app.show_help {
        render_help(f, app);
    }
    if app.mode == Mode::AddUrl {
        render_add_modal(f, app);
    }
    if app.mode == Mode::AddConfirm {
        render_add_confirm_modal(f, app);
    }
    if app.mode == Mode::ConfirmDelete {
        render_confirm_delete(f, app);
    }
}

// ── Splash ────────────────────────────────────────────────────────────────────

fn render_splash(f: &mut Frame, app: &App) {
    let area = f.area();
    let t = &app.theme;

    // Colors
    let orange = Color::Rgb(255, 160, 30);
    let gold = Color::Rgb(255, 220, 60);
    let green_stem = Color::Rgb(60, 200, 80);
    let green_leaf = Color::Rgb(30, 160, 50);
    let dim_border = t.surface;
    let accent = t.accent;
    let secondary = t.secondary;

    let stem_h = MANGO_STEM.len() as u16;
    let body_h = MANGO_BODY.len() as u16;
    let title_h = BLOCK_TITLE.len() as u16; // 5
                                            // total content height: stem + body + 1 gap + title + 1 gap + tagline + 1 gap + hint
    let content_h = stem_h + body_h + 1 + title_h + 1 + 1 + 1 + 1;
    let pad_top = area.height.saturating_sub(content_h) / 2;

    let chunks = Layout::vertical([
        Constraint::Length(pad_top),
        Constraint::Length(stem_h),
        Constraint::Length(body_h),
        Constraint::Length(1), // gap
        Constraint::Length(title_h),
        Constraint::Length(1), // gap
        Constraint::Length(1), // tagline
        Constraint::Length(1), // gap
        Constraint::Length(1), // hint
        Constraint::Min(0),
    ])
    .split(area);

    // ── Stem (green) ─────────────────────────────────────────────────────────
    let stem_text: Text = Text::from(
        MANGO_STEM
            .iter()
            .enumerate()
            .map(|(i, &line)| {
                let col = if i == 0 { green_stem } else { green_leaf };
                Line::from(Span::styled(line, Style::new().fg(col).bold()))
                    .alignment(Alignment::Center)
            })
            .collect::<Vec<_>>(),
    );
    f.render_widget(Paragraph::new(stem_text), chunks[1]);

    // ── Mango body (orange + gold highlight) ──────────────────────────────────
    let body_text: Text = Text::from(
        MANGO_BODY
            .iter()
            .map(|&line| {
                // Split each line on '░' zones → render those in gold
                let mut spans: Vec<Span> = Vec::new();
                let mut seg = String::new();
                let mut in_highlight = false;
                for ch in line.chars() {
                    let is_hl = ch == '░';
                    if is_hl != in_highlight {
                        if !seg.is_empty() {
                            let col = if in_highlight { gold } else { orange };
                            spans.push(Span::styled(seg.clone(), Style::new().fg(col).bold()));
                            seg.clear();
                        }
                        in_highlight = is_hl;
                    }
                    seg.push(if is_hl { '▒' } else { ch });
                }
                if !seg.is_empty() {
                    let col = if in_highlight { gold } else { orange };
                    spans.push(Span::styled(seg, Style::new().fg(col).bold()));
                }
                Line::from(spans).alignment(Alignment::Center)
            })
            .collect::<Vec<_>>(),
    );
    f.render_widget(Paragraph::new(body_text), chunks[2]);

    // ── Block title (MANGOFETCH) ───────────────────────────────────────────────
    let title_text: Text = Text::from(
        BLOCK_TITLE
            .iter()
            .enumerate()
            .map(|(i, &row)| {
                // Rows alternate accent / secondary for a gradient feel
                let col = if i % 2 == 0 { accent } else { secondary };
                Line::from(Span::styled(row, Style::new().fg(col).bold()))
                    .alignment(Alignment::Center)
            })
            .collect::<Vec<_>>(),
    );
    f.render_widget(Paragraph::new(title_text), chunks[4]);

    // ── Tagline ───────────────────────────────────────────────────────────────
    let tagline = Paragraph::new(Line::from(vec![
        Span::styled(" ⬇ ", Style::new().fg(accent)),
        Span::styled(
            format!("download anything  ·  1000+ platforms  ·  v{}", app.version),
            Style::new().fg(t.text_dim),
        ),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(tagline, chunks[6]);

    // ── Hint ─────────────────────────────────────────────────────────────────
    let hint = Paragraph::new(Line::from(vec![
        Span::styled("  [ ", Style::new().fg(dim_border)),
        Span::styled("any key", Style::new().fg(secondary).bold()),
        Span::styled(" ] start    [ ", Style::new().fg(dim_border)),
        Span::styled("q", Style::new().fg(t.error).bold()),
        Span::styled(" ] quit  ", Style::new().fg(dim_border)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[8]);
}

// ── Tabs / header ─────────────────────────────────────────────────────────────

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(" ", Style::new().fg(t.text_dim)));

    for (i, tab) in Tab::ALL.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" │ ", Style::new().fg(t.surface)));
        }

        let label = tab.label(nf);
        let key = format!("{}", i + 1);
        let is_active = i == app.active_tab.index();

        if is_active {
            // Active: inverted pill with accent background
            spans.push(Span::styled(
                format!(" {}{} ", key, label),
                Style::new().fg(t.background).bg(t.accent).bold(),
            ));
        } else {
            // Inactive: dim key number + label
            spans.push(Span::styled(
                format!(" {}", key),
                Style::new().fg(t.surface).dim(),
            ));
            spans.push(Span::styled(
                format!("{} ", label),
                Style::new().fg(t.text_dim),
            ));
        }
    }

    let header_title = format!(" 🥭 MangoFetch v{} ", app.version);

    let tabs_p = Paragraph::new(Line::from(spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .title(header_title)
            .title_style(Style::new().fg(t.accent).bold())
            .title(
                Line::from(vec![
                    Span::styled(" Shift+Tab ", Style::new().fg(t.text_dim)),
                    Span::styled("◂ ▸", Style::new().fg(t.accent)),
                    Span::styled(" Tab ", Style::new().fg(t.text_dim)),
                ])
                .alignment(Alignment::Right),
            )
            .border_style(Style::new().fg(t.surface)),
    );

    f.render_widget(tabs_p, area);
}

// ── Main content router ───────────────────────────────────────────────────────

fn render_main(f: &mut Frame, app: &mut App, area: Rect) {
    match app.active_tab {
        Tab::Home => render_home(f, app, area),
        Tab::Queue | Tab::History => render_queue_table(f, app, area),
        Tab::Logs => render_logs(f, app, area),
        Tab::Settings => render_settings(f, app, area),
        Tab::About => render_about(f, app, area),
    }
}

// ── Queue / History table ─────────────────────────────────────────────────────

fn render_queue_table(f: &mut Frame, app: &mut App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;
    let is_queue = app.active_tab == Tab::Queue;

    // Split area: top category tabs, then table
    let chunks = Layout::vertical([
        Constraint::Length(3), // Categories
        Constraint::Min(0),    // Table
    ])
    .split(area);

    render_categories(f, app, chunks[0]);
    let table_area = chunks[1];

    let h_binding = [
        if nf { "  ID" } else { " ID" },
        if nf { "󰋉  Platform" } else { " Platform" },
        if nf { "󰈙  Title" } else { " Title" },
        " Status",
        " Progress",
        if nf { "󰓅  Speed" } else { " Speed" },
        " ETA",
    ];
    let header_cells = h_binding
        .into_iter()
        .map(|h| Cell::from(h).style(Style::new().fg(t.secondary).bold()));

    let header = Row::new(header_cells)
        .style(Style::new().bg(t.surface))
        .height(1)
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .items
        .iter()
        .map(|item| {
            let (status_color, status_str) = status_display(item, nf, t);

            let bar_len = 10usize;
            let filled = ((item.percent / 100.0) * bar_len as f64).round() as usize;
            let filled = filled.min(bar_len);
            let bar = format!(
                "[{}{}] {:.0}%",
                "█".repeat(filled),
                "░".repeat(bar_len - filled),
                item.percent
            );

            let speed = if item.speed_bytes_per_sec > 0.0 {
                format!("{}/s", format_bytes(item.speed_bytes_per_sec as u64))
            } else {
                "-".into()
            };

            let eta = if item.speed_bytes_per_sec > 0.0 {
                if let Some(total) = item.total_bytes {
                    let remaining = total.saturating_sub(item.downloaded_bytes);
                    let secs = (remaining as f64 / item.speed_bytes_per_sec) as u64;
                    format_duration(secs)
                } else {
                    "-".into()
                }
            } else {
                "-".into()
            };

            let title = truncate(&item.title, 42);

            Row::new([
                Cell::from(item.id.to_string()).style(Style::new().fg(t.text_dim)),
                Cell::from(item.platform.clone()).style(Style::new().fg(t.secondary)),
                Cell::from(title),
                Cell::from(status_str).style(Style::new().fg(status_color)),
                Cell::from(bar).style(Style::new().fg(t.progress)),
                Cell::from(speed).style(Style::new().fg(t.accent)),
                Cell::from(eta).style(Style::new().fg(t.text_dim)),
            ])
            .style(Style::new().fg(t.text))
            .height(1)
        })
        .collect();

    let empty_msg = if is_queue {
        "  No active downloads.  Press 'a' to add a URL."
    } else {
        "  No download history yet."
    };

    let title = if is_queue {
        " ⬇ Active Downloads "
    } else {
        " 📜 History "
    };

    if app.items.is_empty() {
        let p = Paragraph::new(empty_msg)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::new().fg(t.text_dim)),
            )
            .style(Style::new().fg(t.text_dim))
            .alignment(Alignment::Center);
        f.render_widget(p, table_area);
        return;
    }

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Length(14),
            Constraint::Min(30),
            Constraint::Length(18),
            Constraint::Length(16),
            Constraint::Length(10),
            Constraint::Length(7),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::new().fg(t.text_dim)),
    )
    .row_highlight_style(Style::new().bg(t.highlight).bold())
    .highlight_symbol("▶ ");

    f.render_stateful_widget(table, table_area, &mut app.table_state);
}

// ── Logs panel ────────────────────────────────────────────────────────────────

fn render_logs(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;

    let items: Vec<ListItem> = app
        .log_lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let style = if i == app.log_scroll {
                Style::new().fg(t.accent)
            } else {
                Style::new().fg(t.text_dim)
            };
            ListItem::new(line.as_str()).style(style)
        })
        .collect();

    let offset = app
        .log_scroll
        .saturating_sub((area.height as usize).saturating_sub(4));
    let visible: Vec<ListItem> = items.into_iter().skip(offset).collect();

    let list = List::new(visible).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" 📋 Logs  ({} lines) ", app.log_lines.len()))
            .border_style(Style::new().fg(t.text_dim)),
    );
    f.render_widget(list, area);
}

// ── Settings panel ────────────────────────────────────────────────────────────

fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let settings = mangofetch_core::models::settings::AppSettings::load_from_disk();

    let rows: Vec<Row> = SettingKind::ALL
        .iter()
        .enumerate()
        .map(|(i, kind)| {
            let value = get_setting_value(kind, &settings, app);
            let is_sel = i == app.settings_index;

            let key_cell = Cell::from(kind.label()).style(if is_sel {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.secondary)
            });

            let val_cell = Cell::from(format!(" [ {} ] ", value)).style(if is_sel {
                Style::new().fg(t.text).bold().bg(t.highlight)
            } else {
                Style::new().fg(t.text)
            });

            let hint_cell =
                Cell::from(kind.description()).style(Style::new().fg(t.text_dim).italic());

            Row::new([key_cell, val_cell, hint_cell]).height(1)
        })
        .collect();

    let header = Row::new(
        ["Setting", "Value", "Description"]
            .map(|h| Cell::from(h).style(Style::new().fg(t.secondary).bold())),
    )
    .style(Style::new().bg(t.surface))
    .height(1)
    .bottom_margin(1);

    let table = Table::new(
        rows,
        [
            Constraint::Length(25),
            Constraint::Length(22),
            Constraint::Min(40),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" ⚙  Configuration & Appearance ")
            .border_style(Style::new().fg(t.surface)),
    );

    f.render_widget(table, area);
}

fn get_setting_value(kind: &SettingKind, s: &AppSettings, app: &App) -> String {
    match kind {
        SettingKind::TuiTheme => s.appearance.tui_theme.clone(),
        SettingKind::UseNerdFonts => {
            if app.use_nerd_fonts {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::MaxDownloads => s.advanced.max_concurrent_downloads.to_string(),
        SettingKind::VideoQuality => s.download.video_quality.clone(),
        SettingKind::OrganizeByPlatform => {
            if s.download.organize_by_platform {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::SkipExisting => {
            if s.download.skip_existing {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::DownloadSubtitles => {
            if s.download.download_subtitles {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::DownloadAttachments => {
            if s.download.download_attachments {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::DownloadDescriptions => {
            if s.download.download_descriptions {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::SponsorBlock => {
            if s.download.youtube_sponsorblock {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::SplitByChapters => {
            if s.download.split_by_chapters {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::EmbedMetadata => {
            if s.download.embed_metadata {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::EmbedThumbnail => {
            if s.download.embed_thumbnail {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::MaxConcurrentSegments => s.advanced.max_concurrent_segments.to_string(),
        SettingKind::MaxRetries => s.advanced.max_retries.to_string(),
        SettingKind::ConcurrentFragments => s.advanced.concurrent_fragments.to_string(),
        SettingKind::StaggerDelay => format!("{}ms", s.advanced.stagger_delay_ms),
        SettingKind::ClipboardDetection => {
            if s.download.clipboard_detection {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::ProxyEnabled => {
            if s.proxy.enabled {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::PortableMode => {
            if s.portable_mode {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
    }
}

// ── Detail panel ──────────────────────────────────────────────────────────────

fn render_detail(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;

    // Split: left details, right gauge
    let chunks = Layout::horizontal([Constraint::Min(0), Constraint::Length(28)]).split(area);

    let text = if let Some(idx) = app.table_state.selected() {
        if let Some(item) = app.items.get(idx) {
            let dl = format_bytes(item.downloaded_bytes);
            let total = item
                .total_bytes
                .map(format_bytes)
                .unwrap_or_else(|| "?".into());
            let path = item.file_path.as_deref().unwrap_or("—");
            format!(
                " 📌 {}\n 🔗 {}\n 📁 {}\n 💾 {} / {}  ·  Platform: {}",
                truncate(&item.title, 60),
                truncate(&item.url, 65),
                truncate(path, 60),
                dl,
                total,
                item.platform,
            )
        } else {
            " No item selected".into()
        }
    } else {
        match app.active_tab {
            Tab::Home => " Use ↑↓ to navigate actions, Enter to execute.".into(),
            Tab::Settings => " Use ↑↓ to navigate settings, Enter to cycle values.".into(),
            Tab::About => " Use ↑↓ to navigate sections.".into(),
            Tab::Logs => " ↑↓ or j/k to scroll  ·  G jumps to bottom  ·  :clear to clear".into(),
            _ => " Select an item above to see details.".into(),
        }
    };

    let detail = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Details ")
                .border_style(Style::new().fg(t.text_dim)),
        )
        .style(Style::new().fg(t.text))
        .wrap(Wrap { trim: true });
    f.render_widget(detail, chunks[0]);

    // Progress gauge for selected item
    let pct = app
        .table_state
        .selected()
        .and_then(|i| app.items.get(i))
        .map(|item| item.percent.clamp(0.0, 100.0) as u16)
        .unwrap_or(0);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Progress ")
                .border_style(Style::new().fg(t.text_dim)),
        )
        .gauge_style(Style::new().fg(t.progress).bg(t.surface_dim))
        .percent(pct)
        .label(Span::styled(
            format!("{pct}%"),
            Style::new().fg(t.text).bold(),
        ));
    f.render_widget(gauge, chunks[1]);
}

// ── Output panel (reporter events) ───────────────────────────────────────────

fn render_output(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;

    let line_count = app.output_lines.len();
    let visible_h = area.height.saturating_sub(2) as usize; // borders eat 2

    // Show the most recent lines that fit
    let start = line_count.saturating_sub(visible_h);
    let items: Vec<ListItem> = app
        .output_lines
        .iter()
        .skip(start)
        .map(|line| {
            // Color-code by prefix
            let style = if line.starts_with('✓') {
                Style::new().fg(t.success)
            } else if line.starts_with('✗') {
                Style::new().fg(t.error)
            } else if line.starts_with('↻') {
                Style::new().fg(t.warning)
            } else if line.starts_with('⟫') {
                Style::new().fg(t.secondary)
            } else if line.starts_with('◈') {
                Style::new().fg(t.accent)
            } else if line.starts_with('⚙') {
                Style::new().fg(t.progress)
            } else {
                Style::new().fg(t.text_dim)
            };
            ListItem::new(line.as_str()).style(style)
        })
        .collect();

    let title = format!(" ⚡ Output  ({} events) ", line_count);
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::new().fg(t.text_dim)),
    );
    f.render_widget(list, area);
}

// ── Status bar ────────────────────────────────────────────────────────────────

fn render_statusbar(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;

    if app.mode == Mode::Command {
        let prompt = Paragraph::new(format!(":{}", app.command_buffer))
            .style(Style::new().fg(t.accent).bg(t.surface).bold());
        f.render_widget(prompt, area);
        return;
    }

    let left = if let Some(msg) = &app.status_message {
        let color = if app.status_is_error {
            t.error
        } else {
            t.success
        };
        Span::styled(
            format!(" ✦ {} ", msg),
            Style::new().fg(t.background).bg(color).bold(),
        )
    } else {
        let shortcuts = match app.active_tab {
            Tab::Home => " ↑↓:navigate  Enter:execute  Tab:tab  q:quit",
            Tab::Queue | Tab::History => {
                " a:add  p:pause  r:resume  x:delete  /:cmd  Tab:cat  Tab:tab  ?:help  q:quit"
            }
            Tab::Settings => " ↑↓:navigate  Enter:cycle  Tab:tab  q:quit",
            Tab::About => " ↑↓:navigate  Tab:tab  q:quit",
            Tab::Logs => " ↑↓:scroll  G:bottom  :clear  q:quit",
        };
        Span::styled(shortcuts, Style::new().fg(t.text_dim).bg(t.surface_dim))
    };

    let right_str = format!(
        " {} active  v{}  [{}] ",
        app.active_count, app.version, app.current_time
    );
    let right = Span::styled(&right_str, Style::new().fg(t.text_dim).bg(t.surface_dim));

    let left_w = left.width() as u16;
    let right_w = right.width() as u16;
    let gap = area.width.saturating_sub(left_w + right_w + 1);

    let bar = Paragraph::new(Line::from(vec![
        left,
        Span::styled(" ".repeat(gap as usize), Style::new().bg(t.surface_dim)),
        right,
    ]));
    f.render_widget(bar, area);
}

// ── Help modal ────────────────────────────────────────────────────────────────

fn render_help(f: &mut Frame, app: &App) {
    let t = &app.theme;
    let area = centered_rect(62, 80, f.area());
    f.render_widget(Clear, area);

    let items = [
        ListItem::new("  General").style(Style::new().fg(t.secondary).bold()),
        ListItem::new("  q          Quit").style(Style::new().fg(t.text)),
        ListItem::new("  ?          Toggle this help").style(Style::new().fg(t.text)),
        ListItem::new("  : or /     Command mode  (:download <url>  :quit  :clear)")
            .style(Style::new().fg(t.text)),
        ListItem::new("  Tab        Next tab").style(Style::new().fg(t.text)),
        ListItem::new("  Shift+Tab  Previous tab").style(Style::new().fg(t.text)),
        ListItem::new("  1-4        Jump to tab").style(Style::new().fg(t.text)),
        ListItem::new(""),
        ListItem::new("  Queue / History").style(Style::new().fg(t.secondary).bold()),
        ListItem::new("  a / n      Add URL (opens dialog)").style(Style::new().fg(t.text)),
        ListItem::new("  ↑↓ / j k   Navigate items").style(Style::new().fg(t.text)),
        ListItem::new("  g / G      Jump to top / bottom").style(Style::new().fg(t.text)),
        ListItem::new("  p          Pause selected").style(Style::new().fg(t.text)),
        ListItem::new("  r          Resume selected").style(Style::new().fg(t.text)),
        ListItem::new("  x / Del    Delete selected (confirms)").style(Style::new().fg(t.text)),
        ListItem::new(""),
        ListItem::new("  Add URL Modal").style(Style::new().fg(t.secondary).bold()),
        ListItem::new("  Ctrl+V     Paste clipboard").style(Style::new().fg(t.text)),
        ListItem::new("  Tab        Switch field (URL → Quality)").style(Style::new().fg(t.text)),
        ListItem::new("  Enter      Confirm & enqueue").style(Style::new().fg(t.text)),
        ListItem::new("  Esc        Cancel").style(Style::new().fg(t.text)),
        ListItem::new(""),
        ListItem::new("  Logs").style(Style::new().fg(t.secondary).bold()),
        ListItem::new("  ↑↓         Scroll  ·  G to bottom").style(Style::new().fg(t.text)),
    ];

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" ❓ Help & Keybindings  (? to close) ")
            .border_style(Style::new().fg(t.accent)),
    );
    f.render_widget(list, area);
}

// ── Add URL modal ─────────────────────────────────────────────────────────────

fn render_add_modal(f: &mut Frame, app: &App) {
    let t = &app.theme;
    let area = centered_rect(60, 30, f.area());
    f.render_widget(Clear, area);

    let chunks = Layout::vertical([
        Constraint::Length(3), // URL input
        Constraint::Length(3), // Quality input
        Constraint::Length(1), // hint
    ])
    .margin(1)
    .split(area);

    // Outer block
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" ➕ Add Download  (Tab: switch field  ·  Enter: confirm  ·  Esc: cancel) ")
            .border_style(Style::new().fg(t.accent))
            .bg(t.background),
        area,
    );

    let url_style = if app.add_modal_field == 0 {
        Style::new().fg(t.accent)
    } else {
        Style::new().fg(t.text_dim)
    };
    let url_block = Block::default()
        .borders(Borders::ALL)
        .title(" URL (Ctrl+V to paste) ")
        .border_style(url_style);
    let url_p = Paragraph::new(app.url_input.as_str())
        .block(url_block)
        .style(Style::new().fg(t.text));
    f.render_widget(url_p, chunks[0]);

    let q_style = if app.add_modal_field == 1 {
        Style::new().fg(t.accent)
    } else {
        Style::new().fg(t.text_dim)
    };
    let q_block = Block::default()
        .borders(Borders::ALL)
        .title(" Quality (optional: best, 1080p, 720p…) ")
        .border_style(q_style);
    let q_p = Paragraph::new(app.quality_input.as_str())
        .block(q_block)
        .style(Style::new().fg(t.text));
    f.render_widget(q_p, chunks[1]);

    let hint =
        Paragraph::new("  Supported: YouTube, Instagram, TikTok, Twitch, torrents, and 1000+ more")
            .style(Style::new().fg(t.text_dim));
    f.render_widget(hint, chunks[2]);
}

// ── Confirm delete modal ──────────────────────────────────────────────────────

fn render_confirm_delete(f: &mut Frame, app: &App) {
    let t = &app.theme;
    let area = centered_rect(44, 20, f.area());
    f.render_widget(Clear, area);

    let title = app
        .table_state
        .selected()
        .and_then(|i| app.items.get(i))
        .map(|item| truncate(&item.title, 36))
        .unwrap_or_else(|| "this item".into());

    let text = format!(
        "\n  Remove \"{}\"?\n\n  y = Yes     any other key = No",
        title
    );
    let p = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ⚠ Confirm Remove ")
                .border_style(Style::new().fg(t.error)),
        )
        .style(Style::new().fg(t.text))
        .alignment(Alignment::Left);
    f.render_widget(p, area);
}

// ── New Tabs: Home, About, Categories ─────────────────────────────────────────

fn render_home(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    // Use a centered, comfortably sized area for the dashboard
    let dash_area = centered_rect(85, 75, area);

    // Split into Left (Logo) and Right (Info + Actions)
    let main_chunks = Layout::horizontal([
        Constraint::Length(35), // Space for logo
        Constraint::Min(45),    // Space for info and menu
    ])
    .split(dash_area);

    // ── Left: Large Logo ──────────────────────────────────────────────────
    let green_stem = Color::Rgb(60, 200, 80);
    let green_leaf = Color::Rgb(30, 160, 50);
    let orange = Color::Rgb(255, 160, 30);
    let gold = Color::Rgb(255, 220, 60);

    let mut logo_lines = Vec::new();

    // Top padding to vertically center the logo slightly
    for _ in 0..3 {
        logo_lines.push(Line::from(""));
    }

    for (i, &line) in super::assets::MANGO_STEM.iter().enumerate() {
        let col = if i == 0 { green_stem } else { green_leaf };
        logo_lines.push(
            Line::from(Span::styled(line, Style::new().fg(col).bold()))
                .alignment(Alignment::Center),
        );
    }

    for &line in super::assets::MANGO_BODY.iter() {
        let mut spans: Vec<Span> = Vec::new();
        let mut seg = String::new();
        let mut in_highlight = false;
        for ch in line.chars() {
            let is_hl = ch == '░';
            if is_hl != in_highlight {
                if !seg.is_empty() {
                    let col = if in_highlight { gold } else { orange };
                    spans.push(Span::styled(seg.clone(), Style::new().fg(col).bold()));
                    seg.clear();
                }
                in_highlight = is_hl;
            }
            seg.push(if is_hl { '▒' } else { ch });
        }
        if !seg.is_empty() {
            let col = if in_highlight { gold } else { orange };
            spans.push(Span::styled(seg, Style::new().fg(col).bold()));
        }
        logo_lines.push(Line::from(spans).alignment(Alignment::Center));
    }

    f.render_widget(Paragraph::new(logo_lines), main_chunks[0]);

    // ── Right: Info & Actions ─────────────────────────────────────────────
    let right_chunks = Layout::vertical([
        Constraint::Length(2), // Title
        Constraint::Length(2), // Tagline
        Constraint::Length(6), // Features
        Constraint::Length(2), // "Quick Actions" header
        Constraint::Min(8),    // Menu
    ])
    .split(main_chunks[1]);

    // Title
    let title = Line::from(vec![
        Span::styled("M A N G O ", Style::new().fg(t.accent).bold()),
        Span::styled("F E T C H", Style::new().fg(t.secondary).bold()),
        Span::styled(format!("  v{}", app.version), Style::new().fg(t.text_dim)),
    ]);
    f.render_widget(Paragraph::new(title), right_chunks[0]);

    // Tagline
    let tagline = Line::from(Span::styled(
        "The Universal Terminal Download Manager",
        Style::new().fg(t.text).italic(),
    ));
    f.render_widget(Paragraph::new(tagline), right_chunks[1]);

    // Features
    let check = if nf { "󰄬" } else { "✓" };
    let features = vec![
        Line::from(vec![
            Span::styled(format!(" {} ", check), Style::new().fg(t.success)),
            Span::styled(
                "Supports 1000+ platforms (YouTube, IG, TikTok...)",
                Style::new().fg(t.text),
            ),
        ]),
        Line::from(vec![
            Span::styled(format!(" {} ", check), Style::new().fg(t.success)),
            Span::styled("Torrents & Magnet Links", Style::new().fg(t.text)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {} ", check), Style::new().fg(t.success)),
            Span::styled("P2P File Sharing", Style::new().fg(t.text)),
        ]),
        Line::from(vec![
            Span::styled(format!(" {} ", check), Style::new().fg(t.success)),
            Span::styled("Subtitle & Metadata Extraction", Style::new().fg(t.text)),
        ]),
    ];
    f.render_widget(Paragraph::new(features), right_chunks[2]);

    // Quick Actions Header
    let actions_header = Line::from(Span::styled(
        "Quick Actions  (↑↓ navigate · Enter select)",
        Style::new().fg(t.text_dim).bold(),
    ));
    f.render_widget(Paragraph::new(actions_header), right_chunks[3]);

    // Menu
    let action_items = [
        (
            if nf { "󰄖" } else { "[A]" },
            "Add URL",
            "Download media from a URL",
        ),
        (
            if nf { "󰗀" } else { "[B]" },
            "Batch",
            "Download from a list of URLs",
        ),
        (
            if nf { "󰕒" } else { "[S]" },
            "P2P Send",
            "Share a local file peer-to-peer",
        ),
        (
            if nf { "󰄗" } else { "[T]" },
            "Torrent",
            "Open a magnet link or .torrent",
        ),
    ];

    let mut action_lines: Vec<Line> = Vec::new();
    for (i, (icon, name, desc)) in action_items.iter().enumerate() {
        let is_sel = i == app.home_index;
        if is_sel {
            action_lines.push(Line::from(vec![
                Span::styled(" ▸ ", Style::new().fg(t.accent).bold()),
                Span::styled(
                    format!(" {} {} ", icon, name),
                    Style::new().fg(t.background).bg(t.accent).bold(),
                ),
                Span::styled(format!("  {}", desc), Style::new().fg(t.text)),
            ]));
        } else {
            action_lines.push(Line::from(vec![
                Span::styled("   ", Style::new()),
                Span::styled(format!(" {} {} ", icon, name), Style::new().fg(t.text_dim)),
                Span::styled(format!("  {}", desc), Style::new().fg(t.surface)),
            ]));
        }
        action_lines.push(Line::from(""));
    }

    f.render_widget(Paragraph::new(action_lines), right_chunks[4]);
}

fn render_about(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    let sections = [
        (if nf { "󰋽 Info" } else { "Project Info" }),
        (if nf { "󰄗 Roadmap" } else { "Roadmap" }),
        (if nf { "󰄗 Changelog" } else { "Changelog" }),
        (if nf { "󰄬 Terms" } else { "Terms of Use" }),
        (if nf { "󰒓 Debug" } else { "Debug Details" }),
    ];

    let chunks = Layout::horizontal([Constraint::Length(20), Constraint::Min(0)]).split(area);

    // Sidebar
    let sidebar_items: Vec<ListItem> = sections
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let is_sel = i == app.about_index;
            let style = if is_sel {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.text_dim)
            };
            ListItem::new(format!("  {}  ", s)).style(style)
        })
        .collect();

    let sidebar = List::new(sidebar_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" About ")
            .border_style(Style::new().fg(t.text_dim)),
    );
    f.render_widget(sidebar, chunks[0]);

    // Content
    let theme_obj: Arc<dyn crate::reporter::CliTheme> =
        Arc::new(crate::reporter::BrutalistTheme::new(true));
    let text = match app.about_index {
        0 => crate::output::format_about_info(
            &app.version,
            "julesklord",
            "github.com/julesklord/mangofetch-cli",
            &theme_obj,
        ),
        1 => crate::output::format_about_roadmap(&theme_obj),
        2 => crate::output::format_about_changelog(&theme_obj),
        3 => crate::output::format_about_terms(&theme_obj),
        _ => format!(
            "\n  MangoFetch CLI v{}\n  Target: {}\n  Data Dir: {:?}\n",
            app.version,
            std::env::consts::OS,
            mangofetch_core::core::paths::app_data_dir().unwrap_or_default()
        ),
    };

    let p = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(t.text_dim)),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(p, chunks[1]);
}

fn render_categories(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;
    let mut spans: Vec<Span> = Vec::new();

    spans.push(Span::styled(" Filter: ", Style::new().fg(t.text_dim)));

    for (idx, cat) in DownloadsCategory::ALL.iter().enumerate() {
        if idx > 0 {
            spans.push(Span::styled("  ", Style::new().fg(t.surface)));
        }
        let is_sel = *cat == app.download_category;
        let count = match cat {
            DownloadsCategory::All => {
                app.active_count + app.queued_count + app.completed_count + app.failed_count
            }
            DownloadsCategory::Active => app.active_count,
            DownloadsCategory::Queued => app.queued_count,
            DownloadsCategory::Completed => app.completed_count,
            DownloadsCategory::Failed => app.failed_count,
        };

        let label = format!(" {} {} ", cat.label(nf), count);
        if is_sel {
            spans.push(Span::styled(
                label,
                Style::new().fg(t.background).bg(t.secondary).bold(),
            ));
        } else {
            spans.push(Span::styled(label, Style::new().fg(t.text_dim)));
        }
    }

    spans.push(Span::styled("   ", Style::new()));
    spans.push(Span::styled(" Tab ", Style::new().fg(t.surface).italic()));
    spans.push(Span::styled(
        "cycle filter ",
        Style::new().fg(t.text_dim).italic(),
    ));

    let p = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::new().fg(t.surface)),
        )
        .alignment(Alignment::Left);
    f.render_widget(p, area);
}

fn status_display(
    item: &mangofetch_core::models::queue::QueueItemInfo,
    nf: bool,
    t: &super::themes::Theme,
) -> (Color, String) {
    fn ic(nf: bool, nf_icon: &'static str, fb: &'static str) -> &'static str {
        if nf {
            nf_icon
        } else {
            fb
        }
    }
    match &item.status {
        QueueStatus::Queued => (t.text_dim, format!("{} Queued", ic(nf, "󰄗", "·"))),
        QueueStatus::Active => (
            t.accent,
            format!("{} {} {:.0}%", ic(nf, "󰄖", "▶"), item.phase, item.percent),
        ),
        QueueStatus::Paused => (t.warning, format!("{} Paused", ic(nf, "󰏤", "⏸"))),
        QueueStatus::Seeding => (t.secondary, format!("{} Seeding", ic(nf, "󰕒", "↑"))),
        QueueStatus::Complete { success } => {
            if *success {
                (t.success, format!("{} Done", ic(nf, "󰄬", "✓")))
            } else {
                (t.error, format!("{} Failed", ic(nf, "󰅖", "✗")))
            }
        }
        QueueStatus::Error { message } => (
            t.error,
            format!("{} Err: {}", ic(nf, "󰅚", "!"), truncate(message, 20)),
        ),
    }
}

fn render_add_confirm_modal(f: &mut Frame, app: &App) {
    let t = &app.theme;
    let area = centered_rect(60, 45, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🧐 Confirm Download ")
        .border_style(Style::new().fg(t.accent))
        .bg(t.background);
    f.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Min(3),    // Content
        Constraint::Length(3), // Actions
    ])
    .margin(2)
    .split(area);

    if app.is_fetching_preview {
        let loading = Paragraph::new("\n\n  🔍 Fetching media info...\n  Please wait.")
            .style(Style::new().fg(t.text_dim))
            .alignment(Alignment::Center);
        f.render_widget(loading, chunks[0]);
    } else if let Some(err) = &app.preview_error {
        let error = Paragraph::new(format!(
            "\n  ❌ Error: {}\n\n  You can still try to download, but some info might be missing.",
            err
        ))
        .style(Style::new().fg(t.error))
        .wrap(Wrap { trim: true });
        f.render_widget(error, chunks[0]);
    } else if let Some(info) = &app.preview_info {
        let lines = vec![
            Line::from(vec![
                Span::styled(" TITLE:    ", Style::new().fg(t.text_dim)),
                Span::styled(&info.title, Style::new().fg(t.text).bold()),
            ]),
            Line::from(vec![
                Span::styled(" PLATFORM: ", Style::new().fg(t.text_dim)),
                Span::styled(&info.platform, Style::new().fg(t.accent)),
            ]),
            Line::from(vec![
                Span::styled(" URL:      ", Style::new().fg(t.text_dim)),
                Span::styled(&info.url, Style::new().fg(t.text_dim)),
            ]),
        ];

        let info_p = Paragraph::new(lines).wrap(Wrap { trim: true });
        f.render_widget(info_p, chunks[0]);
    }

    let help = Paragraph::new(Line::from(vec![
        Span::styled("  [Enter] ", Style::new().fg(t.success).bold()),
        Span::raw(" Confirm & Download    "),
        Span::styled(" [Esc] ", Style::new().fg(t.error).bold()),
        Span::raw(" Cancel"),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(help, chunks[1]);
}

fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let vert = Layout::vertical([
        Constraint::Percentage((100 - pct_y) / 2),
        Constraint::Percentage(pct_y),
        Constraint::Percentage((100 - pct_y) / 2),
    ])
    .split(r);
    Layout::horizontal([
        Constraint::Percentage((100 - pct_x) / 2),
        Constraint::Percentage(pct_x),
        Constraint::Percentage((100 - pct_x) / 2),
    ])
    .split(vert[1])[1]
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
