use super::app::{App, AppState, Mode, Tab};
use super::assets::MANGO_LOGO;
use crate::formatting::{format_bytes, format_duration};
use mangofetch_core::models::queue::QueueStatus;
use ratatui::{
    prelude::*,
    widgets::{
        Block, Borders, Cell, Clear, Gauge, List, ListItem, Paragraph, Row, Table, Tabs, Wrap,
    },
};

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
        Constraint::Min(0),    // main content
        Constraint::Length(5), // detail panel
        Constraint::Length(1), // status bar
    ])
    .split(area);

    render_tabs(f, app, chunks[0]);
    render_main(f, app, chunks[1]);
    render_detail(f, app, chunks[2]);
    render_statusbar(f, app, chunks[3]);

    // Overlays
    if app.show_help {
        render_help(f, app);
    }
    if app.mode == Mode::AddUrl {
        render_add_modal(f, app);
    }
    if app.mode == Mode::ConfirmDelete {
        render_confirm_delete(f, app);
    }
}

// ── Splash ────────────────────────────────────────────────────────────────────

fn render_splash(f: &mut Frame, app: &App) {
    let area = f.area();
    let t = &app.theme;

    let logo_h = MANGO_LOGO.len() as u16;
    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(logo_h),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(1),
    ])
    .split(area);

    let logo = Paragraph::new(MANGO_LOGO.join("\n"))
        .alignment(Alignment::Center)
        .style(Style::new().fg(t.accent).bold());
    f.render_widget(logo, chunks[1]);

    let sub = Paragraph::new(format!(
        "  v{}  ·  1000+ platforms  ·  yt-dlp powered",
        app.version
    ))
    .alignment(Alignment::Center)
    .style(Style::new().fg(t.text_dim));
    f.render_widget(sub, chunks[2]);

    let hint = Paragraph::new("Press any key to start  ·  q to quit")
        .alignment(Alignment::Center)
        .style(Style::new().fg(t.secondary).bold());
    f.render_widget(hint, chunks[4]);
}

// ── Tabs / header ─────────────────────────────────────────────────────────────

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;

    let titles: Vec<String> = Tab::ALL
        .iter()
        .map(|tab| tab.label(nf).to_string())
        .collect();
    let selected = app.active_tab.index();

    // Right-side stats
    let speed_str = if app.total_speed > 0.0 {
        format!("  {}/s", format_bytes(app.total_speed as u64))
    } else {
        String::new()
    };
    let header_title = format!(
        " MangoFetch  ▸  {} active  {}{}  {} ",
        app.active_count,
        if app.queued_count > 0 {
            format!("· {} queued", app.queued_count)
        } else {
            String::new()
        },
        speed_str,
        app.current_time,
    );

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(header_title)
                .title_style(Style::new().fg(t.text_dim))
                .border_style(Style::new().fg(t.accent)),
        )
        .select(selected)
        .style(Style::new().fg(t.text_dim))
        .highlight_style(Style::new().fg(t.accent).bold())
        .divider(Span::styled(" │ ", Style::new().fg(t.surface)));

    f.render_widget(tabs, area);
}

// ── Main content router ───────────────────────────────────────────────────────

fn render_main(f: &mut Frame, app: &mut App, area: Rect) {
    match app.active_tab {
        Tab::Queue | Tab::History => render_queue_table(f, app, area),
        Tab::Logs => render_logs(f, app, area),
        Tab::Settings => render_settings(f, app, area),
    }
}

// ── Queue / History table ─────────────────────────────────────────────────────

