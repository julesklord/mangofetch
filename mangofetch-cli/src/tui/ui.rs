use super::app::{App, AppState, DownloadsCategory, Mode, SettingKind, Tab};
use super::assets::{BLOCK_TITLE, MANGO_BODY, MANGO_STEM};
use crate::formatting::{format_bytes, format_duration};
use chrono::Local;
use mangofetch_core::models::queue::QueueStatus;
use mangofetch_core::models::settings::AppSettings;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap},
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

    // ARCHETYPE A: COMMAND STATION
    // Vertical split: [ Main Space (Min 5), Keybindings (Length 1), Dense Status (Length 1) ]
    let chunks = Layout::vertical([
        Constraint::Min(5),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(area);

    // 1-character horizontal padding from edges
    let main_padded = Rect {
        x: area.x + 1,
        y: area.y,
        width: area.width.saturating_sub(2),
        height: chunks[0].height,
    };

    if app.layout == "topbar" {
        let top_chunks =
            Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(main_padded);

        render_topbar(f, app, top_chunks[0]);
        render_main(f, app, top_chunks[1]);
    } else {
        let top_chunks =
            Layout::horizontal([Constraint::Length(24), Constraint::Min(0)]).split(main_padded);

        render_sidebar(f, app, top_chunks[0]);
        render_main(f, app, top_chunks[1]);
    }

    // Keybindings area padded
    let keybindings_padded = Rect {
        x: area.x + 1,
        y: chunks[1].y,
        width: area.width.saturating_sub(2),
        height: chunks[1].height,
    };
    render_keybindings(f, app, keybindings_padded);

    // Status area also padded
    let status_padded = Rect {
        x: area.x + 1,
        y: chunks[2].y,
        width: area.width.saturating_sub(2),
        height: chunks[2].height,
    };
    render_dense_statusbar(f, app, status_padded);

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
    let nf = app.use_nerd_fonts;

    // Colors
    let orange = Color::Rgb(255, 160, 30);
    let gold = Color::Rgb(255, 220, 60);
    let green_stem = Color::Rgb(60, 200, 80);
    let green_leaf = Color::Rgb(30, 160, 50);
    let accent = t.accent;
    let secondary = t.secondary;

    // Define a beautiful mechanical container centered on screen
    let splash_w = 72;
    let splash_h = 23;
    let margin_x = area.width.saturating_sub(splash_w) / 2;
    let margin_y = area.height.saturating_sub(splash_h) / 2;

    let centered_splash = Rect {
        x: area.x + margin_x,
        y: area.y + margin_y,
        width: splash_w.min(area.width),
        height: splash_h.min(area.height),
    };

    // Render container block with mechanical corners and cybernetic style
    let container = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(t.surface))
        .title(Span::styled(
            " MANGOFETCH CORE TERMINAL v0.5.x ",
            Style::new().fg(t.accent).bold(),
        ));

    f.render_widget(container.clone(), centered_splash);
    let inner = container.inner(centered_splash);

    // Split inner area of container
    let stem_h = MANGO_STEM.len() as u16;
    let body_h = MANGO_BODY.len() as u16;
    let title_h = BLOCK_TITLE.len() as u16; // 5

    // Split vertical chunks within the inner block (height 21)
    let chunks = Layout::vertical([
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
    .split(inner);

    // Stem
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
    f.render_widget(Paragraph::new(stem_text), chunks[0]);

    // Body
    let body_text: Text = Text::from(
        MANGO_BODY
            .iter()
            .map(|&line| {
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
    f.render_widget(Paragraph::new(body_text), chunks[1]);

    // Title
    let title_text: Text = Text::from(
        BLOCK_TITLE
            .iter()
            .enumerate()
            .map(|(i, &row)| {
                let col = if i % 2 == 0 { accent } else { secondary };
                Line::from(Span::styled(row, Style::new().fg(col).bold()))
                    .alignment(Alignment::Center)
            })
            .collect::<Vec<_>>(),
    );
    f.render_widget(Paragraph::new(title_text), chunks[3]);

    // Tagline
    let splash_icon = if nf { " 󰄖 " } else { " v " };
    let separator = if nf { " │ " } else { " │ " };
    let tagline = Paragraph::new(Line::from(vec![
        Span::styled(splash_icon, Style::new().fg(accent)),
        Span::styled(
            format!(
                "download anything{}1000+ platforms{}v{}",
                separator, separator, app.version
            ),
            Style::new().fg(t.text_dim),
        ),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(tagline, chunks[5]);

    // Hint / Mechanical Initializer block
    let enter_prompt = if nf { " 󰌑 ANY KEY " } else { " ANY KEY " };
    let quit_prompt = if nf { " 󱊷 Q " } else { " Q " };

    let hint = Paragraph::new(Line::from(vec![
        Span::styled(
            enter_prompt,
            Style::new().fg(t.background).bg(t.accent).bold(),
        ),
        Span::styled(" INITIALIZE SYSTEMS", Style::new().fg(t.accent).bold()),
        Span::styled("     ", Style::new()),
        Span::styled(
            quit_prompt,
            Style::new().fg(t.background).bg(t.error).bold(),
        ),
        Span::styled(" ABORT", Style::new().fg(t.error).bold()),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(hint, chunks[7]);
}

// ── Main content router ───────────────────────────────────────────────────────

fn render_main(f: &mut Frame, app: &mut App, area: Rect) {
    // If we are in the 'Logs' (now Output) tab, we show the full screen output
    if app.active_tab == Tab::Logs {
        render_logs(f, app, area);
        return;
    }

    // Split the main area into Content (Min) and Terminal Output (Fixed height)
    let chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(6), // Reduced terminal height for more content space
    ])
    .split(area);

    match app.active_tab {
        Tab::Home => render_home(f, app, chunks[0]),
        Tab::Queue | Tab::History => render_queue_table(f, app, chunks[0]),
        Tab::Settings => render_settings(f, app, chunks[0]),
        Tab::About => render_about(f, app, chunks[0]),
        _ => {}
    }

    // Render the persistent terminal output
    render_terminal_output(f, app, chunks[1]);
}

fn render_terminal_output(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;

    // Use a high-density industrial block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📡 TERMINAL_OUTPUT ")
        .title_style(Style::new().fg(t.accent).bold())
        .border_style(Style::new().fg(t.surface));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Get the last N lines that fit in the area
    let visible_height = inner.height as usize;
    let lines: Vec<ListItem> = app
        .log_lines
        .iter()
        .rev()
        .take(visible_height)
        .rev()
        .map(|line| {
            let style = if line.contains("ERROR") || line.contains("✗") {
                Style::new().fg(t.error)
            } else if line.contains("SUCCESS") || line.contains("✓") {
                Style::new().fg(t.success)
            } else if line.contains("CMD") || line.contains("❯") {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.text_dim)
            };
            ListItem::new(Line::from(vec![
                Span::styled(" ", Style::new()), // left margin
                Span::styled(line.as_str(), style),
            ]))
        })
        .collect();

    let list = List::new(lines);
    f.render_widget(list, inner);
}

// ── Queue / History table ─────────────────────────────────────────────────────

fn render_queue_table(f: &mut Frame, app: &mut App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;
    let is_queue = app.active_tab == Tab::Queue;

    let (margin, cat_height) = if app.layout == "topbar" {
        (0, 2)
    } else {
        (1, 3)
    };

    // Split area: top gap, category tabs, then table
    let chunks = Layout::vertical([
        Constraint::Length(margin),     // Top margin
        Constraint::Length(cat_height), // Categories
        Constraint::Min(0),             // Table
    ])
    .split(area);

    render_categories(f, app, chunks[1]);
    let table_area = chunks[2];

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
            let bar_line = Line::from(vec![
                Span::styled("[", Style::new().fg(t.surface)),
                Span::styled("█".repeat(filled), Style::new().fg(t.progress)),
                Span::styled(
                    "░".repeat(bar_len.saturating_sub(filled)),
                    Style::new().fg(t.surface_dim),
                ),
                Span::styled("]", Style::new().fg(t.surface)),
                Span::styled(
                    format!(" {:.0}%", item.percent),
                    Style::new().fg(t.text).bold(),
                ),
            ]);

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
                Cell::from(bar_line),
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
        if nf {
            " 󰄖 Active Downloads "
        } else {
            " Active Downloads "
        }
    } else {
        if nf {
            " 󰄗 History "
        } else {
            " History "
        }
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

            let prefix = if is_sel { "▶ " } else { "  " };
            let key_cell = Cell::from(format!("{}{}", prefix, kind.label())).style(if is_sel {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.text_dim)
            });

            let val_cell = Cell::from(format!(" ◀ {} ▶ ", value)).style(if is_sel {
                Style::new().fg(t.background).bg(t.accent).bold()
            } else {
                Style::new().fg(t.secondary)
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
    let get_statusbar_val = |name: &str| {
        if let Some(pos) = app.statusbar_modules.iter().position(|m| m == name) {
            format!("ACTIVE (Pos {})", pos + 1)
        } else {
            "DISABLED".to_string()
        }
    };

    match kind {
        SettingKind::TuiTheme => s.appearance.tui_theme.clone(),
        SettingKind::UseNerdFonts => {
            if app.use_nerd_fonts {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::EnableAnimations => {
            if app.enable_animations {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::NavigationLayout => app.layout.to_uppercase(),
        SettingKind::MaxDownloads => s.advanced.max_concurrent_downloads.to_string(),
        SettingKind::VideoQuality => s.download.video_quality.clone(),
        SettingKind::AlwaysAskConfirm => {
            if s.download.always_ask_confirm {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
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
        SettingKind::StatusbarMode => get_statusbar_val("mode"),
        SettingKind::StatusbarTab => get_statusbar_val("tab"),
        SettingKind::StatusbarRadar => get_statusbar_val("radar"),
        SettingKind::StatusbarCpu => get_statusbar_val("cpu"),
        SettingKind::StatusbarRam => get_statusbar_val("ram"),
        SettingKind::StatusbarSpeed => get_statusbar_val("speed"),
        SettingKind::StatusbarQueue => get_statusbar_val("queue"),
        SettingKind::StatusbarTime => get_statusbar_val("time"),
    }
}

// ── Help modal ────────────────────────────────────────────────────────────────

fn render_help(f: &mut Frame, app: &App) {
    let t = &app.theme;
    let area = centered_rect(62, 80, f.area());
    f.render_widget(Clear, area);

    let kb = |key: &str, desc: &str| -> ListItem<'static> {
        ListItem::new(Line::from(vec![
            Span::styled(format!("  {: <10} ", key), Style::new().fg(t.accent).bold()),
            Span::styled(desc.to_string(), Style::new().fg(t.text)),
        ]))
    };
    let section = |name: &str| -> ListItem<'static> {
        ListItem::new(Line::from(vec![Span::styled(
            format!("  {}  ", name.to_uppercase()),
            Style::new().fg(t.secondary).bold(),
        )]))
    };
    let items = vec![
        section("General"),
        kb("q", "Quit application"),
        kb("?", "Toggle this help overlay"),
        kb("l / L", "Toggle layout (Sidebar / TopBar)"),
        kb(": or /", "Open command mode prompt"),
        kb("Tab", "Switch to the next tab"),
        kb("Shift+Tab", "Switch to the previous tab"),
        kb("1-6", "Directly jump to tab number"),
        ListItem::new(""),
        section("Queue & History"),
        kb("a / n", "Add new media URL"),
        kb("↑↓ / j k", "Navigate item list"),
        kb("g / G", "Jump to top / bottom of list"),
        kb("p", "Pause the selected download"),
        kb("r", "Resume the selected download"),
        kb("x / Del", "Remove the selected item"),
        ListItem::new(""),
        section("Add URL Modal"),
        kb("Ctrl+V", "Paste URL from clipboard"),
        kb("Tab", "Switch input field (URL ⇄ Quality)"),
        kb("Enter", "Confirm and start download"),
        kb("Esc", "Close overlay dialog"),
        ListItem::new(""),
        section("Output Logs"),
        kb("↑↓", "Scroll logs list"),
        kb("G", "Jump to latest log line"),
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

    let title = if app.use_nerd_fonts {
        " 󰄖 Add Download  (Tab: switch field  │  Enter: confirm  │  Esc: cancel) "
    } else {
        " Add Download  (Tab: switch field  │  Enter: confirm  │  Esc: cancel) "
    };

    // Outer block
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
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
    let url_val = if app.add_modal_field == 0 {
        format!("{}█", app.url_input)
    } else {
        app.url_input.clone()
    };
    let url_p = Paragraph::new(url_val)
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
    let q_val = if app.add_modal_field == 1 {
        format!("{}█", app.quality_input)
    } else {
        app.quality_input.clone()
    };
    let q_p = Paragraph::new(q_val)
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

    let delete_title = if app.use_nerd_fonts {
        " 󰅚 Confirm Remove "
    } else {
        " ! Confirm Remove "
    };

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  REMOVE \"", Style::new().fg(t.text)),
            Span::styled(title, Style::new().fg(t.accent).bold()),
            Span::styled("\"?", Style::new().fg(t.text)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  [Y] YES  ",
                Style::new().fg(t.background).bg(t.error).bold(),
            ),
            Span::styled("   ", Style::new()),
            Span::styled(
                "  [ANY OTHER KEY] NO  ",
                Style::new().fg(t.text).bg(t.surface).bold(),
            ),
        ]),
    ];
    let p = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(delete_title)
                .border_style(Style::new().fg(t.error)),
        )
        .alignment(Alignment::Left);
    f.render_widget(p, area);
}

// ── New Tabs: Home, About, Categories ─────────────────────────────────────────

fn render_home(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    // Outer border block representing the home panel container
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::new().fg(t.surface));
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Center the content horizontally and vertically inside the central workspace (orbital)
    let home_w = 85;
    let home_h = 21;
    let margin_x = inner_area.width.saturating_sub(home_w) / 2;
    let margin_y = inner_area.height.saturating_sub(home_h) / 2;

    let centered_vertical = Layout::vertical([
        Constraint::Length(margin_y),
        Constraint::Length(home_h.min(inner_area.height)),
        Constraint::Length(margin_y),
    ])
    .split(inner_area)[1];

    let centered_area = Layout::horizontal([
        Constraint::Length(margin_x),
        Constraint::Length(home_w.min(inner_area.width)),
        Constraint::Length(margin_x),
    ])
    .split(centered_vertical)[1];

    // Split centered area into Left (Logo) and Right (Info + Actions)
    let main_chunks = Layout::horizontal([
        Constraint::Length(35), // Space for logo
        Constraint::Min(0),     // Space for info and menu
    ])
    .split(centered_area);

    // ── Left: Mango Art (Center aligned in its column) ────────────────────
    let orange = Color::Rgb(255, 160, 30);
    let gold = Color::Rgb(255, 220, 60);
    let green_stem = Color::Rgb(60, 200, 80);
    let green_leaf = Color::Rgb(30, 160, 50);

    let mut logo_lines = Vec::new();
    for _ in 0..2 {
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
        Constraint::Min(0),    // Menu
    ])
    .margin(1)
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
                Span::styled(" ┣━❯ ", Style::new().fg(t.accent).bold()),
                Span::styled(
                    format!(" {} {} ", icon, name.to_uppercase()),
                    Style::new().fg(t.background).bg(t.accent).bold(),
                ),
                Span::styled(format!("  {}", desc), Style::new().fg(t.text).bold()),
            ]));
        } else {
            action_lines.push(Line::from(vec![
                Span::styled(" ┃   ", Style::new().fg(t.surface_dim)),
                Span::styled(
                    format!(" {} {} ", icon, name.to_uppercase()),
                    Style::new().fg(t.text_dim),
                ),
                Span::styled(format!("  {}", desc), Style::new().fg(t.text_dim)),
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
            if is_sel {
                ListItem::new(Line::from(vec![
                    Span::styled(" ┣━❯ ", Style::new().fg(t.accent).bold()),
                    Span::styled(
                        format!(" {} ", s.to_uppercase()),
                        Style::new().fg(t.background).bg(t.accent).bold(),
                    ),
                ]))
            } else {
                ListItem::new(Line::from(vec![
                    Span::styled(" ┃   ", Style::new().fg(t.surface_dim)),
                    Span::styled(s.to_uppercase(), Style::new().fg(t.text_dim)),
                ]))
            }
        })
        .collect();

    let sidebar = List::new(sidebar_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" ABOUT ")
            .border_style(Style::new().fg(t.surface)),
    );
    f.render_widget(sidebar, chunks[0]);

    // Content
    let theme_obj: Arc<dyn crate::reporter::CliTheme> =
        Arc::new(crate::reporter::PlainTheme::new(true));
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
                .border_style(Style::new().fg(t.surface)),
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

    let alignment = if app.layout == "topbar" {
        Alignment::Center
    } else {
        Alignment::Left
    };

    let p = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::new().fg(t.surface)),
        )
        .alignment(alignment);
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
    let nf = app.use_nerd_fonts;
    let area = centered_rect(60, 45, f.area());
    f.render_widget(Clear, area);

    let preview_title = if nf {
        " 󰄖 DOWNLOAD_PREVIEW "
    } else {
        " DOWNLOAD_PREVIEW "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(preview_title)
        .title_style(Style::new().fg(t.accent).bold())
        .border_style(Style::new().fg(t.surface))
        .bg(t.background);
    f.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Min(0),    // Content
        Constraint::Length(3), // Actions
    ])
    .margin(2)
    .split(area);

    if app.is_fetching_preview {
        let loading = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  ░▒▓ ", Style::new().fg(t.accent)),
                Span::styled(
                    "SYNAPSE_HANDSHAKE_IN_PROGRESS",
                    Style::new().fg(t.text).bold(),
                ),
                Span::styled(" ▓▒░", Style::new().fg(t.accent)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  Attempting to fetch remote metadata... Please wait.",
                Style::new().fg(t.text_dim),
            )),
        ])
        .alignment(Alignment::Center);
        f.render_widget(loading, chunks[0]);
    } else if let Some(err) = &app.preview_error {
        let err_icon = if nf { " 󰅖 " } else { " ! " };
        let error = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(err_icon, Style::new().fg(t.background).bg(t.error).bold()),
                Span::styled(" PROTOCOL_ERROR ", Style::new().fg(t.error).bold()),
            ]),
            Line::from(""),
            Line::from(Span::styled(format!("  {}", err), Style::new().fg(t.text))),
            Line::from(""),
            Line::from(Span::styled(
                "  [System bypass available: Press ENTER to force queue]",
                Style::new().fg(t.secondary).italic(),
            )),
        ])
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(error, chunks[0]);
    } else if let Some(info) = &app.preview_info {
        let content_chunks = Layout::vertical([
            Constraint::Length(6), // Info header
            Constraint::Min(4),    // Qualities list
            Constraint::Length(4), // Options
        ])
        .split(chunks[0]);

        let title_label = if nf { " 󰄖 TARGET " } else { " [TARGET] " };
        let platform_label = if nf { " 󰏘 SOURCE " } else { " [SOURCE] " };
        let url_label = if nf { " 󰌷 URL    " } else { " [URL]    " };
        let status_label = if nf { " 󰔛 STATUS " } else { " [STATUS] " };

        let info_lines = vec![
            Line::from(vec![
                Span::styled(title_label, Style::new().fg(t.accent).bold()),
                Span::styled(" ", Style::new()),
                Span::styled(truncate(&info.title, 50), Style::new().fg(t.text).bold()),
            ]),
            Line::from(vec![
                Span::styled(platform_label, Style::new().fg(t.secondary).bold()),
                Span::styled(" ", Style::new()),
                Span::styled(info.platform.to_uppercase(), Style::new().fg(t.text)),
            ]),
            Line::from(vec![
                Span::styled(url_label, Style::new().fg(t.text_dim)),
                Span::styled(" ", Style::new()),
                Span::styled(truncate(&app.url_input, 50), Style::new().fg(t.text_dim)),
            ]),
            Line::from(vec![
                Span::styled(status_label, Style::new().fg(t.text_dim)),
                Span::styled(" ", Style::new()),
                Span::styled(
                    " READY_FOR_TRANSFER ",
                    Style::new().bg(t.success).fg(t.background).bold(),
                ),
            ]),
        ];

        let info_p = Paragraph::new(info_lines).wrap(Wrap { trim: true });
        f.render_widget(info_p, content_chunks[0]);

        // Qualities
        let mut q_items = vec![];
        if info.available_qualities.is_empty() {
            q_items.push(ListItem::new(Line::from(Span::styled("  No specific qualities found. Will use default/best.", Style::new().fg(t.text_dim)))));
        } else {
            for (i, q) in info.available_qualities.iter().enumerate() {
                let prefix = if i == app.confirm_quality_idx {
                    if app.confirm_focused_field == 0 { "  ▶ " } else { "  ▷ " }
                } else {
                    "    "
                };
                let style = if i == app.confirm_quality_idx {
                    if app.confirm_focused_field == 0 {
                        Style::new().fg(t.background).bg(t.accent).bold()
                    } else {
                        Style::new().fg(t.accent)
                    }
                } else {
                    Style::new().fg(t.text)
                };
                
                let size_str = if let Some(bytes) = q.filesize_bytes {
                    format!(" ({:.1} MB)", bytes as f64 / 1_048_576.0)
                } else {
                    String::new()
                };

                let label = format!("{}{} ({}x{}){}", prefix, q.label, q.width, q.height, size_str);
                q_items.push(ListItem::new(Line::from(Span::styled(label, style))));
            }
        }
        
        let q_list = ratatui::widgets::List::new(q_items)
            .block(Block::default().borders(Borders::TOP).title(" Resoluciones Disponibles ").border_style(Style::new().fg(t.surface)));
        f.render_widget(q_list, content_chunks[1]);

        // Options
        let opt_style_1 = if app.confirm_focused_field == 1 { Style::new().bg(t.secondary).fg(t.background).bold() } else { Style::new().fg(t.text) };
        let opt_style_2 = if app.confirm_focused_field == 2 { Style::new().bg(t.secondary).fg(t.background).bold() } else { Style::new().fg(t.text) };
        
        let format_lbl = format!(" Format: < {} > ", app.confirm_download_mode.to_uppercase());
        let subs_lbl = format!(" Subtitles: < {} > ", if app.confirm_download_subtitles { "YES" } else { "NO" });

        let opts_lines = vec![
            Line::from(vec![
                Span::styled(if app.confirm_focused_field == 1 { " ▶" } else { "  " }, Style::new().fg(t.secondary)),
                Span::styled(format_lbl, opt_style_1),
            ]),
            Line::from(vec![
                Span::styled(if app.confirm_focused_field == 2 { " ▶" } else { "  " }, Style::new().fg(t.secondary)),
                Span::styled(subs_lbl, opt_style_2),
            ]),
        ];
        let opts_p = Paragraph::new(opts_lines)
            .block(Block::default().borders(Borders::TOP).title(" Opciones Extras ").border_style(Style::new().fg(t.surface)));
        f.render_widget(opts_p, content_chunks[2]);
    }

    let enter_key = if nf { " 󰌑 ENTER " } else { " ENTER " };
    let esc_key = if nf { " 󱊷 ESC " } else { " ESC " };

    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            enter_key,
            Style::new().fg(t.background).bg(t.success).bold(),
        ),
        Span::styled(" INIT_DOWNLOAD", Style::new().fg(t.success).bold()),
        Span::styled("    ", Style::new()),
        Span::styled(esc_key, Style::new().fg(t.background).bg(t.error).bold()),
        Span::styled(" ABORT", Style::new().fg(t.error).bold()),
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

