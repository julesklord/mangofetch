fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let settings = mangofetch_core::models::settings::AppSettings::load_from_disk();

    let rows: Vec<Row> = SettingKind::ALL
        .iter()
        .enumerate()
        .map(|(i, kind)| {
            let value = get_setting_value(kind, &settings, app);
            let is_sel = i == app.settings_index;

            let key_cell = Cell::from(kind.label()).style(if is_sel {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.secondary)
            });

            let val_cell = Cell::from(format!(" [ {} ] ", value)).style(if is_sel {
                Style::new().fg(t.text).bold().bg(t.highlight)
            } else {
                Style::new().fg(t.text)
            });

            let hint_cell =
                Cell::from(kind.description()).style(Style::new().fg(t.text_dim).italic());

            Row::new([key_cell, val_cell, hint_cell]).height(1)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(25),
            Constraint::Length(22),
            Constraint::Min(40),
        ],
    )
    .header(
        Row::new([
            Cell::from("  Option"),
            Cell::from(" Current Value"),
            Cell::from(" Description"),
        ])
        .style(Style::new().fg(t.secondary).bold())
        .bottom_margin(1),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" ⚙  Configuration & Appearance ")
            .border_style(Style::new().fg(t.surface)),
    );

    f.render_widget(table, area);
}

fn get_setting_value(kind: &SettingKind, s: &AppSettings, app: &App) -> String {
    match kind {
        SettingKind::TuiTheme => s.appearance.tui_theme.clone(),
        SettingKind::UseNerdFonts => {
            if app.use_nerd_fonts {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::MaxDownloads => s.advanced.max_concurrent_downloads.to_string(),
        SettingKind::VideoQuality => s.download.video_quality.clone(),
        SettingKind::OrganizeByPlatform => {
            if s.download.organize_by_platform {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::SkipExisting => {
            if s.download.skip_existing {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::DownloadSubtitles => {
            if s.download.download_subtitles {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::DownloadAttachments => {
            if s.download.download_attachments {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::DownloadDescriptions => {
            if s.download.download_descriptions {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::SponsorBlock => {
            if s.download.youtube_sponsorblock {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::SplitByChapters => {
            if s.download.split_by_chapters {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::EmbedMetadata => {
            if s.download.embed_metadata {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::EmbedThumbnail => {
            if s.download.embed_thumbnail {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::MaxConcurrentSegments => s.advanced.max_concurrent_segments.to_string(),
        SettingKind::MaxRetries => s.advanced.max_retries.to_string(),
        SettingKind::ConcurrentFragments => s.advanced.concurrent_fragments.to_string(),
        SettingKind::StaggerDelay => format!("{}ms", s.advanced.stagger_delay_ms),
        SettingKind::ClipboardDetection => {
            if s.download.clipboard_detection {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::ProxyEnabled => {
            if s.proxy.enabled {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
        SettingKind::PortableMode => {
            if s.portable_mode {
                "ON".into()
            } else {
                "OFF".into()
            }
        }
    }
}
