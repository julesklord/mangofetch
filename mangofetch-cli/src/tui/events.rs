use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;
use super::app::{App, Tab, AppState, Mode};

pub async fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // If in splash, any key starts the app
                if matches!(app.state, AppState::Splash) {
                    if key.code != KeyCode::Char('q') {
                        app.start();
                    } else {
                        app.quit();
                    }
                    return Ok(());
                }

                if app.mode == Mode::Command {
                    match key.code {
                        KeyCode::Esc => {
                            app.mode = Mode::Normal;
                            app.command_buffer.clear();
                        }
                        KeyCode::Enter => {
                            let cmd = app.command_buffer.clone();
                            execute_command(app, &cmd).await;
                            app.command_buffer.clear();
                            app.mode = Mode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.command_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app.command_buffer.pop();
                        }
                        _ => {}
                    }
                    return Ok(());
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
                    KeyCode::Char('/') => {
                        app.mode = Mode::Command;
                        app.command_buffer.clear();
                    }

                    // Tab Navigation
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::BackTab => app.prev_tab(),

                    // Item / Setting Navigation
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.active_tab == Tab::Settings {
                            app.prev_setting();
                        } else {
                            app.prev_item();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.active_tab == Tab::Settings {
                            app.next_setting();
                        } else {
                            app.next_item();
                        }
                    }

                    KeyCode::Enter => {
                        if app.active_tab == Tab::Settings {
                            app.toggle_setting();
                        }
                    }

                    // Download Actions
                    KeyCode::Char('p') => {
                        app.pause_selected().await;
                        app.set_status("Paused download".to_string());
                    }
                    KeyCode::Char('r') => {
                        app.resume_selected().await;
                        app.set_status("Resumed download".to_string());
                    }
                    KeyCode::Char('x') => {
                        app.remove_selected().await;
                        app.set_status("Removed from queue".to_string());
                    }

                    // Direct Tab Access
                    KeyCode::Char('1') => app.active_tab = Tab::Queue,
                    KeyCode::Char('2') => app.active_tab = Tab::History,
                    KeyCode::Char('3') => app.active_tab = Tab::Settings,

                    KeyCode::Char('?') => app.toggle_help(),

                    _ => {}
                }
            }
        }
    }
    Ok(())
}

async fn execute_command(app: &mut App, cmd: &str) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() { return; }

    match parts[0] {
        "d" | "download" => {
            if parts.len() > 1 {
                let url = parts[1].to_string();
                let queue = app.queue.clone();
                let registry = app.registry.clone();

                app.set_status(format!("Queueing: {}", url));

                // Spawn the download task to not block the UI
                tokio::spawn(async move {
                    let _ = crate::engine::enqueue_download(&url, None, registry, queue).await;
                });
            }
        }

        "q" | "quit" => {
            app.quit();
        }
        "help" => {
            app.toggle_help();
        }
        _ => {}
    }
}
