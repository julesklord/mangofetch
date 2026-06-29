## 2024-05-20 - [Avoid compiling regex in loop or hot path]
**Learning:** In mangofetch-core instagram scraper, dynamic regexes were being instantiated using format strings and `Regex::new` during HTML parsing, creating a significant unnecessary overhead for identical static keys.
**Action:** Replace `Regex::new` with pre-compiled `LazyLock<Regex>` statically. If the pattern must be dynamic, strongly consider pure string searching algorithms (`.find()`, `.split()`) or refactoring to use capture groups instead of fully dynamic patterns.
## 2024-05-30 - Replace unsafe unwrap in direct_downloader
**Learning:** Replacing `.unwrap()` with proper error handling (e.g., `anyhow::Result`) is easy to do in a `spawn` block that returns a Result. Replacing it prevents potential panics.
**Action:** Always prefer `.map_err(...)?` over `.unwrap()` in fallible code blocks that return a `Result`.
## 2024-05-19 - Optimize `find_downloaded_file` in `ytdlp.rs`
**Learning:** Avoid multiple passes when reading directories (like `std::fs::read_dir`) when the logic can be combined into a single loop.
**Action:** When filtering through files looking for a specific type of file or trying to find a fallback match, check if it's possible to track all necessary state in a single iteration over the directory entries instead of calling `read_dir` twice.
## 2024-05-18 - Avoid Blocking Tokio Executor with Synchronous Filesystem Traversals
**Learning:** `std::fs::read_dir` and iterating over files synchronously blocks the Tokio executor thread. When performing complex or unbounded directory iterations inside an async function like `cleanup_part_files`, this blocking behavior can significantly impact overall async runtime performance.
**Action:** Wrap the entire synchronous block inside `tokio::task::spawn_blocking` and await its result. This offloads the blocking operations to a separate blocking thread pool, allowing the async executor to continue scheduling other tasks without interruption.
