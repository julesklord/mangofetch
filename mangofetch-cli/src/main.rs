mod formatting;
mod output;
mod reporter; // NEW: Import output formatters module
mod tui; // NEW: Terminal User Interface module
mod engine; // Shared download engine logic

use crate::output::{
    format_about_changelog, format_about_info, format_about_roadmap, format_about_terms,
    format_batch_summary, format_clean_summary, format_config_display, format_dependency_check,
    format_info_card, format_queue_list,
};
use crate::reporter::{BrutalistTheme, CLIReporter, CliTheme};
use anyhow::Result;
use clap::{Parser, Subcommand};
use mangofetch_core::core::manager::queue::DownloadQueue;
use mangofetch_core::core::manager::recovery;
use mangofetch_core::core::registry::PlatformRegistry;
use mangofetch_core::models::queue::QueueStatus;
use mangofetch_core::models::settings::AppSettings;
use std::sync::Arc;

// ============================================================================
// COMMAND LINE INTERFACE DEFINITIONS
// ============================================================================

#[derive(Parser)]
#[command(name = "mangofetch")]
#[command(about = "MangoFetch Command Line Interface", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Theme: 'brutalist' (default), 'zen', or 'auto'
    #[arg(long, default_value = "auto")]
    theme: String,

    /// Force ASCII-only output (no Unicode)
    #[arg(long)]
    ascii_only: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Download media from a URL
    Download {
        /// URL to download
        url: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<String>,

        /// Quality (e.g. 1080p, 720p)
        #[arg(short, long)]
        quality: Option<String>,

        /// Download audio only
        #[arg(short, long)]
        audio_only: bool,
    },

    /// Batch download from a text file
    DownloadMultiple {
        /// Path to the text file containing URLs
        file: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Inspect media info without downloading
    Info {
        /// URL to inspect
        url: String,
    },

    /// Send a local file via P2P
    Send {
        /// Path to the file to send
        file: String,
    },

    /// List downloads in the queue
    List {
        /// Show only active downloads
        #[arg(long)]
        active: bool,

        /// Show only queued downloads
        #[arg(long)]
        queued: bool,

        /// Show only completed downloads
        #[arg(long)]
        completed: bool,

        /// Show only failed downloads
        #[arg(long)]
        failed: bool,
    },

    /// Clean download history
    Clean {
        /// Remove only finished downloads
        #[arg(long)]
        finished: bool,

        /// Remove only failed downloads
        #[arg(long)]
        failed: bool,
    },

    /// Manage settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Check system dependencies
    Check,

    /// Update internal dependencies (yt-dlp, etc.)
    Update,

    /// View activity logs
    Logs {
        /// Number of lines to show
        #[arg(long, default_value_t = 20)]
        tail: usize,
    },

    /// Show application information
    About {
        #[command(subcommand)]
        topic: Option<AboutTopic>,
    },

    /// Launch the interactive Terminal User Interface (TUI)
    Tui,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get a setting value
    Get { key: String },
    /// Set a setting value
    Set { key: String, value: String },
    /// List all settings
    List,
}

#[derive(Subcommand)]
enum AboutTopic {
    Version,
    Roadmap,
    Changelog,
    Terms,
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    mangofetch_core::core::logger::init_logging(cli.verbose);

    // Initialize recovery from disk
    recovery::init_from_disk();

    // Create theme based on CLI flag
    let theme: Arc<dyn CliTheme> = if cli.ascii_only {
        Arc::new(BrutalistTheme::new(false))
    } else {
        // Auto-detect Unicode support (Unix-like systems generally support it)
        let supports_unicode = cfg!(unix) || cfg!(target_os = "macos");
        Arc::new(BrutalistTheme::new(supports_unicode))
    };

    // Create reporter with theme
    let reporter = Arc::new(CLIReporter::with_theme(theme.clone()));

    let mut registry = PlatformRegistry::new();
    engine::register_platforms(&mut registry);
    let registry = Arc::new(registry);

    let queue = Arc::new(tokio::sync::Mutex::new(DownloadQueue::new(
        3,
        Some(reporter.clone()),
    )));

    // ========================================================================
    // COMMAND DISPATCHER
    // ========================================================================

