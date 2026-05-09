use super::app::{App, AppState, Mode, Tab};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> std::io::Result<()> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(());
    }

    match event::read()? {
        Event::Key(key) => {
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
                Mode::AddConfirm => handle_add_confirm_mode(app, key.code).await,
                Mode::ConfirmDelete => handle_confirm_delete(app, key.code).await,
                Mode::Normal => handle_normal_mode(app, key.code, key.modifiers).await,
            }
        }
        Event::Mouse(mouse) => {
            if matches!(app.state, AppState::Splash) {
                app.start();
                return Ok(());
            }
            handle_mouse_event(app, mouse).await;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_mouse_event(app: &mut App, mouse: crossterm::event::MouseEvent) {
    use crossterm::event::MouseEventKind;
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            if app.active_tab == Tab::Logs {
                app.log_scroll_down();
            } else {
                app.next_item();
            }
        }
        MouseEventKind::ScrollUp => {
            if app.active_tab == Tab::Logs {
                app.log_scroll_up();
            } else {
                app.prev_item();
            }
        }
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            // Simple logic: if click is in top row, maybe switch tabs?
            // (Hard to do perfectly without area context, but we can try)
            if mouse.row < 3 {
                let col = mouse.column;
                // Rough estimate of tab positions
                if col < 10 {
                    app.active_tab = Tab::Home;
                } else if col < 20 {
                    app.active_tab = Tab::Queue;
                } else if col < 30 {
                    app.active_tab = Tab::History;
                } else if col < 40 {
                    app.active_tab = Tab::Settings;
                } else if col < 50 {
                    app.active_tab = Tab::About;
                } else if col < 60 {
                    app.active_tab = Tab::Logs;
                }
            }
        }
        _ => {}
    }
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
        KeyCode::Enter if !app.url_input.is_empty() => {
            let url = app.url_input.clone();
            app.set_status(format!("Fetching info: {}", &url));
            app.mode = Mode::AddConfirm;
            app.is_fetching_preview = true;
            app.preview_info = None;
            app.preview_error = None;

            let tx = app.msg_tx.clone();
            let registry = app.registry.clone();
            tokio::spawn(async move {
                let downloader = registry.find_platform(&url);
                if let Some(dl) = downloader {
                    let info = mangofetch_core::core::manager::queue::fetch_and_cache_info(
                        &url,
                        &*dl,
                        dl.name(),
                    )
                    .await;
                    match info {
                        Ok(i) => {
                            let item_info = mangofetch_core::models::queue::QueueItemInfo {
                                id: 0,
                                url: url.clone(),
                                platform: dl.name().to_string(),
                                title: i.title,
                                status: mangofetch_core::models::queue::QueueStatus::Queued,
                                percent: 0.0,
                                speed_bytes_per_sec: 0.0,
                                downloaded_bytes: 0,
                                total_bytes: None,
                                phase: "Pending".to_string(),
                                file_path: None,
                                file_size_bytes: None,
                                file_count: None,
                                thumbnail_url: i.thumbnail_url,
                            };
                            let _ = tx.send(super::app::AppMsg::PreviewFetched(Ok(item_info)));
                        }
                        Err(e) => {
                            let _ = tx.send(super::app::AppMsg::PreviewFetched(Err(e.to_string())));
                        }
                    }
                } else {
                    let _ = tx.send(super::app::AppMsg::PreviewFetched(Err(
                        "Platform not supported".to_string(),
                    )));
                }
            });
        }
        KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Ok(clip) = cli_clipboard::get_contents() {
                let clip = clip.trim().to_string();
                if app.add_modal_field == 0 {
                    app.url_input = clip;
                } else {
                    app.quality_input = clip;
                }
            }
        }
        KeyCode::Char(c) => {
            if app.add_modal_field == 0 {
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

async fn handle_add_confirm_mode(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.preview_info = None;
            app.preview_error = None;
        }
        KeyCode::Enter if app.preview_info.is_some() => {
            if let Some(info) = app.preview_info.take() {
                let url = info.url.clone();
                let quality = if app.quality_input.is_empty() {
                    None
                } else {
                    Some(app.quality_input.clone())
                };
                let queue = app.queue.clone();
                let registry = app.registry.clone();

                app.push_log(format!(
                    "[{}] Added: {}",
                    chrono::Local::now().format("%H:%M:%S"),
                    info.title
                ));
                app.close_add_modal();
                app.mode = Mode::Normal;

                tokio::spawn(async move {
                    let _ = crate::engine::enqueue_download_with_quality(
                        &url, None, quality, registry, queue,
                    )
                    .await;
                });
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
        KeyCode::Tab => {
            if app.active_tab == Tab::Queue || app.active_tab == Tab::History {
                app.next_category();
            } else {
                app.next_tab();
            }
        }
        KeyCode::BackTab => app.prev_tab(),
        KeyCode::Char('1') => {
            app.active_tab = Tab::Home;
        }
        KeyCode::Char('2') => {
            app.active_tab = Tab::Queue;
            app.refresh_data();
        }
        KeyCode::Char('3') => {
            app.active_tab = Tab::History;
            app.refresh_data();
        }
        KeyCode::Char('4') => {
            app.active_tab = Tab::Settings;
        }
        KeyCode::Char('5') => {
            app.active_tab = Tab::About;
        }
        KeyCode::Char('6') => {
            app.active_tab = Tab::Logs;
        }

        // Navigation
        KeyCode::Up | KeyCode::Char('k') => match app.active_tab {
            Tab::Settings => app.prev_setting(),
            Tab::Logs => app.scroll_logs_up(),
            Tab::Home => {
                app.home_index = app.home_index.saturating_sub(1);
            }
            Tab::About => {
                app.about_index = app.about_index.saturating_sub(1);
            }
            _ => app.prev_item(),
        },
        KeyCode::Down | KeyCode::Char('j') => match app.active_tab {
            Tab::Settings => app.next_setting(),
            Tab::Logs => app.scroll_logs_down(),
            Tab::Home => {
                if app.home_index < 3 {
                    app.home_index += 1;
                }
            }
            Tab::About => {
                if app.about_index < 4 {
                    app.about_index += 1;
                }
            }
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
        KeyCode::Enter => match app.active_tab {
            Tab::Settings => app.toggle_setting(),
            Tab::Home => app.execute_home_action().await,
            _ => {}
        },
        KeyCode::Char('p') => app.pause_selected().await,
        KeyCode::Char('r') => app.resume_selected().await,
        KeyCode::Char('x') | KeyCode::Delete if app.table_state.selected().is_some() => {
            app.mode = Mode::ConfirmDelete;
        }

        _ => {}
    }
}
