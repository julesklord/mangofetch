pub fn app_data_dir() -> Option<std::path::PathBuf> {
    if let Ok(dir) = std::env::var("MANGOFETCH_DATA_DIR") {
        return Some(std::path::PathBuf::from(dir));
    }

    let base = dirs::data_dir()?;
    let new_path = base.join("mangofetch");
    let old_path = base.join("omniget");

    if old_path.exists() {
        let _ = std::fs::create_dir_all(&new_path);

        for dir_name in &["bin", "plugins"] {
            let src = old_path.join(dir_name);
            let dst = new_path.join(dir_name);
            if src.exists() && !dst.exists() {
                let _ = copy_dir_recursive(&src, &dst);
            }
        }

        if let Ok(entries) = std::fs::read_dir(&old_path) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                    let dest = new_path.join(entry.file_name());
                    if !dest.exists() {
                        let _ = std::fs::copy(entry.path(), &dest);
                    }
                }
            }
        }
    }

    Some(new_path)
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if src.is_dir() {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let dest = dst.join(entry.file_name());
            copy_dir_recursive(&entry.path(), &dest)?;
        }
    } else {
        std::fs::copy(src, dst)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_app_data_dir_env_var() {
        let _guard = ENV_MUTEX.lock().unwrap();

        let old_val = env::var("MANGOFETCH_DATA_DIR");

        let mock_path = if cfg!(windows) {
            "C:\\mock\\mangofetch\\data"
        } else {
            "/mock/mangofetch/data"
        };
        env::set_var("MANGOFETCH_DATA_DIR", mock_path);

        let path = app_data_dir().expect("Should return some path");

        assert_eq!(path, std::path::PathBuf::from(mock_path));

        match old_val {
            Ok(val) => env::set_var("MANGOFETCH_DATA_DIR", val),
            Err(_) => env::remove_var("MANGOFETCH_DATA_DIR"),
        }
    }

    #[test]
    fn test_app_data_dir_no_env_var() {
        let _guard = ENV_MUTEX.lock().unwrap();

        let old_val = env::var("MANGOFETCH_DATA_DIR");

        env::remove_var("MANGOFETCH_DATA_DIR");

        if let Some(path) = app_data_dir() {
            let path_str = path.to_string_lossy();
            assert!(
                path_str.ends_with("mangofetch") || path_str.ends_with("mangofetch/"),
                "Path should end with mangofetch, got: {}",
                path_str
            );
        }

        match old_val {
            Ok(val) => env::set_var("MANGOFETCH_DATA_DIR", val),
            Err(_) => env::remove_var("MANGOFETCH_DATA_DIR"),
        }
    }
}
