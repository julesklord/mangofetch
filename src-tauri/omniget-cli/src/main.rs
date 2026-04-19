mod reporter;

use anyhow::Result;
use clap::{Parser, Subcommand};
use omniget_core::core::manager::queue::DownloadQueue;
use omniget_core::models::queue::QueueStatus;
use omniget_core::core::registry::PlatformRegistry;
use omniget_core::models::settings::AppSettings;
use omniget_core::core::manager::recovery;
use crate::reporter::CLIReporter;
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "omniget-cli")]
#[command(about = "OmniGet Command Line Interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    omniget_core::core::logger::init_logging(cli.verbose);

    // Initialize recovery from disk
    recovery::init_from_disk();

    let mut registry = PlatformRegistry::new();
    register_platforms(&mut registry);

    let reporter: omniget_core::core::traits::SharedReporter = Arc::new(CLIReporter::new());
    let queue = Arc::new(tokio::sync::Mutex::new(DownloadQueue::new(3, Some(reporter.clone()))));

    match cli.command {
        Commands::Download { url, output, quality, audio_only } => {
            perform_download(&url, output, quality, audio_only, &registry, &queue, reporter.clone()).await?;
            wait_for_queue(&queue).await;
        },
        Commands::DownloadMultiple { file, output } => {
            let content = std::fs::read_to_string(&file)?;
            let urls: Vec<String> = content.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            
            println!("Starting batch download of {} URLs...", urls.len());
            for url in urls {
                if let Err(e) = perform_download(&url, output.clone(), None, false, &registry, &queue, reporter.clone()).await {
                    eprintln!("Error queueing {}: {}", url, e);
                }
            }
            wait_for_queue(&queue).await;
        },
        Commands::Info { url } => {
            let downloader = registry.find_platform(&url).ok_or_else(|| anyhow::anyhow!("No supported platform found for URL"))?;
            let platform_name = downloader.name().to_string();

            println!("Fetching media info for: {}", url);
            let info = omniget_core::core::manager::queue::fetch_and_cache_info(&url, &*downloader, &platform_name).await?;
            
            println!("--- Media Info ---");
            println!("Title:    {}", info.title);
            println!("Author:   {}", info.author);
            println!("Platform: {}", info.platform);
            if let Some(duration) = info.duration_seconds {
                println!("Duration: {:.1} seconds", duration);
            }
            println!("Type:     {:?}", info.media_type);
            println!("------------------");
        },
        Commands::Send { file } => {
            println!("P2P Send not yet implemented in CLI. File: {}", file);
        },
        Commands::List { active, queued, completed, failed } => {
            let items = recovery::list();
            let mut filtered = items;
            
            let any_filter = active || queued || completed || failed;
            
            if any_filter {
                filtered = filtered.into_iter().filter(|i| {
                    (active && matches!(i.status, QueueStatus::Active)) ||
                    (queued && matches!(i.status, QueueStatus::Queued)) ||
                    (completed && matches!(i.status, QueueStatus::Complete { .. })) ||
                    (failed && matches!(i.status, QueueStatus::Error { .. }))
                }).collect();
            }

            if filtered.is_empty() {
                println!("No downloads found matching the criteria.");
            } else {
                println!("{:<5} {:<30} {:<15} {:<10}", "ID", "Title", "Platform", "Status");
                println!("{:-<60}", "");
                for item in filtered {
                    println!("{:<5} {:<30} {:<15} {:<10?}", item.id, item.title, item.platform, item.status);
                }
            }
        },
        Commands::Clean { finished, failed } => {
            if finished || failed {
                 println!("Selective cleaning not yet implemented, clearing all...");
            }
            recovery::clear_all();
            println!("Queue cleared.");
        },
        Commands::Config { action } => {
            let mut settings = AppSettings::load_from_disk();
            match action {
                ConfigAction::Get { key } => {
                    let val = serde_json::to_value(&settings)?;
                    if let Some(v) = get_json_path(&val, &key) {
                        println!("{} = {}", key, v);
                    } else {
                        println!("Key not found: {}", key);
                    }
                },
                ConfigAction::Set { key, value } => {
                    let mut val = serde_json::to_value(&settings)?;
                    if set_json_path(&mut val, &key, &value) {
                        settings = serde_json::from_value(val)?;
                        settings.save_to_disk()?;
                        println!("Set {} = {}", key, value);
                    } else {
                        println!("Failed to set key: {}", key);
                    }
                },
                ConfigAction::List => {
                    println!("{}", serde_json::to_string_pretty(&settings)?);
                },
            }
        },
        Commands::Check => {
            println!("Checking system dependencies...");
            match omniget_core::core::dependencies::ensure_dependencies(false, Some(reporter.clone())).await {
                Ok(deps) => {
                    println!("✅ yt-dlp: Found at {:?}", deps.ytdlp);
                    println!("✅ FFmpeg: Found at {:?}", deps.ffmpeg);
                },
                Err(e) => println!("❌ Dependency check failed: {}", e),
            }
        },
        Commands::Update => {
            println!("Updating dependencies (yt-dlp, FFmpeg)...");
            match omniget_core::core::dependencies::ensure_dependencies(true, Some(reporter.clone())).await {
                Ok(deps) => {
                    println!("✅ Update complete.");
                    println!("   yt-dlp: {:?}", deps.ytdlp);
                    println!("   FFmpeg: {:?}", deps.ffmpeg);
                },
                Err(e) => println!("❌ Update failed: {}", e),
            }
        },
        Commands::Logs { tail } => {
            let log_dir = omniget_core::core::paths::app_data_dir().map(|d| d.join("logs"));
            if let Some(dir) = log_dir {
                if !dir.exists() {
                    println!("No logs found.");
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
                    println!("Showing last {} lines from {}", tail, path.display());
                    let content = std::fs::read_to_string(&path)?;
                    let lines: Vec<_> = content.lines().collect();
                    let start = lines.len().saturating_sub(tail);
                    for line in &lines[start..] {
                        println!("{}", line);
                    }
                } else {
                    println!("No log files found in {}", dir.display());
                }
            } else {
                println!("Could not determine log directory.");
            }
        },
        Commands::About { topic } => {
            let topic = topic.unwrap_or(AboutTopic::Version);
            match topic {
                AboutTopic::Version => {
                    println!("OmniGet CLI v0.1.0");
                    println!("Build Edition: 2021");
                },
                AboutTopic::Roadmap => println!("Roadmap: TUI integration, Plugin Manager, P2P support."),
                AboutTopic::Changelog => println!("Changelog: v0.1.0 - Initial CLI release with unified core."),
                AboutTopic::Terms => println!("Terms: Open source. Respect content creator rights."),
            }
        }
    }

    Ok(())
}

