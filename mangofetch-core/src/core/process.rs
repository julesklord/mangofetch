fn enhanced_path() -> Option<String> {
    use std::sync::OnceLock;
    static CACHED: OnceLock<Option<String>> = OnceLock::new();
    CACHED
        .get_or_init(|| {
            let bin_dir = crate::core::paths::app_data_dir()?.join("bin");
            let sep = if cfg!(windows) { ";" } else { ":" };
            let current = std::env::var("PATH").unwrap_or_default();

            let extra_dirs: Vec<String> = vec![
                Some(bin_dir.display().to_string()),
                #[cfg(target_os = "macos")]
                Some("/opt/homebrew/bin".into()),
                #[cfg(target_os = "macos")]
                Some("/usr/local/bin".into()),
                #[cfg(target_os = "linux")]
                dirs::home_dir().map(|h| h.join(".local").join("bin").display().to_string()),
                #[cfg(target_os = "linux")]
                Some("/usr/local/bin".into()),
            ]
            .into_iter()
            .flatten()
            .collect();

            Some(format!("{}{}{}", extra_dirs.join(sep), sep, current))
        })
        .clone()
}

pub fn command<S: AsRef<std::ffi::OsStr>>(program: S) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(program);
    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);
    if let Some(path) = enhanced_path() {
        cmd.env("PATH", path);
    }
    cmd.env_remove("PYTHONHOME");
    cmd.env_remove("PYTHONPATH");
    cmd.env("PYTHONIOENCODING", "utf-8");
    cmd.env("PYTHONUTF8", "1");
    cmd
}

pub fn std_command<S: AsRef<std::ffi::OsStr>>(program: S) -> std::process::Command {
    let mut cmd = std::process::Command::new(program);
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    if let Some(path) = enhanced_path() {
        cmd.env("PATH", path);
    }
    cmd.env_remove("PYTHONHOME");
    cmd.env_remove("PYTHONPATH");
    cmd.env("PYTHONIOENCODING", "utf-8");
    cmd.env("PYTHONUTF8", "1");
    cmd
}