    match cli.command {
        Commands::Download {
            url,
            output,
            quality,
            audio_only,
        } => {
            perform_download(
                &url,
                output,
                quality,
                audio_only,
                &registry,
                &queue,
                reporter.clone(),
                &theme,
            )
            .await?;
            wait_for_queue(&queue).await;
        }

        Commands::DownloadMultiple { file, output } => {
            let content = std::fs::read_to_string(&file)?;
            let urls: Vec<String> = content
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let total = urls.len();
            let mut failed = 0;

            println!(
                "{}📥 Starting batch download of {} URLs{}...\n",
                theme.color_info(),
                total,
                theme.color_reset()
            );

            for (idx, url) in urls.iter().enumerate() {
                println!("  [{}/{}] Queueing: {}", idx + 1, total, url);
                if let Err(e) = perform_download(
                    url,
                    output.clone(),
                    None,
                    false,
                    &registry,
                    &queue,
                    reporter.clone(),
                    &theme,
                )
                .await
                {
                    eprintln!(
                        "  {}✗ Error queueing: {}{}",
                        theme.color_error(),
                        e,
                        theme.color_reset()
                    );
                    failed += 1;
                }
            }

            println!(
                "{}",
                format_batch_summary(total, total - failed, failed, &theme)
            );
            wait_for_queue(&queue).await;
        }

        Commands::Info { url } => {
            let downloader = registry
                .find_platform(&url)
                .ok_or_else(|| anyhow::anyhow!("No supported platform found for URL"))?;
            let platform_name = downloader.name().to_string();

            println!(
                "{}🔍 Fetching media info...{}\n",
                theme.color_info(),
                theme.color_reset()
            );

            let info = mangofetch_core::core::manager::queue::fetch_and_cache_info(
                &url,
                &*downloader,
                &platform_name,
            )
            .await?;

            // Use formatted card output
            let card = format_info_card(
                &info.title,
                &info.author,
                &info.platform,
                info.duration_seconds,
                &format!("{:?}", info.media_type),
                &theme,
            );
            println!("{}", card);
        }

        Commands::Send { file } => {
            println!(
                "{}⚠ P2P Send not yet implemented in CLI. File: {}{}",
                theme.color_warning(),
                file,
                theme.color_reset()
            );
        }

        Commands::List {
            active,
            queued,
            completed,
            failed,
        } => {
            let items = recovery::list();
            let mut filtered = items;

            let any_filter = active || queued || completed || failed;

            if any_filter {
                filtered = filtered
                    .into_iter()
                    .filter(|i| {
                        (active && matches!(i.status, QueueStatus::Active))
                            || (queued && matches!(i.status, QueueStatus::Queued))
                            || (completed && matches!(i.status, QueueStatus::Complete { .. }))
                            || (failed && matches!(i.status, QueueStatus::Error { .. }))
                            || (active && matches!(i.status, QueueStatus::Seeding))
                            || (queued && matches!(i.status, QueueStatus::Paused))
                    })
                    .collect();
            }

            // Convert to displayable format
            let display_items: Vec<_> = filtered
                .iter()
                .map(|i| {
                    let status_str = match &i.status {
                        QueueStatus::Active => "Active".to_string(),
                        QueueStatus::Queued => "Queued".to_string(),
                        QueueStatus::Paused => "Paused".to_string(),
                        QueueStatus::Seeding => "Seeding".to_string(),
                        QueueStatus::Complete { .. } => "Complete".to_string(),
                        QueueStatus::Error { message } => format!("Error: {}", message),
                    };
                    let title = if i.title.len() > 35 {
                        format!("{}...", &i.title[..32])
                    } else {
                        i.title.clone()
                    };
                    (i.id, title, i.platform.clone(), status_str, String::new())
                })
                .collect();

            println!("{}", format_queue_list(display_items, &theme));
        }

        Commands::Clean { finished, failed } => {
            let items_before = recovery::list().len();

            if finished || failed {
                eprintln!(
                    "{}ℹ Selective cleaning not yet implemented, clearing all...{}",
                    theme.color_warning(),
                    theme.color_reset()
                );
            }

            recovery::clear_all();

            println!("{}", format_clean_summary(items_before, None, &theme));
        }

        Commands::Config { action } => {
            let mut settings = AppSettings::load_from_disk();
            match action {
                ConfigAction::Get { key } => {
                    let val = serde_json::to_value(&settings)?;
                    if let Some(v) = get_json_path(&val, &key) {
                        println!(
                            "{}{}{}  = {}",
                            theme.color_accent(),
                            key,
                            theme.color_reset(),
                            v
                        );
                    } else {
                        println!(
                            "{}Key not found: {}{}",
                            theme.color_warning(),
                            key,
                            theme.color_reset()
                        );
                    }
                }

                ConfigAction::Set { key, value } => {
                    let mut val = serde_json::to_value(&settings)?;
                    if set_json_path(&mut val, &key, &value) {
                        settings = serde_json::from_value(val)?;
                        settings.save_to_disk()?;
                        println!(
                            "{}✓ Set {}{}  = {}",
                            theme.color_success(),
                            key,
                            theme.color_reset(),
                            value
                        );
                    } else {
                        println!(
                            "{}✗ Failed to set key: {}{}",
                            theme.color_error(),
                            key,
                            theme.color_reset()
                        );
                    }
                }

                ConfigAction::List => {
                    let settings_json = serde_json::to_string_pretty(&settings)?;
                    println!("{}", format_config_display(&settings_json, &theme));
                }
            }
        }

        Commands::Check => {
            println!(
                "{}🔍 Checking system dependencies...{}\n",
                theme.color_info(),
                theme.color_reset()
            );

            match mangofetch_core::core::dependencies::ensure_dependencies(
                false,
                Some(reporter.clone()),
            )
            .await
            {
                Ok(deps) => {
                    let yt_dlp_path = deps.ytdlp.as_ref().map(|p| p.to_string_lossy().to_string());
                    let ffmpeg_path = deps
                        .ffmpeg
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string());

                    println!(
                        "{}",
                        format_dependency_check(
                            yt_dlp_path.as_deref(),
                            ffmpeg_path.as_deref(),
                            &theme
                        )
                    );
                }
                Err(e) => println!(
                    "{}❌ Dependency check failed: {}{}",
                    theme.color_error(),
                    e,
                    theme.color_reset()
                ),
            }
        }

