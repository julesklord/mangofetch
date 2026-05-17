use thiserror::Error;

#[derive(Error, Debug)]
pub enum MangoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Dependency missing: {0}")]
    DependencyMissing(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("Download error: {0}")]
    Download(String),

    #[error("Authentication required: {0}")]
    AuthRequired(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Content restricted: {0}")]
    Restricted(String),

    #[error("Content not found: {0}")]
    NotFound(String),

    #[error("FFmpeg error: {0}")]
    FFmpeg(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("{0}")]
    Custom(String),
}

pub type MangoResult<T> = Result<T, MangoError>;

pub fn classify_download_error(error: &str) -> (&str, &str) {
    let lower = error.to_lowercase();

    if lower.contains("cookie")
        || lower.contains("login")
        || lower.contains("sign in")
        || lower.contains("authentication")
        || lower.contains("403")
    {
        return ("auth_required", "This content requires login. Install the browser extension and visit the site while logged in.");
    }

    if lower.contains("captcha")
        || lower.contains("blocking")
        || lower.contains("rate limit")
        || lower.contains("429")
        || lower.contains("too many")
    {
        return (
            "rate_limited",
            "Too many requests. Try again in a few minutes.",
        );
    }

    if lower.contains("private") || lower.contains("restricted") || lower.contains("age") {
        return ("restricted", "This content is private or age-restricted.");
    }

    if lower.contains("downloaded file") && lower.contains("not found") {
        return (
            "file_missing",
            "Downloaded file could not be located in the output folder.",
        );
    }

    if lower.contains("not found")
        || lower.contains("404")
        || lower.contains("unavailable")
        || lower.contains("deleted")
    {
        return ("not_found", "Content not found or has been deleted.");
    }

    if lower.contains("ffmpeg") || lower.contains("mux") || lower.contains("merge") {
        return (
            "ffmpeg_needed",
            "FFmpeg is required for this download. Install it from Settings.",
        );
    }

    if lower.contains("yt-dlp") || lower.contains("ytdlp") || lower.contains("no downloader") {
        return (
            "ytdlp_needed",
            "yt-dlp is required. Install it from Settings.",
        );
    }

    if lower.contains("nsig") || lower.contains("signature") || lower.contains("cipher") {
        return (
            "ytdlp_outdated",
            "yt-dlp needs updating. Restart the app to auto-update.",
        );
    }

    ("unknown", error)
}
