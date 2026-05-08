use mangofetch_core::core::registry::PlatformRegistry;
use mangofetch_core::core::manager::queue::DownloadQueue;
use mangofetch_core::core::dependencies::ensure_dependencies;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

pub fn register_platforms(registry: &mut PlatformRegistry) {
    use mangofetch_core::platforms::*;
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

pub async fn enqueue_download(
    url: &str,
    output_dir: Option<String>,
    registry: Arc<PlatformRegistry>,
    queue: Arc<Mutex<DownloadQueue>>,
) -> Result<()> {
    let downloader = registry
        .find_platform(url)
        .ok_or_else(|| anyhow::anyhow!("No supported platform found for URL"))?;
    let platform_name = downloader.name().to_string();

    let output = output_dir.unwrap_or_else(|| {
        dirs::download_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .to_string_lossy()
            .to_string()
    });

    let deps = ensure_dependencies(false, None).await?;

    // Fetch info
    let media_info = mangofetch_core::core::manager::queue::fetch_and_cache_info(
        url,
        &*downloader,
        &platform_name,
    )
    .await
    .ok();

    static ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(100); // Start high for TUI manually added ones
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
        output,
        None, None, None, None, None, None, None,
        media_info,
        None, None,
        downloader,
        deps.ytdlp,
        false,
    );

    drop(q);
    mangofetch_core::core::manager::queue::try_start_next(queue.clone()).await;
    Ok(())
}
