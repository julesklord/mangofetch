// output.rs
// ============================================================================
// Enhanced Output Formatting Module for OmniGet CLI
// Place in: src-tauri/omniget-cli/src/output.rs
// ============================================================================

use crate::reporter::CliTheme;
use std::sync::Arc;

/// Formats the `info` command output with a beautiful card-style layout
pub fn format_info_card(
    title: &str,
    author: &str,
    platform: &str,
    duration_seconds: Option<f64>,
    media_type: &str,
    theme: &Arc<dyn CliTheme>,
) -> String {
    let duration_str = duration_seconds
        .map(|d| {
            let secs = d as u64;
            if secs < 60 {
                format!("{}s", secs)
            } else if secs < 3600 {
                format!("{}m {}s", secs / 60, secs % 60)
            } else {
                format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
            }
        })
        .unwrap_or_else(|| "—".to_string());

    let box_char = "─";
    let corner_tl = "┌";
    let corner_tr = "┐";
    let corner_bl = "└";
    let corner_br = "┘";
    let vert = "│";

    let width = 60;
    let box_line = format!("{}{}{}", corner_tl, box_char.repeat(width - 2), corner_tr);
    let closing = format!("{}{}{}", corner_bl, box_char.repeat(width - 2), corner_br);

    let platform_colored = format!(
        "{}{}{}",
        theme.color_platform(platform),
        platform,
        theme.color_reset()
    );

    format!(
        "{}\n{} {}Media Information{}\n{} Title:     {}\n{} Author:    {}\n{} Platform:  {}\n{} Type:      {}\n{} Duration:  {}\n{}",
        box_line,
        vert,
        theme.color_info(),
        theme.color_reset(),
        vert,
        title,
        vert,
        author,
        vert,
        platform_colored,
        vert,
        media_type,
        vert,
        duration_str,
        closing,
    )
}

/// Formats the `list` command output with better visual hierarchy
pub fn format_queue_list(
    items: Vec<(u64, String, String, String, String)>, // (id, title, platform, status, progress)
    theme: &Arc<dyn CliTheme>,
) -> String {
    if items.is_empty() {
        return format!(
            "{}ℹ No downloads found.{}",
            theme.color_info(),
            theme.color_reset()
        );
    }

    let mut output = String::new();
    output.push_str(&format!(
        "\n{}📋 Queue Status (Total: {}){}:\n",
        theme.color_info(),
        items.len(),
        theme.color_reset()
    ));

    let separator = format!("  {}", "─".repeat(55));
    output.push_str(&separator);
    output.push('\n');

    for (_idx, (id, title, platform, status, progress)) in items.iter().enumerate() {
        let status_icon = match status.as_str() {
            s if s.contains("Active") => "▶",
            s if s.contains("Queued") => "○",
            s if s.contains("Complete") => "✓",
            s if s.contains("Error") => "✗",
            _ => "⏳",
        };

        let status_colored = match status.as_str() {
            s if s.contains("Active") => format!(
                "{}{}{}",
                theme.color_accent(),
                status,
                theme.color_reset()
            ),
            s if s.contains("Complete") => format!(
                "{}{}{}",
                theme.color_success(),
                status,
                theme.color_reset()
            ),
            s if s.contains("Error") => format!(
                "{}{}{}",
                theme.color_error(),
                status,
                theme.color_reset()
            ),
            _ => format!(
                "{}{}{}",
                theme.color_warning(),
                status,
                theme.color_reset()
            ),
        };

        let platform_colored = format!(
            "{}[{}]{}",
            theme.color_platform(platform),
            platform,
            theme.color_reset()
        );

        output.push_str(&format!(
            "  {} #{:<3} {:<35} {} {}\n",
            status_icon, id, title, platform_colored, status_colored
        ));

        // Show progress bar inline for active downloads
        if status.contains("Active") && !progress.is_empty() {
            output.push_str(&format!("         {}\n", progress));
        }
    }

    output.push_str(&separator);
    output.push('\n');

    output
}

/// Formats the `config list` output with better structure
pub fn format_config_display(
    config_json: &str,
    theme: &Arc<dyn CliTheme>,
) -> String {
    // Parse and pretty-print with colors
    let mut output = format!(
        "{}⚙️  Configuration{}:\n",
        theme.color_info(),
        theme.color_reset()
    );

    if let Ok(value) = serde_json::from_str::<serde_json::Value>(config_json) {
        output.push_str(&format_json_pretty(&value, theme, 0));
    } else {
        output.push_str(config_json);
    }

    output
}