        Commands::Update => {
            println!(
                "{}⬆️  Updating dependencies (yt-dlp, FFmpeg)...{}\n",
                theme.color_accent(),
                theme.color_reset()
            );

            match mangofetch_core::core::dependencies::ensure_dependencies(
                true,
                Some(reporter.clone()),
            )
            .await
            {
                Ok(deps) => {
                    let yt_dlp_path = deps.ytdlp.as_ref().map(|p| p.to_string_lossy().to_string());
                    let ffmpeg_path = deps
                        .ffmpeg
                        .as_ref()
                        .map(|p| p.to_string_lossy().to_string());

                    println!(
                        "{}✓ Update complete.{}\n",
                        theme.color_success(),
                        theme.color_reset()
                    );
                    println!(
                        "{}",
                        format_dependency_check(
                            yt_dlp_path.as_deref(),
                            ffmpeg_path.as_deref(),
                            &theme
                        )
                    );
                }
                Err(e) => println!(
                    "{}❌ Update failed: {}{}",
                    theme.color_error(),
                    e,
                    theme.color_reset()
                ),
            }
        }

        Commands::Logs { tail } => {
            let log_dir = mangofetch_core::core::paths::app_data_dir().map(|d| d.join("logs"));
            if let Some(dir) = log_dir {
                if !dir.exists() {
                    println!(
                        "{}ℹ No logs found.{}",
                        theme.color_info(),
                        theme.color_reset()
                    );
                    return Ok(());
                }

                let entries = std::fs::read_dir(&dir)?;
                let mut files: Vec<_> = entries
                    .flatten()
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "log"))
                    .collect();

