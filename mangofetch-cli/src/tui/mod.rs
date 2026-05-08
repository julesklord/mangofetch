pub mod app;
pub mod assets;
pub mod events;
pub mod terminal;
pub mod themes;
pub mod ui;

use mangofetch_core::core::manager::queue::DownloadQueue;
use mangofetch_core::core::registry::PlatformRegistry;
use std::sync::Arc;
use tokio::sync::Mutex;
use app::App;
use terminal::TerminalHandler;

pub async fn run(
    queue: Arc<Mutex<DownloadQueue>>,
    registry: Arc<PlatformRegistry>,
) -> anyhow::Result<()> {
    let mut terminal_handler = TerminalHandler::new()?;
    let mut app = App::new(queue, registry);

    while app.running {
        app.refresh_data();
        app.clear_status_if_needed();
        terminal_handler
            .terminal()
            .draw(|f| ui::render(f, &mut app))?;
        events::handle_events(&mut app).await?;
    }

    Ok(())
}
