pub mod app;
pub mod assets;
pub mod events;
pub mod terminal;
pub mod themes;
pub mod tui_reporter;
pub mod ui;

use app::App;
use mangofetch_core::core::manager::queue::DownloadQueue;
use mangofetch_core::core::registry::PlatformRegistry;
use std::sync::Arc;
use terminal::TerminalHandler;
use tokio::sync::Mutex;

pub async fn run(
    queue: Arc<Mutex<DownloadQueue>>,
    registry: Arc<PlatformRegistry>,
) -> anyhow::Result<()> {
    // Create TUI reporter that captures output into a shared buffer
    let log_sink = tui_reporter::new_log_sink();
    let reporter = Arc::new(tui_reporter::TuiReporter::new(log_sink.clone()));

    // Replace any existing CLI reporter with our TUI reporter
    {
        let mut q = queue.lock().await;
        q.set_reporter(reporter);
    }

    let mut terminal_handler = TerminalHandler::new()?;
    let mut app = App::new(queue, registry, log_sink);

    while app.running {
        app.drain_reporter_logs();
        app.refresh_data();
        app.clear_status_if_needed();
        terminal_handler
            .terminal()
            .draw(|f| ui::render(f, &mut app))?;
        events::handle_events(&mut app).await?;
    }

    Ok(())
}