fn render_queue_table(f: &mut Frame, app: &mut App, area: Rect) {
    let t = &app.theme;
    let nf = app.use_nerd_fonts;
    let is_queue = app.active_tab == Tab::Queue;

    // Split area: table top, progress bars bottom
    let chunks = Layout::vertical([Constraint::Min(0)]).split(area);
    let table_area = chunks[0];

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

            Row::new(vec![
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
                    .border_style(Style::new().fg(t.surface)),
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
            .border_style(Style::new().fg(t.surface)),
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
            .border_style(Style::new().fg(t.surface)),
    );
    f.render_widget(list, area);
}

// ── Settings panel ────────────────────────────────────────────────────────────

fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let settings = mangofetch_core::models::settings::AppSettings::load_from_disk();

    let rows_data = vec![
        (
            "TUI Theme",
            settings.appearance.tui_theme.clone(),
            "mango │ pitaya │ coconut │ dracula  (Enter to cycle)",
        ),
        (
            "Nerd Fonts",
            if app.use_nerd_fonts {
                "ON".into()
            } else {
                "OFF".into()
            },
            "Requires a Nerd Font patched terminal font",
        ),
        (
            "Max Downloads",
            settings.advanced.max_concurrent_downloads.to_string(),
            "Max simultaneous downloads  (1 │ 2 │ 3 │ 5)",
        ),
        (
            "Default Quality",
            settings.download.video_quality.clone(),
            "best │ 1080p │ 720p │ 480p │ 360p",
        ),
    ];

    let rows: Vec<Row> = rows_data
        .iter()
        .enumerate()
        .map(|(i, (key, val, hint))| {
            let selected = i == app.settings_index;
            let key_cell = Cell::from(*key).style(if selected {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.secondary)
            });
            let val_cell = Cell::from(val.as_str()).style(if selected {
                Style::new().fg(t.text).bold().bg(t.highlight)
            } else {
                Style::new().fg(t.text)
            });
            let hint_cell = Cell::from(*hint).style(Style::new().fg(t.text_dim));
            Row::new(vec![key_cell, val_cell, hint_cell])
                .height(1)
                .bottom_margin(0)
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
            Constraint::Length(18),
            Constraint::Length(14),
            Constraint::Min(30),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" ⚙ Settings  (↑↓ navigate · Enter cycle) ")
            .border_style(Style::new().fg(t.surface)),
    )
    .row_highlight_style(Style::new().bg(t.highlight));

    f.render_widget(table, area);
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
            Tab::Settings => " Use ↑↓ to navigate settings, Enter to cycle values.".into(),
            Tab::Logs => " ↑↓ or j/k to scroll  ·  G jumps to bottom  ·  :clear to clear".into(),
            _ => " Select an item above to see details.".into(),
        }
    };

    let detail = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Details ")
                .border_style(Style::new().fg(t.surface)),
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
                .border_style(Style::new().fg(t.surface)),
        )
        .gauge_style(Style::new().fg(t.progress).bg(t.surface_dim))
        .percent(pct)
        .label(Span::styled(
            format!("{pct}%"),
            Style::new().fg(t.text).bold(),
        ));
    f.render_widget(gauge, chunks[1]);
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
            Tab::Queue | Tab::History => {
                " a:add  p:pause  r:resume  x:delete  /:cmd  Tab:tab  ?:help  q:quit"
            }
            Tab::Settings => " ↑↓:navigate  Enter:cycle  Tab:tab  q:quit",
            Tab::Logs => " ↑↓:scroll  G:bottom  :clear  q:quit",
        };
        Span::styled(shortcuts, Style::new().fg(t.text_dim).bg(t.surface_dim))
    };

    let right_str = format!(
        " {} active  v{}  {} ",
        app.active_count, app.version, app.current_time
    );
    let right = Span::styled(&right_str, Style::new().fg(t.text_dim).bg(t.surface_dim));

    let left_w = left.width() as u16;
    let right_w = right.width() as u16;
    let gap = area.width.saturating_sub(left_w + right_w);

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

    let items: Vec<ListItem> = vec![
        ListItem::new("  General").style(Style::new().fg(t.secondary).bold()),
        ListItem::new("  q          Quit").style(Style::new().fg(t.text)),
        ListItem::new("  ?          Toggle this help").style(Style::new().fg(t.text)),
        ListItem::new("  : or /     Command mode  (:download <url>  :quit  :clear)").style(Style::new().fg(t.text)),
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

    let styled: Vec<ListItem> = items;

    let list = List::new(styled).block(
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
        Style::new().fg(t.surface)
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
        Style::new().fg(t.surface)
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

// ── Helpers ───────────────────────────────────────────────────────────────────

fn status_display(
    item: &mangofetch_core::models::queue::QueueItemInfo,
    nf: bool,
    t: &super::themes::Theme,
) -> (Color, String) {
    fn ic(nf: bool, nf_icon: &'static str, fb: &'static str) -> &'static str {
        if nf { nf_icon } else { fb }
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
                (t.success, format!("{} Done",   ic(nf, "󰄬", "✓")))
            } else {
                (t.error,   format!("{} Failed", ic(nf, "󰅖", "✗")))
            }
        }
        QueueStatus::Error { message } => (
            t.error,
            format!("{} Err: {}", ic(nf, "󰅚", "!"), truncate(message, 20)),
        ),
    }
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
