# 🔌 Plugin Development SDK

> **Note**: The Plugin SDK is currently in **Beta** as of v0.5.0.

MangoFetch is designed to be infinitely extensible. If a platform is not supported natively or via yt-dlp, you can build your own extractor using our Rust SDK.

## 🏗️ The `PlatformDownloader` Trait

All plugins must implement the `PlatformDownloader` trait defined in `mangofetch-core`.

```rust
#[async_trait]
pub trait PlatformDownloader: Send + Sync {
    fn name(&self) -> &str;
    fn can_handle(&self, url: &str) -> bool;
    async fn get_media_info(&self, url: &str) -> anyhow::Result<MediaInfo>;
    async fn download(
        &self,
        info: &MediaInfo,
        opts: &DownloadOptions,
        progress: mpsc::Sender<f64>,
    ) -> anyhow::Result<DownloadResult>;
}
```

## 🛠️ Development Workflow

1.  **Initialize**: Create a new library crate and add `mangofetch-plugin-sdk` as a dependency.
2.  **Implement**: Write your extraction logic.
3.  **Compile**: Build as a dynamic library (`.so`, `.dll`, or `.dylib`).
4.  **Install**: Place the compiled library in the `plugins/` directory of MangoFetch.

---

Detailed tutorials and ABI stability documentation are coming soon in **v0.6.0**.
