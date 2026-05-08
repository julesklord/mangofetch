use super::app::{App, AppState, Mode, Tab};
use super::assets::{BLOCK_TITLE, MANGO_BODY, MANGO_STEM};
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
        Constraint::Length(3),  // header / tabs
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

    let rows_data = [
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
            Row::new([key_cell, val_cell, hint_cell])
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
            .border_style(Style::new().fg(t.surface)),
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
