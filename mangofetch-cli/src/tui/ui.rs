use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs, List, ListItem, Clear},
};
use super::app::{App, Tab, AppState, Mode};
use super::assets::STYLIZED_MANGO;
use crate::formatting::format_bytes;
use mangofetch_core::models::queue::QueueStatus;

pub fn render(f: &mut Frame, app: &mut App) {
    if matches!(app.state, AppState::Splash) {
        render_splash(f, app);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Main content
            Constraint::Length(6), // Details / Info Panel
            Constraint::Length(1), // Footer (shortcuts)
            Constraint::Length(1), // Statusline
        ])
        .split(f.area());

    render_tabs(f, app, chunks[0]);
    render_main_content(f, app, chunks[1]);
    render_details(f, app, chunks[2]);
    render_footer(f, app, chunks[3]);
    render_statusline(f, app, chunks[4]);

    if app.show_help {
        render_help_modal(f, app);
    }
}

fn render_help_modal(f: &mut Frame, app: &App) {
    let area = f.area();
    let theme = &app.theme;

    let help_text = vec![
        " General Shortcuts: ",
        "  q        : Quit ",
        "  Tab      : Next Tab ",
        "  S-Tab    : Previous Tab ",
        "  1, 2, 3  : Switch to Tab ",
        "  ?        : Toggle Help ",
        "  /        : Command Mode ",
        "",
        " Navigation: ",
        "  Up / k   : Move Up ",
        "  Down / j : Move Down ",
        "  Enter    : Toggle Setting ",
        "",
        " Download Actions: ",
        "  p        : Pause Selected ",
        "  r        : Resume Selected ",
        "  x        : Remove Selected ",
        "",
        " Commands (:mode): ",
        "  :download <url> ",
        "  :quit ",
        "  :help ",
    ];

    let items: Vec<ListItem> = help_text
        .iter()
        .map(|&s| {
            if s.ends_with(':') {
                ListItem::new(s).style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(s).style(Style::default().fg(theme.text))
            }
        })
        .collect();

    let help_list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Help / Shortcuts ")
            .border_style(Style::default().fg(theme.accent)));

    let area = centered_rect(60, 80, area);
    f.render_widget(Clear, area); // Clear the background
    f.render_widget(help_list, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_icon(nerd_font: bool, nf_icon: &str, fallback: &str) -> String {
    if nerd_font {
        nf_icon.to_string()
    } else {
        fallback.to_string()
    }
}

fn render_splash(f: &mut Frame, app: &App) {
    let area = f.area();
    let theme = &app.theme;

    f.render_widget(Block::default().bg(theme.background), area);

    let mango_height = STYLIZED_MANGO.len() as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(mango_height + 4)) / 2),
            Constraint::Length(mango_height),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    let mango_text = STYLIZED_MANGO.join("\n");
    let mango_para = Paragraph::new(mango_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD));
    f.render_widget(mango_para, chunks[1]);

    let title = Paragraph::new("MANGO FETCH")
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[2]);

    let subtitle = Paragraph::new("Press any key to start")
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme.text).add_modifier(Modifier::DIM));
    f.render_widget(subtitle, chunks[3]);
}

fn render_tabs(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let nf = app.use_nerd_fonts;

    let titles = vec![
        format!(" {} Queue ", get_icon(nf, "󰄖", "📥")),
        format!(" {} History ", get_icon(nf, "󰄗", "📜")),
        format!(" {} Settings ", get_icon(nf, "󰒓", "⚙")),
    ];

    let selected_index = match app.active_tab {
        Tab::Queue => 0,
        Tab::History => 1,
        Tab::Settings => 2,
    };

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" MangoFetch ").border_style(Style::default().fg(theme.accent)))
        .select(selected_index)
        .style(Style::default().fg(theme.text))
        .highlight_style(Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD))
        .divider(Span::raw("|").style(Style::default().fg(theme.surface)));

    f.render_widget(tabs, area);
}

fn render_main_content(f: &mut Frame, app: &mut App, area: Rect) {
    match app.active_tab {
        Tab::Queue | Tab::History => render_table(f, app, area),
        Tab::Settings => render_settings(f, app, area),
    }
}

fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let nf = app.use_nerd_fonts;

    let settings_list = vec![
        format!(" {} TUI Theme: [ {} ] ", get_icon(nf, "󰏘", "🎨"), get_current_theme_name(app)),
        format!(" {} Nerd Fonts: [ {} ] ", get_icon(nf, "󰙅", "🔤"), if app.use_nerd_fonts { "ON" } else { "OFF" }),
    ];

    let items: Vec<ListItem> = settings_list
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let style = if i == app.settings_index {
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD).bg(theme.surface)
            } else {
                Style::default().fg(theme.text)
            };
            ListItem::new(s.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Settings ").border_style(Style::default().fg(theme.surface)));

    f.render_widget(list, area);
}

