use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub schema_version: u32,
    pub appearance: AppearanceSettings,
    pub download: DownloadSettings,
    pub advanced: AdvancedSettings,
    #[serde(default)]
    pub telegram: TelegramSettings,
    #[serde(default)]
    pub proxy: ProxySettings,
    #[serde(default)]
    pub onboarding_completed: bool,
    #[serde(default)]
    pub start_with_windows: bool,
    #[serde(default)]
    pub portable_mode: bool,
    #[serde(default)]
    pub legal_acknowledged: bool,
    #[serde(default)]
    pub last_download_options: LastDownloadOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastDownloadOptions {
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub quality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,
    pub language: String,
    #[serde(default = "default_tui_theme")]
    pub tui_theme: String,
    #[serde(default)]
    pub use_nerd_fonts: bool,
}

fn default_tui_theme() -> String {
    "mango".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    pub default_output_dir: PathBuf,
    pub always_ask_path: bool,
    pub video_quality: String,
    pub skip_existing: bool,
    pub download_attachments: bool,
    pub download_descriptions: bool,
    #[serde(default = "default_true")]
    pub embed_metadata: bool,
    #[serde(default = "default_true")]
    pub embed_thumbnail: bool,
    #[serde(default)]
    pub clipboard_detection: bool,
    #[serde(default = "default_filename_template")]
    pub filename_template: String,
    #[serde(default)]
    pub organize_by_platform: bool,
    #[serde(default)]
    pub download_subtitles: bool,
    #[serde(default)]
    pub include_auto_subtitles: bool,
    #[serde(default)]
    pub translate_metadata: bool,
    #[serde(default)]
    pub youtube_sponsorblock: bool,
    #[serde(default)]
    pub split_by_chapters: bool,
    #[serde(default)]
    pub hotkey_enabled: bool,
    #[serde(default = "default_hotkey_binding")]
    pub hotkey_binding: String,
    #[serde(default)]
    pub extra_ytdlp_flags: Vec<String>,
    #[serde(default = "default_true")]
    pub copy_to_clipboard_on_hotkey: bool,
    #[serde(default)]
    pub cookie_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    pub max_concurrent_segments: u32,
    pub max_retries: u32,
    #[serde(default = "default_max_concurrent_downloads")]
    pub max_concurrent_downloads: u32,
    #[serde(default = "default_concurrent_fragments")]
    pub concurrent_fragments: u32,
    #[serde(default = "default_stagger_delay_ms")]
    pub stagger_delay_ms: u64,
    #[serde(default = "default_torrent_listen_port")]
    pub torrent_listen_port: u16,
    #[serde(default)]
    pub cookies_from_browser: String,
    #[serde(default)]
    pub twitter_manual_cookie: String,
}

fn default_concurrent_fragments() -> u32 {
    8
}

fn default_max_concurrent_downloads() -> u32 {
    2
}

fn default_stagger_delay_ms() -> u64 {
    150
}

fn default_torrent_listen_port() -> u16 {
    6881
}

fn default_true() -> bool {
    true
}

pub fn default_filename_template() -> String {
    "%(title).200s [%(id)s].%(ext)s".into()
}