// ── TropicalUI Components ───────────────────────────────────────────────────

fn render_topbar(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    // TopBar block with borders
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::new().fg(t.accent));
    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split inner area into: [ Logo/Identity (Length 25), Tabs (Min 0), Status (Length 20) ]
    let cols = Layout::horizontal([
        Constraint::Length(25),
        Constraint::Min(0),
        Constraint::Length(20),
    ])
    .split(inner);

    // 1. Logo & Identity
    let logo_span = if nf {
        Span::styled("󰄖 MANGOFETCH", Style::new().fg(t.accent).bold())
    } else {
        Span::styled("MANGOFETCH", Style::new().fg(t.accent).bold())
    };
    let logo_para = Paragraph::new(Line::from(vec![
        logo_span,
        Span::styled(format!(" v{}", app.version), Style::new().fg(t.text_dim)),
    ]))
    .alignment(Alignment::Left);
    f.render_widget(logo_para, cols[0]);

    // 2. Navigation Tabs (Horizontal)
    let mut nav_spans = Vec::new();
    for (i, tab) in Tab::ALL.iter().enumerate() {
        let is_active = i == app.active_tab.index();
        let label = match tab {
            Tab::Home => "OVERVIEW",
            Tab::Queue => "ACTIVE",
            Tab::History => "HISTORY",
            Tab::Settings => "SETTINGS",
            Tab::About => "ABOUT",
            Tab::Logs => "OUTPUT",
        };

        if i > 0 {
            nav_spans.push(Span::styled(" │ ", Style::new().fg(t.surface_dim)));
        }

        if is_active {
            nav_spans.push(Span::styled(
                format!(" █ {} ", label),
                Style::new().fg(t.background).bg(t.accent).bold(),
            ));
        } else {
            nav_spans.push(Span::styled(
                format!("  {} ", label),
                Style::new().fg(t.text_dim),
            ));
        }
    }

    let tabs_para = Paragraph::new(Line::from(nav_spans)).alignment(Alignment::Center);
    f.render_widget(tabs_para, cols[1]);

    // 3. Status/Active Info
    let (status_dot, status_color, status_text) = if app.active_count > 0 {
        ("●", t.accent, "TRANSFERRING")
    } else if app.queued_count > 0 {
        ("●", t.warning, "QUEUED")
    } else {
        ("●", t.success, "SYSTEM IDLE")
    };

    let status_para = Paragraph::new(Line::from(vec![
        Span::styled(status_dot, Style::new().fg(status_color)),
        Span::styled(format!(" {}", status_text), Style::new().fg(t.text).bold()),
    ]))
    .alignment(Alignment::Right);
    f.render_widget(status_para, cols[2]);
}

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    // Sidebar block with right border in brand color
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::new().fg(t.accent));
    let inner = block.inner(area);
    f.render_widget(block, area);

    let side_chunks = Layout::vertical([
        Constraint::Length(10), // Logo + Identity
        Constraint::Min(0),     // Nav area
    ])
    .split(inner);

    // ── TUI_IDENTITY ───────────────────────────────────────────────────────
    let logo_line = if nf {
        Line::from(vec![
            Span::styled("󰄖 ", Style::new().fg(t.accent).bold()),
            Span::styled("MANGO", Style::new().fg(t.accent).bold()),
            Span::styled("FETCH", Style::new().fg(t.secondary).bold()),
        ])
    } else {
        Line::from(vec![
            Span::styled("MANGO", Style::new().fg(t.accent).bold()),
            Span::styled("FETCH", Style::new().fg(t.secondary).bold()),
        ])
    };

    let separator_line = Line::from(vec![Span::styled(
        "────────────────────",
        Style::new().fg(t.surface_dim),
    )]);

    let stats_line = Line::from(vec![
        Span::styled(format!("v{}", app.version), Style::new().fg(t.text_dim)),
        Span::styled(" │ ", Style::new().fg(t.surface_dim)),
        Span::styled(
            std::env::consts::OS.to_uppercase(),
            Style::new().fg(t.text_dim),
        ),
    ]);

    let (status_dot, status_color, status_text) = if app.active_count > 0 {
        ("●", t.accent, "TRANSFERRING")
    } else if app.queued_count > 0 {
        ("●", t.warning, "QUEUED")
    } else {
        ("●", t.success, "SYSTEM IDLE")
    };

    let status_line = Line::from(vec![
        Span::styled(status_dot, Style::new().fg(status_color)),
        Span::styled(format!(" {} ", status_text), Style::new().fg(t.text).bold()),
    ]);

    let identity_lines = vec![
        Line::from(""), // top padding
        logo_line.alignment(Alignment::Center),
        separator_line.alignment(Alignment::Center),
        stats_line.alignment(Alignment::Center),
        Line::from(""), // spacing
        status_line.alignment(Alignment::Center),
    ];

    f.render_widget(Paragraph::new(identity_lines), side_chunks[0]);

    // ── TUI_NAV_ORBIT ───────────────────────────────────────────────────────
    let mut nav_lines = Vec::new();
    nav_lines.push(Line::from(""));

    for (i, tab) in Tab::ALL.iter().enumerate() {
        let is_active = i == app.active_tab.index();
        let label = match tab {
            Tab::Home => "Overview",
            Tab::Queue => "Active",
            Tab::History => "History",
            Tab::Settings => "Settings",
            Tab::About => "About",
            Tab::Logs => "Output",
        };
        let num = format!("{:02}", i + 1);

        if is_active {
            nav_lines.push(Line::from(vec![
                Span::styled(" ┣━❯ ", Style::new().fg(t.accent).bold()),
                Span::styled(
                    format!("{}. {} ", num, label.to_uppercase()),
                    Style::new().fg(t.background).bg(t.accent).bold(),
                ),
            ]));
        } else {
            nav_lines.push(Line::from(vec![
                Span::styled(" ┃   ", Style::new().fg(t.surface_dim)),
                Span::styled(
                    format!("{}. {}", num, label.to_uppercase()),
                    Style::new().fg(t.text_dim),
                ),
            ]));
        }
        nav_lines.push(Line::from(""));
    }

    let nav_orbit = Paragraph::new(nav_lines).block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::new().fg(t.background)),
    );

    f.render_widget(nav_orbit, side_chunks[1]);
}