                files.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());

                if let Some(last_file) = files.last() {
                    let path = last_file.path();
                    println!(
                        "{}📋 Last {} lines from:{}  {}\n",
                        theme.color_info(),
                        tail,
                        theme.color_reset(),
                        path.display()
                    );

                    let content = std::fs::read_to_string(&path)?;
                    let lines: Vec<_> = content.lines().collect();
                    let start = lines.len().saturating_sub(tail);

                    for line in &lines[start..] {
                        println!("  {}", line);
                    }
                } else {
                    println!(
                        "{}ℹ No log files found in: {}{}",
                        theme.color_info(),
                        dir.display(),
                        theme.color_reset()
                    );
                }
            } else {
                println!(
                    "{}⚠ Could not determine log directory.{}",
                    theme.color_warning(),
                    theme.color_reset()
                );
            }
        }

        Commands::About { topic } => {
            let topic = topic.unwrap_or(AboutTopic::Version);
            match topic {
                AboutTopic::Version => {
                    println!(
                        "{}",
                        format_about_info(
                            env!("CARGO_PKG_VERSION"),
                            "Jules Martins",
                            "https://github.com/julesklord/mangofetch-cli",
                            &theme
                        )
                    );
                }
                AboutTopic::Roadmap => {
                    println!("{}", format_about_roadmap(&theme));
                }
                AboutTopic::Changelog => {
                    println!("{}", format_about_changelog(&theme));
                }
                AboutTopic::Terms => {
                    println!("{}", format_about_terms(&theme));
                }
            }
        }

        Commands::Tui => {
            tui::run(queue.clone(), registry.clone()).await?;
        }
    }

    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

async fn perform_download(
    url: &str,
    output: Option<String>,
    quality: Option<String>,
    audio_only: bool,
    registry: &PlatformRegistry,
    queue: &Arc<tokio::sync::Mutex<DownloadQueue>>,
    reporter: Arc<crate::reporter::CLIReporter>,
    theme: &Arc<dyn CliTheme>,
) -> Result<()> {
    let downloader = registry
        .find_platform(url)
        .ok_or_else(|| anyhow::anyhow!("No supported platform found for URL"))?;
    let platform_name = downloader.name().to_string();

    let output_dir = output.unwrap_or_else(|| {
        dirs::download_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .to_string_lossy()
            .to_string()
    });

    let deps =
        mangofetch_core::core::dependencies::ensure_dependencies(false, Some(reporter.clone()))
            .await?;

    // Pre-fetch info to show title
    let media_info = mangofetch_core::core::manager::queue::fetch_and_cache_info(
        url,
        &*downloader,
        &platform_name,
    )
    .await
    .ok();

    if let Some(ref info) = media_info {
        println!(
            "{}✓ Queued:{} {} [{}]",
            theme.color_success(),
            theme.color_reset(),
            info.title,
            platform_name
        );
    } else {
        println!(
            "{}✓ Queued:{} {} [{}]",
            theme.color_success(),
            theme.color_reset(),
            url,
            platform_name
        );
    }

    static ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let mut q = queue.lock().await;
    q.enqueue(
        id,
        url.to_string(),
        platform_name,
        media_info
            .as_ref()
            .map(|i| i.title.clone())
            .unwrap_or_else(|| url.to_string()),
        output_dir,
        None,
        quality,
        None,
        None,
        None,
        None,
        None,
        media_info,
        None,
        None,
        downloader,
        deps.ytdlp,
        audio_only,
    );

    drop(q);
    mangofetch_core::core::manager::queue::try_start_next(queue.clone()).await;
    Ok(())
}

async fn wait_for_queue(queue: &Arc<tokio::sync::Mutex<DownloadQueue>>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let q = queue.lock().await;
        if q.active_count() == 0 && q.next_queued_ids().is_empty() {
            break;
        }
    }
}

fn get_json_path(val: &serde_json::Value, path: &str) -> Option<String> {
    let mut curr = val;
    for part in path.split('.') {
        curr = curr.get(part)?;
    }
    if curr.is_string() {
        Some(curr.as_str()?.to_string())
    } else {
        Some(curr.to_string())
    }
}

fn set_json_path(val: &mut serde_json::Value, path: &str, value: &str) -> bool {
    let mut curr = val;
    let parts: Vec<&str> = path.split('.').collect();
    for (i, part) in parts.iter().enumerate() {
        if i == parts.len() - 1 {
            if let Some(obj) = curr.as_object_mut() {
                let json_val = serde_json::from_str(value)
                    .unwrap_or(serde_json::Value::String(value.to_string()));
                obj.insert(part.to_string(), json_val);
                return true;
            }
        } else {
            curr = match curr.get_mut(*part) {
                Some(v) => v,
                None => return false,
            };
        }
    }
    false
}