fn default_hotkey_binding() -> String {
    "CmdOrCtrl+Shift+D".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramSettings {
    pub concurrent_downloads: u32,
    pub fix_file_extensions: bool,
}

impl Default for TelegramSettings {
    fn default() -> Self {
        Self {
            concurrent_downloads: 3,
            fix_file_extensions: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxySettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_proxy_type")]
    pub proxy_type: String,
    #[serde(default)]
    pub host: String,
    #[serde(default = "default_proxy_port")]
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}

fn default_proxy_type() -> String {
    "http".into()
}

fn default_proxy_port() -> u16 {
    8080
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            schema_version: 1,
            appearance: AppearanceSettings {
                theme: "system".into(),
                language: "en".into(),
                tui_theme: "mango".into(),
                use_nerd_fonts: false,
            },
            download: DownloadSettings {
                default_output_dir: dirs::download_dir().unwrap_or_else(|| PathBuf::from(".")),
                always_ask_path: true,
                video_quality: "720p".into(),
                skip_existing: true,
                download_attachments: true,
                download_descriptions: true,
                embed_metadata: true,
                embed_thumbnail: true,
                clipboard_detection: false,
                filename_template: default_filename_template(),
                organize_by_platform: false,
                download_subtitles: false,
                include_auto_subtitles: false,
                translate_metadata: false,
                youtube_sponsorblock: false,
                split_by_chapters: false,
                hotkey_enabled: false,
                hotkey_binding: default_hotkey_binding(),
                extra_ytdlp_flags: Vec::new(),
                copy_to_clipboard_on_hotkey: true,
                cookie_file: String::new(),
            },
            advanced: AdvancedSettings {
                max_concurrent_segments: 20,
                max_retries: 3,
                max_concurrent_downloads: 2,
                concurrent_fragments: 8,
                stagger_delay_ms: 150,
                torrent_listen_port: 6881,
                cookies_from_browser: String::new(),
                twitter_manual_cookie: String::new(),
            },
            telegram: TelegramSettings::default(),
            proxy: ProxySettings::default(),
            onboarding_completed: false,
            start_with_windows: false,
            portable_mode: false,
            legal_acknowledged: false,
            last_download_options: LastDownloadOptions::default(),
        }
    }
}

impl AppSettings {
    pub fn load_from_disk() -> Self {
        let data_dir = match crate::core::paths::app_data_dir() {
            Some(d) => d,
            None => return Self::default(),
        };
        Self::load_from_path(&data_dir.join("settings.json"))
    }

    pub fn load_from_path(store_path: &std::path::Path) -> Self {
        let content = match std::fs::read_to_string(store_path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };
        let json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => return Self::default(),
        };
        match json.get("app_settings") {
            Some(val) => serde_json::from_value::<Self>(val.clone()).unwrap_or_default(),
            None => Self::default(),
        }
    }

    pub fn save_to_disk(&self) -> anyhow::Result<()> {
        let data_dir = crate::core::paths::app_data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find app data dir"))?;
        self.save_to_path(&data_dir.join("settings.json"))
    }

    pub fn save_to_path(&self, store_path: &std::path::Path) -> anyhow::Result<()> {
        let mut json = if store_path.exists() {
            let content = std::fs::read_to_string(store_path)?;
            serde_json::from_str::<serde_json::Value>(&content).unwrap_or(serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        if let Some(obj) = json.as_object_mut() {
            obj.insert("app_settings".to_string(), serde_json::to_value(self)?);
        }

        let serialized = serde_json::to_string_pretty(&json)?;
        std::fs::write(store_path, serialized)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_default_when_no_file() {
        let uuid = uuid::Uuid::new_v4().to_string();
        let path = std::env::temp_dir().join(&uuid).join("settings.json");
        let settings = AppSettings::load_from_path(&path);
        assert_eq!(settings.appearance.theme, "system");
    }

    #[test]
    fn test_save_and_load_settings() {
        let uuid = uuid::Uuid::new_v4().to_string();
        let dir = std::env::temp_dir().join(&uuid);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("settings.json");

        let mut settings = AppSettings::default();
        settings.appearance.theme = "dark".into();
        settings.save_to_path(&path).unwrap();

        let loaded = AppSettings::load_from_path(&path);
        assert_eq!(loaded.appearance.theme, "dark");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_load_invalid_json() {
        let uuid = uuid::Uuid::new_v4().to_string();
        let dir = std::env::temp_dir().join(&uuid);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("settings.json");

        fs::write(&path, "{ invalid json }").unwrap();

        let settings = AppSettings::load_from_path(&path);
        assert_eq!(settings.appearance.theme, "system"); // Should return default
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_save_preserves_other_keys() {
        let uuid = uuid::Uuid::new_v4().to_string();
        let dir = std::env::temp_dir().join(&uuid);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("settings.json");

        // Create initial JSON with an extra key
        let initial_json = serde_json::json!({
            "other_plugin_data": { "key": "value" },
            "app_settings": {}
        });
        fs::write(&path, serde_json::to_string(&initial_json).unwrap()).unwrap();

        // Save settings
        let settings = AppSettings::default();
        settings.save_to_path(&path).unwrap();

        // Verify other key is preserved
        let content = fs::read_to_string(&path).unwrap();
        let saved_json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert!(saved_json.get("other_plugin_data").is_some());
        assert_eq!(saved_json["other_plugin_data"]["key"], "value");
        assert!(saved_json.get("app_settings").is_some());
        let _ = fs::remove_dir_all(dir);
    }
}