fn render_keybindings(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;

    let keybindings = vec![
        ("a", "add"),
        ("p", "pause"),
        ("r", "resume"),
        ("x", "delete"),
        ("/", "cmd"),
        ("l", "layout"),
        ("Tab", "tab"),
        ("Esc", "clear"),
        ("?", "help"),
        ("q", "quit"),
    ];

    let mut left_spans: Vec<Span> = Vec::new();
    for (i, (key, action)) in keybindings.iter().enumerate() {
        if i > 0 {
            left_spans.push(Span::styled(" │ ", Style::new().fg(t.surface_dim)));
        }
        left_spans.push(Span::styled(
            format!(" {} ", key),
            Style::new().fg(t.secondary).bold(),
        ));
        left_spans.push(Span::styled(action.to_uppercase(), Style::new().fg(t.text)));
    }

    // System status indicator tags (Right-aligned)
    let layout_tag = if app.layout == "topbar" {
        "TOPBAR"
    } else {
        "SIDEBAR"
    };
    let nf_tag = if app.use_nerd_fonts {
        "NF:ON"
    } else {
        "NF:OFF"
    };
    let theme_tag = app.theme_name.to_uppercase();

    let bracket_style = Style::new().fg(t.surface_dim);
    let right_spans = vec![
        Span::styled(" [ ", bracket_style),
        Span::styled(
            format!("LAY:{}", layout_tag),
            Style::new().fg(t.accent).bold(),
        ),
        Span::styled(" │ ", bracket_style),
        Span::styled(
            format!("FONTS:{}", nf_tag),
            Style::new().fg(t.secondary).bold(),
        ),
        Span::styled(" │ ", bracket_style),
        Span::styled(
            format!("THEME:{}", theme_tag),
            Style::new().fg(t.text).bold(),
        ),
        Span::styled(" ] ", bracket_style),
    ];

    let left_w: u16 = left_spans.iter().map(|s| s.width() as u16).sum();
    let right_w: u16 = right_spans.iter().map(|s| s.width() as u16).sum();
    let gap = area.width.saturating_sub(left_w + right_w);

    let mut all_spans = left_spans;
    if gap > 0 {
        all_spans.push(Span::raw(" ".repeat(gap as usize)));
    }
    all_spans.extend(right_spans);

    f.render_widget(Paragraph::new(Line::from(all_spans)), area);
}