fn register_platforms(registry: &mut PlatformRegistry) {
    use omniget_lib::platforms::*;
    registry.register(Arc::new(instagram::InstagramDownloader::new()));
    registry.register(Arc::new(pinterest::PinterestDownloader::new()));
    registry.register(Arc::new(tiktok::TikTokDownloader::new()));
    registry.register(Arc::new(twitter::TwitterDownloader::new()));
    registry.register(Arc::new(twitch::TwitchClipsDownloader::new()));
    registry.register(Arc::new(bluesky::BlueskyDownloader::new()));
    registry.register(Arc::new(reddit::RedditDownloader::new()));
    registry.register(Arc::new(youtube::YouTubeDownloader::new()));
    registry.register(Arc::new(vimeo::VimeoDownloader::new()));
    registry.register(Arc::new(bilibili::BilibiliDownloader::new()));
    let torrent_session = Arc::new(tokio::sync::Mutex::new(None));
    registry.register(Arc::new(magnet::MagnetDownloader::new(torrent_session)));
    registry.register(Arc::new(p2p::P2pDownloader::new()));
    registry.register(Arc::new(generic_ytdlp::GenericYtdlpDownloader::new()));
}

async fn perform_download(
    url: &str,
    output: Option<String>,
    quality: Option<String>,
    audio_only: bool,
    registry: &PlatformRegistry,
    queue: &Arc<tokio::sync::Mutex<DownloadQueue>>,
    reporter: omniget_core::core::traits::SharedReporter,
) -> Result<()> {
    let downloader = registry.find_platform(url).ok_or_else(|| anyhow::anyhow!("No supported platform found for URL"))?;
    let platform_name = downloader.name().to_string();

    let output_dir = output.unwrap_or_else(|| {
        dirs::download_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .to_string_lossy()
            .to_string()
    });

    let deps = omniget_core::core::dependencies::ensure_dependencies(false, Some(reporter.clone())).await?;

    // Pre-fetch info to show title
    let media_info = omniget_core::core::manager::queue::fetch_and_cache_info(
        url,
        &*downloader,
        &platform_name,
    ).await.ok();

    if let Some(ref info) = media_info {
        println!("Queueing: {} [{}]", info.title, platform_name);
    } else {
        println!("Queueing: {} [{}]", url, platform_name);
    }

    static ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let id = ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    
    let mut q = queue.lock().await;
    q.enqueue(
        id,
        url.to_string(),
        platform_name,
        media_info.as_ref().map(|i| i.title.clone()).unwrap_or_else(|| url.to_string()),
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
    omniget_core::core::manager::queue::try_start_next(queue.clone()).await;
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
            // Last part, set the value
            if let Some(obj) = curr.as_object_mut() {
                // Try to parse value as JSON (for numbers, bools, etc)
                let json_val = serde_json::from_str(value).unwrap_or(serde_json::Value::String(value.to_string()));
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
