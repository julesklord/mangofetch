fn render_settings(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let settings = &app.settings;

    let rows: Vec<Row> = SettingKind::ALL
        .iter()
        .enumerate()
        .map(|(i, kind)| {
            let value = get_setting_value(kind, &settings, app);
            let is_sel = i == app.settings_index;

            let prefix = if is_sel { "▶ " } else { "  " };
            let key_cell = Cell::from(format!("{}{}", prefix, kind.label())).style(if is_sel {
                Style::new().fg(t.accent).bold()
            } else {
                Style::new().fg(t.text_dim)
            });

            let val_cell = Cell::from(format!(" ◀ {} ▶ ", value)).style(if is_sel {
                Style::new().fg(t.background).bg(t.accent).bold()
            } else {
                Style::new().fg(t.secondary)
            });

            let hint_cell =
                Cell::from(kind.description()).style(Style::new().fg(t.text_dim).italic());

            Row::new([key_cell, val_cell, hint_cell]).height(1)
        })
        .collect();

    let title = if app.use_nerd_fonts {
        " 󰒓 Configuration & Appearance "
    } else {
        " Configuration & Appearance "
    };

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
            .title(title)
            .border_style(Style::new().fg(t.surface)),
    );

    f.render_widget(table, area);
}

fn get_setting_value(kind: &SettingKind, s: &AppSettings, app: &App) -> String {
    let get_statusbar_val = |name: &str| {
        if let Some(pos) = app.statusbar_modules.iter().position(|m| m == name) {
            format!("ACTIVE (Pos {})", pos + 1)
        } else {
            "DISABLED".to_string()
        }
    };

    let on_off = |b: bool| if b { "ON".to_string() } else { "OFF".to_string() };

    match kind {
        SettingKind::TuiTheme => s.appearance.tui_theme.clone(),
        SettingKind::UseNerdFonts => on_off(app.use_nerd_fonts),
        SettingKind::EnableAnimations => on_off(app.enable_animations),
        SettingKind::NavigationLayout => app.layout.to_uppercase(),
        SettingKind::MaxDownloads => s.advanced.max_concurrent_downloads.to_string(),
        SettingKind::VideoQuality => s.download.video_quality.clone(),
        SettingKind::AlwaysAskConfirm => on_off(s.download.always_ask_confirm),
        SettingKind::OrganizeByPlatform => on_off(s.download.organize_by_platform),
        SettingKind::SkipExisting => on_off(s.download.skip_existing),
        SettingKind::DownloadSubtitles => on_off(s.download.download_subtitles),
        SettingKind::DownloadAttachments => on_off(s.download.download_attachments),
        SettingKind::DownloadDescriptions => on_off(s.download.download_descriptions),
        SettingKind::SponsorBlock => on_off(s.download.youtube_sponsorblock),
        SettingKind::SplitByChapters => on_off(s.download.split_by_chapters),
        SettingKind::EmbedMetadata => on_off(s.download.embed_metadata),
        SettingKind::EmbedThumbnail => on_off(s.download.embed_thumbnail),
        SettingKind::MaxConcurrentSegments => s.advanced.max_concurrent_segments.to_string(),
        SettingKind::MaxRetries => s.advanced.max_retries.to_string(),
        SettingKind::ConcurrentFragments => s.advanced.concurrent_fragments.to_string(),
        SettingKind::StaggerDelay => format!("{}ms", s.advanced.stagger_delay_ms),
        SettingKind::ClipboardDetection => on_off(s.download.clipboard_detection),
        SettingKind::ProxyEnabled => on_off(s.proxy.enabled),
        SettingKind::PortableMode => on_off(s.portable_mode),
        SettingKind::StatusbarMode => get_statusbar_val("mode"),
        SettingKind::StatusbarTab => get_statusbar_val("tab"),
        SettingKind::StatusbarRadar => get_statusbar_val("radar"),
        SettingKind::StatusbarCpu => get_statusbar_val("cpu"),
        SettingKind::StatusbarRam => get_statusbar_val("ram"),
        SettingKind::StatusbarSpeed => get_statusbar_val("speed"),
        SettingKind::StatusbarQueue => get_statusbar_val("queue"),
        SettingKind::StatusbarTime => get_statusbar_val("time"),
    }
}
