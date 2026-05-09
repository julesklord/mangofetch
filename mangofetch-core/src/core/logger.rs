use std::path::PathBuf;
use tracing_appender::non_blocking;
use tracing_subscriber::{fmt, prelude::*, Registry};

pub fn init_logging(verbose: bool) {
    init_logging_ext(verbose, true);
}

pub fn init_logging_ext(verbose: bool, use_stdout: bool) {
    let log_dir = crate::core::paths::app_data_dir()
        .map(|d| d.join("logs"))
        .unwrap_or_else(|| PathBuf::from("logs"));

    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = tracing_appender::rolling::daily(&log_dir, "mangofetch.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    // Keep the guard alive as long as the program runs.
    Box::leak(Box::new(_guard));

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    if use_stdout {
        let stdout_layer = fmt::layer()
            .with_ansi(true)
            .with_target(verbose)
            .with_filter(if verbose {
                tracing_subscriber::filter::LevelFilter::DEBUG
            } else {
                tracing_subscriber::filter::LevelFilter::INFO
            });

        let subscriber = Registry::default().with(file_layer).with(stdout_layer);
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    } else {
        let subscriber = Registry::default().with(file_layer);
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}