fn get_current_theme_name(_app: &App) -> &'static str {
    let settings = mangofetch_core::models::settings::AppSettings::load_from_disk();
    match settings.appearance.tui_theme.as_str() {
        "mango" => "Mango",
        "pitaya" => "Pitaya",
        "coconut" => "Coconut",
        _ => "Custom",
    }
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let theme = &app.theme;
    let nf = app.use_nerd_fonts;

    let headers = vec![
        "ID".to_string(),
        format!("{} Platform", get_icon(nf, "󰋉", "")),
        format!("{} Title", get_icon(nf, "󰈙", "")),
        "Status".to_string(),
        "Progress".to_string(),
        format!("{} Speed", get_icon(nf, "󰓅", "")),
    ];

    let header_cells = headers
        .iter()
        .map(|h| Cell::from(h.as_str()).style(Style::default().fg(theme.secondary)));

    let header = Row::new(header_cells)
        .style(Style::default().bg(theme.surface))
        .height(1)
        .bottom_margin(1);

    let rows = app.items.iter().map(|item| {
        let status_color = match &item.status {
            QueueStatus::Queued => theme.text,
            QueueStatus::Active => theme.accent,
            QueueStatus::Paused => theme.warning,
            QueueStatus::Seeding => theme.secondary,
            QueueStatus::Complete { success } => if *success { theme.success } else { theme.error },
            QueueStatus::Error { .. } => theme.error,
        };

        let status_str = match &item.status {
            QueueStatus::Queued => format!("{} Queued", get_icon(nf, "󰄗", "•")),
            QueueStatus::Active => format!("{} {}", get_icon(nf, "󰄖", "▶"), item.phase),
            QueueStatus::Paused => format!("{} Paused", get_icon(nf, "󰏤", "II")),
            QueueStatus::Seeding => format!("{} Seeding", get_icon(nf, "󰕒", "S")),
            QueueStatus::Complete { success } => if *success {
                format!("{} Done", get_icon(nf, "󰄬", "✓"))
            } else {
                format!("{} Failed", get_icon(nf, "󰅖", "✗"))
            },
            QueueStatus::Error { message } => format!("{} Error: {}", get_icon(nf, "󰅚", "E"), message),
        };

        let progress = format!("{:.1}%", item.percent);
        let speed = if item.speed_bytes_per_sec > 0.0 {
            format!("{}/s", format_bytes(item.speed_bytes_per_sec as u64))
        } else {
            "-".to_string()
        };

        let cells = vec![
            Cell::from(item.id.to_string()),
            Cell::from(item.platform.clone()),
            Cell::from(item.title.clone()),
            Cell::from(status_str).style(Style::default().fg(status_color)),
            Cell::from(progress),
            Cell::from(speed),
        ];
        Row::new(cells).height(1).style(Style::default().fg(theme.text))
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Length(15),
            Constraint::Min(30),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .title(if app.active_tab == Tab::Queue { " Active Downloads " } else { " History " })
        .border_style(Style::default().fg(theme.surface)))
    .row_highlight_style(Style::default().bg(theme.surface).add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, area, &mut app.table_state);
}

fn render_details(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let selected_idx = app.table_state.selected();
    let detail_text = if let Some(idx) = selected_idx {
        if let Some(item) = app.items.get(idx) {
            let downloaded = format_bytes(item.downloaded_bytes);
            let total = item.total_bytes.map(format_bytes).unwrap_or("?".to_string());
            format!(
                " Title: {}\n URL: {}\n Path: {}\n Size: {} / {}\n Platform: {}",
                item.title,
                item.url,
                item.file_path.as_deref().unwrap_or("N/A"),
                downloaded,
                total,
                item.platform
            )
        } else {
            "No item selected".to_string()
        }
    } else {
        "No item selected".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Selected Item Details ")
        .border_style(Style::default().fg(theme.surface));
    let details = Paragraph::new(detail_text).block(block).style(Style::default().fg(theme.text));
    f.render_widget(details, area);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let footer_text = if app.mode == Mode::Command {
        " <Esc> Cancel | <Enter> Execute "
    } else if app.active_tab == Tab::Settings {
        " <q> Quit | <Tab> Switch Tab | <↑/↓> Navigate | <Enter> Toggle/Select "
    } else {
        " <q> Quit | <Tab> Switch Tab | <↑/↓> Navigate | </> Command Mode "
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(theme.surface));
    f.render_widget(footer, area);
}

fn render_statusline(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let nf = app.use_nerd_fonts;

    if app.mode == Mode::Command {
        let prompt = format!(":{}", app.command_buffer);
        let statusline = Paragraph::new(prompt)
            .style(Style::default().fg(theme.text).bg(theme.surface));
        f.render_widget(statusline, area);
        return;
    }

    let left_content = if let Some(msg) = &app.status_message {
        Span::styled(
            format!(" {} {} ", get_icon(nf, "󰋼", "ℹ"), msg),
            Style::default().fg(theme.background).bg(theme.accent).add_modifier(Modifier::BOLD)
        )
    } else {
        let active_count = app.items.iter().filter(|i| matches!(i.status, QueueStatus::Active)).count();
        let total_count = app.items.len();

        Span::styled(
            format!(
                " {} Active: {} | {} Total: {} ",
                get_icon(nf, "󰄖", "▶"), active_count,
                get_icon(nf, "󰄗", "📥"), total_count
            ),
            Style::default().fg(theme.accent).bg(theme.surface)
        )
    };

    let right_text = format!(
        " {} v{} | {} {} ",
        get_icon(nf, "󰒓", "⚙"), app.version,
        get_icon(nf, "󱑊", "🕒"), app.current_time
    );
    let right = Span::styled(right_text, Style::default().fg(theme.text).bg(theme.surface));

    let left_width = left_content.width() as u16;
    let right_width = right.width() as u16;
    let spacer_width = area.width.saturating_sub(left_width + right_width);

    let statusline = Paragraph::new(Line::from(vec![
        left_content,
        Span::raw(" ".repeat(spacer_width as usize)),
        right,
    ]))
    .style(Style::default().bg(theme.surface));

    f.render_widget(statusline, area);
}
