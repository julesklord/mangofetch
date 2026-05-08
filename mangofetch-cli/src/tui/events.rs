use super::app::{App, AppState, Mode, Tab};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> std::io::Result<()> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(());
    }
    let Event::Key(key) = event::read()? else {
        return Ok(());
    };
    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    // Splash: any key starts
    if matches!(app.state, AppState::Splash) {
        if key.code == KeyCode::Char('q') {
            app.quit();
        } else {
            app.start();
        }
        return Ok(());
    }

    match app.mode {
        Mode::Command => handle_command_mode(app, key.code),
        Mode::AddUrl => handle_add_url_mode(app, key).await,
        Mode::ConfirmDelete => handle_confirm_delete(app, key.code).await,
        Mode::Normal => handle_normal_mode(app, key.code, key.modifiers).await,
    }
    Ok(())
}

fn handle_command_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.command_buffer.clear();
        }
        KeyCode::Enter => {
            let cmd = app.command_buffer.clone();
            app.command_buffer.clear();
            app.mode = Mode::Normal;
            execute_colon_command(app, &cmd);
        }
        KeyCode::Char(c) => app.command_buffer.push(c),
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }
        _ => {}
    }
}

fn execute_colon_command(app: &mut App, raw: &str) {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let Some(&cmd) = parts.first() else { return };
    match cmd {
        "q" | "quit" => app.quit(),
        "help" | "h" => app.toggle_help(),
        "d" | "download" => {
            if parts.len() > 1 {
                app.url_input = parts[1].to_string();
                app.quality_input = parts.get(2).unwrap_or(&"").to_string();
                let url = app.url_input.clone();
                let quality = if app.quality_input.is_empty() {
                    None
                } else {
                    Some(app.quality_input.clone())
                };
                let queue = app.queue.clone();
                let registry = app.registry.clone();
                app.set_status(format!("Queueing: {}", &url));
                tokio::spawn(async move {
                    let _ = crate::engine::enqueue_download_with_quality(
                        &url, None, quality, registry, queue,
                    )
                    .await;
                });
            }
        }
        "clear" => app.log_lines.clear(),
        _ => app.set_error(format!("Unknown command: {}", cmd)),
    }
}

async fn handle_add_url_mode(app: &mut App, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.close_add_modal(),
        KeyCode::Tab => app.add_modal_next_field(),
        KeyCode::Enter => {
            if !app.url_input.is_empty() {
                let url = app.url_input.clone();
                let quality = if app.quality_input.is_empty() {
                    None
                } else {
                    Some(app.quality_input.clone())
                };
                let queue = app.queue.clone();
                let registry = app.registry.clone();
                app.set_status(format!("Queueing: {}", &url));
                app.push_log(format!(
                    "[{}] Added: {}",
                    chrono::Local::now().format("%H:%M:%S"),
                    &url
                ));
                app.close_add_modal();
                tokio::spawn(async move {
                    let _ = crate::engine::enqueue_download_with_quality(
                        &url, None, quality, registry, queue,
                    )
                    .await;
                });
            }
        }
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'v' {
                // Paste from clipboard if available
                if let Ok(clip) = cli_clipboard::get_contents() {
                    let clip = clip.trim().to_string();
                    if app.add_modal_field == 0 {
                        app.url_input = clip;
                    } else {
                        app.quality_input = clip;
                    }
                }
            } else if app.add_modal_field == 0 {
                app.url_input.push(c);
            } else {
                app.quality_input.push(c);
            }
        }
        KeyCode::Backspace => {
            if app.add_modal_field == 0 {
                app.url_input.pop();
            } else {
                app.quality_input.pop();
            }
        }
        _ => {}
    }
}

async fn handle_confirm_delete(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.remove_selected().await;
        }
        _ => {
            app.mode = Mode::Normal;
            app.set_status("Cancelled".to_string());
        }
    }
}

async fn handle_normal_mode(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    // Ctrl+C always quits
    if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
        app.quit();
        return;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char(':') | KeyCode::Char('/') => {
            app.mode = Mode::Command;
            app.command_buffer.clear();
        }
        KeyCode::Char('a') | KeyCode::Char('n') => app.open_add_modal(),
        KeyCode::Char('?') => app.toggle_help(),

        // Tab navigation
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.prev_tab(),
        KeyCode::Char('1') => {
            app.active_tab = Tab::Queue;
            app.refresh_data();
        }
        KeyCode::Char('2') => {
            app.active_tab = Tab::History;
            app.refresh_data();
        }
        KeyCode::Char('3') => {
            app.active_tab = Tab::Logs;
        }
        KeyCode::Char('4') => {
            app.active_tab = Tab::Settings;
        }

        // Navigation
        KeyCode::Up | KeyCode::Char('k') => match app.active_tab {
            Tab::Settings => app.prev_setting(),
            Tab::Logs => app.scroll_logs_up(),
            _ => app.prev_item(),
        },
        KeyCode::Down | KeyCode::Char('j') => match app.active_tab {
            Tab::Settings => app.next_setting(),
            Tab::Logs => app.scroll_logs_down(),
            _ => app.next_item(),
        },
        KeyCode::Char('G') => {
            if app.active_tab == Tab::Logs {
                app.scroll_logs_bottom();
            } else if !app.items.is_empty() {
                app.table_state.select(Some(app.items.len() - 1));
            }
        }
        KeyCode::Char('g') => {
            if app.active_tab != Tab::Logs {
                app.table_state.select(Some(0));
            } else {
                app.log_scroll = 0;
            }
        }

        // Actions
        KeyCode::Enter => {
            if app.active_tab == Tab::Settings {
                app.toggle_setting();
            }
        }
        KeyCode::Char('p') => app.pause_selected().await,
        KeyCode::Char('r') => app.resume_selected().await,
        KeyCode::Char('x') | KeyCode::Delete => {
            if app.table_state.selected().is_some() {
                app.mode = Mode::ConfirmDelete;
            }
        }

        _ => {}
    }
}