fn render_dense_statusbar(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    let is_downloading = app.total_speed > 0.0 || app.active_count > 0;
    let millisecond = Local::now().timestamp_subsec_millis();

    let mut left_spans = Vec::new();
    let mut right_spans = Vec::new();

    // Start Left with a leading block
    left_spans.push(Span::styled(" █ ", Style::new().fg(t.background)));

    let active_count = app.statusbar_modules.len();
    let left_limit = active_count / 2;

    for (idx, module_name) in app.statusbar_modules.iter().enumerate() {
        let spans = match module_name.as_str() {
            "mode" => {
                let mode_str = match app.mode {
                    Mode::Normal => "NORMAL",
                    Mode::AddUrl | Mode::AddConfirm => "INPUT",
                    Mode::Command => "COMMAND",
                    Mode::ConfirmDelete => "CONFIRM",
                };
                let pulse_char = if !app.enable_animations {
                    if nf { "⬢" } else { "o" }
                } else if nf {
                    let pulse_idx = ((millisecond / 250) % 4) as usize;
                    ["⬡", "⬢", "⬡", "⬢"][pulse_idx]
                } else {
                    let pulse_idx = ((millisecond / 250) % 4) as usize;
                    ["-", "o", "-", "o"][pulse_idx]
                };
                vec![
                    Span::styled(
                        format!(" {} ", pulse_char),
                        Style::new().fg(t.background).bold(),
                    ),
                    Span::styled(
                        format!("MODE: {} ", mode_str.to_uppercase()),
                        Style::new().fg(t.background).bold(),
                    ),
                ]
            }
            "tab" => vec![Span::styled(
                app.active_tab.label(false).trim().to_uppercase(),
                Style::new().fg(t.background).bold(),
            )],
            "time" => vec![Span::styled(
                Local::now().format("%H:%M").to_string(),
                Style::new().fg(t.background),
            )],
            "radar" => {
                let scan_chars = if nf {
                    ["░", "▒", "▓", "█", "▓", "▒", "░", " "]
                } else {
                    ["=", "o", "x", "X", "x", "o", "=", " "]
                };
                let scan_idx = if !app.enable_animations {
                    0
                } else {
                    ((millisecond / 120) % 8) as usize
                };
                let mut scan_bar = String::new();
                for i in 0..6 {
                    let char_idx = (scan_idx + i) % 8;
                    scan_bar.push(scan_chars[char_idx].chars().next().unwrap_or(' '));
                }
                vec![Span::styled(
                    format!("[SYS: {}] ", scan_bar),
                    Style::new().fg(t.background).bold(),
                )]
            }
            "cpu" => {
                let cpu_icon = if nf { "󰻠 " } else { "CPU " };
                vec![Span::styled(
                    format!("{}{:.0}%", cpu_icon, app.cpu_usage),
                    Style::new().fg(t.background),
                )]
            }
            "ram" => {
                let mem_icon = if nf { "󰍛 " } else { "RAM " };
                vec![Span::styled(
                    format!("{}{}", mem_icon, format_bytes(app.mem_usage)),
                    Style::new().fg(t.background),
                )]
            }
            "speed" => {
                let mut s_spans = Vec::new();
                let dl_indicator = if nf {
                    if is_downloading && app.enable_animations {
                        let idx = ((millisecond / 150) % 4) as usize;
                        [" 󰇚 ", " 󰇛 ", " 󰇜 ", " 󰇚 "][idx]
                    } else {
                        " 󰇚 "
                    }
                } else {
                    if is_downloading && app.enable_animations {
                        let idx = ((millisecond / 200) % 2) as usize;
                        if idx == 0 {
                            " v "
                        } else {
                            " . "
                        }
                    } else {
                        " v "
                    }
                };
                if is_downloading {
                    s_spans.push(Span::styled(
                        dl_indicator,
                        Style::new().fg(t.background).bold(),
                    ));
                    s_spans.push(Span::styled(
                        format!("{}/s", format_bytes(app.total_speed as u64)),
                        Style::new().fg(t.background).bold(),
                    ));
                } else {
                    let net_icon = if nf { "󰇚 " } else { "NET " };
                    s_spans.push(Span::styled(
                        format!("{}OFFLINE", net_icon),
                        Style::new().fg(t.background),
                    ));
                }
                s_spans
            }
            "queue" => vec![Span::styled(
                format!("Q: {}/{}", app.active_count, app.items.len()),
                Style::new().fg(t.background),
            )],
            _ => vec![],
        };

        if spans.is_empty() {
            continue;
        }

        if idx < left_limit {
            if left_spans.len() > 1 {
                left_spans.push(Span::styled(" │ ", Style::new().fg(t.surface_dim)));
            }
            left_spans.extend(spans);
        } else {
            if !right_spans.is_empty() {
                right_spans.push(Span::styled(" │ ", Style::new().fg(t.surface_dim)));
            }
            right_spans.extend(spans);
        }
    }

    // End Right with a closing block
    right_spans.push(Span::styled(" █", Style::new().fg(t.background)));

    // ── CENTER: Commands & Notifications ──────────────────────────────────
    let left_w: u16 = left_spans.iter().map(|s| s.width() as u16).sum();
    let right_w: u16 = right_spans.iter().map(|s| s.width() as u16).sum();

    let center_area_w = area.width.saturating_sub(left_w + right_w);

    let center_span = if app.mode == Mode::Command {
        Span::styled(
            format!(":{}█", app.command_buffer),
            Style::new().fg(t.accent).bold().bg(t.background),
        )
    } else if let Some(msg) = &app.status_message {
        let col = if app.status_is_error {
            t.error
        } else {
            t.success
        };
        Span::styled(
            format!(" {} ", msg.to_uppercase()),
            Style::new().fg(t.background).bg(col).bold(),
        )
    } else {
        Span::raw("")
    };

    let center_w = center_span.width() as u16;
    let left_gap = (center_area_w.saturating_sub(center_w)) / 2;
    let right_gap = center_area_w.saturating_sub(center_w + left_gap);

    let mut all_spans = left_spans;
    all_spans.push(Span::styled(
        " ".repeat(left_gap as usize),
        Style::new().bg(t.accent),
    ));
    all_spans.push(center_span);
    all_spans.push(Span::styled(
        " ".repeat(right_gap as usize),
        Style::new().bg(t.accent),
    ));
    all_spans.append(&mut right_spans);

    let p = Paragraph::new(Line::from(all_spans))
        .style(Style::new().bg(t.accent).fg(t.background).bold());

    f.render_widget(p, area);
}
