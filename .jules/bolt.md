## 2024-05-20 - [Avoid compiling regex in loop or hot path]
**Learning:** In mangofetch-core instagram scraper, dynamic regexes were being instantiated using format strings and `Regex::new` during HTML parsing, creating a significant unnecessary overhead for identical static keys.
**Action:** Replace `Regex::new` with pre-compiled `LazyLock<Regex>` statically. If the pattern must be dynamic, strongly consider pure string searching algorithms (`.find()`, `.split()`) or refactoring to use capture groups instead of fully dynamic patterns.

## 2024-05-24 - Async File Creation
**Learning:** Using `std::fs::File::create` inside an async function blocks the async executor's worker thread, leading to potential task starvation. Replacing it with `tokio::fs::File::create` offloads the operation to the blocking thread pool.
**Action:** Always prefer `tokio::fs` over `std::fs` inside async functions for file creation, reading, and writing to avoid stalling the Tokio runtime, unless explicitly offloaded using `spawn_blocking`.
