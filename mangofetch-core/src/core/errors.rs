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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_required() {
        let (code, _) = classify_download_error("Please login to continue");
        assert_eq!(code, "auth_required");
        let (code, _) = classify_download_error("HTTP Error 403: Forbidden");
        assert_eq!(code, "auth_required");
        let (code, _) = classify_download_error("authentication failed");
        assert_eq!(code, "auth_required");
    }

    #[test]
    fn test_rate_limited() {
        let (code, _) = classify_download_error("Too many requests");
        assert_eq!(code, "rate_limited");
        let (code, _) = classify_download_error("HTTP Error 429");
        assert_eq!(code, "rate_limited");
        let (code, _) = classify_download_error("captcha required");
        assert_eq!(code, "rate_limited");
    }

    #[test]
    fn test_restricted() {
        let (code, _) = classify_download_error("This video is private");
        assert_eq!(code, "restricted");
        let (code, _) = classify_download_error("age-restricted content");
        assert_eq!(code, "restricted");
    }

    #[test]
    fn test_file_missing() {
        let (code, _) = classify_download_error("downloaded file not found on disk");
        assert_eq!(code, "file_missing");
    }

    #[test]
    fn test_not_found() {
        let (code, _) = classify_download_error("404 Not Found");
        assert_eq!(code, "not_found");
        let (code, _) = classify_download_error("The video has been deleted");
        assert_eq!(code, "not_found");
        let (code, _) = classify_download_error("content unavailable");
        assert_eq!(code, "not_found");
    }

    #[test]
    fn test_ffmpeg_needed() {
        let (code, _) = classify_download_error("ffmpeg is required");
        assert_eq!(code, "ffmpeg_needed");
        let (code, _) = classify_download_error("error while muxing");
        assert_eq!(code, "ffmpeg_needed");
    }

    #[test]
    fn test_ytdlp_needed() {
        let (code, _) = classify_download_error("yt-dlp is missing");
        assert_eq!(code, "ytdlp_needed");
        let (code, _) = classify_download_error("no downloader found");
        assert_eq!(code, "ytdlp_needed");
    }

    #[test]
    fn test_ytdlp_outdated() {
        let (code, _) = classify_download_error("Unable to extract nsig");
        assert_eq!(code, "ytdlp_outdated");
        let (code, _) = classify_download_error("signature decryption failed");
        assert_eq!(code, "ytdlp_outdated");
    }

    #[test]
    fn test_unknown() {
        let (code, msg) = classify_download_error("some random error");
        assert_eq!(code, "unknown");
        assert_eq!(msg, "some random error");
    }

    #[test]
    fn test_case_insensitivity() {
        let (code, _) = classify_download_error("LOGIN REQUIRED");
        assert_eq!(code, "auth_required");
    }
}
