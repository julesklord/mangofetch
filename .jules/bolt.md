## 2024-05-18 - Optimize directory traversal in find_downloaded_file
**Learning:** Checking `entry.path().is_file()` during `fs::read_dir` creates unneeded heap allocations for `PathBuf`.
**Action:** Use `entry.file_type()` which accesses filesystem metadata directly without path allocations, reducing overhead significantly inside tight loops. Also avoid intermediate path creations when inspecting filenames and extensions.
