## 2024-05-20 - [Avoid compiling regex in loop or hot path]
**Learning:** In mangofetch-core instagram scraper, dynamic regexes were being instantiated using format strings and `Regex::new` during HTML parsing, creating a significant unnecessary overhead for identical static keys.
**Action:** Replace `Regex::new` with pre-compiled `LazyLock<Regex>` statically. If the pattern must be dynamic, strongly consider pure string searching algorithms (`.find()`, `.split()`) or refactoring to use capture groups instead of fully dynamic patterns.
## 2025-03-01 - Blocking Directory Creation in Async Context
**Learning:** Using `std::fs::create_dir_all` inside an asynchronous function blocks the `tokio` worker thread, which can cause latency spikes or thread starvation.
**Action:** Use `tokio::fs::create_dir_all` (with `.await`) in async contexts to prevent blocking the executor.