/// Helper to pretty-print JSON with colors
fn format_json_pretty(value: &serde_json::Value, theme: &Arc<dyn CliTheme>, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let next_indent = "  ".repeat(indent + 1);

    match value {
        serde_json::Value::Object(map) => {
            let mut result = String::from("{\n");
            for (i, (key, val)) in map.iter().enumerate() {
                result.push_str(&format!(
                    "{}{}{}{}{}: {}",
                    next_indent,
                    theme.color_accent(),
                    key,
                    theme.color_reset(),
                    "",
                    format_json_pretty(val, theme, indent + 1).trim_start()
                ));
                if i < map.len() - 1 {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            }
            result.push_str(&format!("{}}}\n", indent_str));
            result
        }
        serde_json::Value::Array(arr) => {
            let mut result = String::from("[\n");
            for (i, item) in arr.iter().enumerate() {
                let comma = if i < arr.len() - 1 { "," } else { "" };
                result.push_str(&next_indent);
                result.push_str(&format_json_pretty(item, theme, indent + 1).trim_start());
                if !comma.is_empty() {
                    result.push(',');
                }
                result.push('\n');
            }
            result.push_str(&format!("{}]\n", indent_str));
            result
        }
        serde_json::Value::String(s) => {
            format!(
                "{}\"{}\"{}",
                theme.color_info(),
                s,
                theme.color_reset()
            )
        }
        serde_json::Value::Number(n) => {
            format!(
                "{}{}{}",
                theme.color_accent(),
                n,
                theme.color_reset()
            )
        }
        serde_json::Value::Bool(b) => {
            let color = if *b {
                theme.color_success()
            } else {
                theme.color_error()
            };
            format!("{}{}{}", color, b, theme.color_reset())
        }
        serde_json::Value::Null => {
            format!("{}null{}", theme.color_warning(), theme.color_reset())
        }
    }
}

/// Formats the `check` command output
pub fn format_dependency_check(
    yt_dlp: Option<&str>,
    ffmpeg: Option<&str>,
    theme: &Arc<dyn CliTheme>,
) -> String {
    let mut output = format!(
        "\n{}🔍 System Dependencies{}:\n\n",
        theme.color_info(),
        theme.color_reset()
    );

    let check_yt_dlp = match yt_dlp {
        Some(path) => format!(
            "{}✓{} yt-dlp: {}",
            theme.color_success(),
            theme.color_reset(),
            path
        ),
        None => format!(
            "{}✗{} yt-dlp: NOT FOUND (will auto-install)",
            theme.color_error(),
            theme.color_reset()
        ),
    };

    let check_ffmpeg = match ffmpeg {
        Some(path) => format!(
            "{}✓{} FFmpeg: {}",
            theme.color_success(),
            theme.color_reset(),
            path
        ),
        None => format!(
            "{}✗{} FFmpeg: NOT FOUND (will auto-install)",
            theme.color_error(),
            theme.color_reset()
        ),
    };

    output.push_str(&check_yt_dlp);
    output.push('\n');
    output.push_str(&check_ffmpeg);
    output.push_str("\n\n");

    output
}

/// Formats the `clean` command confirmation + summary
pub fn format_clean_summary(
    total_removed: usize,
    bytes_freed: Option<u64>,
    theme: &Arc<dyn CliTheme>,
) -> String {
    let size_str = bytes_freed
        .map(|b| format_bytes(b))
        .unwrap_or_else(|| "unknown".to_string());

    format!(
        "{}✓ Cleaned {}{}. Freed: {} of disk space.{}\n",
        theme.color_success(),
        total_removed,
        if total_removed == 1 {
            " download"
        } else {
            " downloads"
        },
        size_str,
        theme.color_reset()
    )
}

/// Formats batch download summary
pub fn format_batch_summary(
    total: usize,
    queued: usize,
    failed: usize,
    theme: &Arc<dyn CliTheme>,
) -> String {
    let mut output = format!("\n{}📊 Batch Download Summary{}:\n", theme.color_info(), theme.color_reset());
    output.push_str(&format!("  Total URLs: {}\n", total));
    output.push_str(&format!(
        "  {}Queued: {}{}  ",
        theme.color_accent(),
        queued,
        theme.color_reset()
    ));
    if failed > 0 {
        output.push_str(&format!(
            "{}Failed: {}{}",
            theme.color_error(),
            failed,
            theme.color_reset()
        ));
    } else {
        output.push_str("No failures!");
    }
    output.push('\n');

    output
}

/// Helper to format bytes
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
    }
}
