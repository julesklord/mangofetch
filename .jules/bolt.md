## 2024-05-20 - [Avoid compiling regex in loop or hot path]
**Learning:** In mangofetch-core instagram scraper, dynamic regexes were being instantiated using format strings and `Regex::new` during HTML parsing, creating a significant unnecessary overhead for identical static keys.
**Action:** Replace `Regex::new` with pre-compiled `LazyLock<Regex>` statically. If the pattern must be dynamic, strongly consider pure string searching algorithms (`.find()`, `.split()`) or refactoring to use capture groups instead of fully dynamic patterns.
## 2024-05-30 - Replace unsafe unwrap in direct_downloader
**Learning:** Replacing `.unwrap()` with proper error handling (e.g., `anyhow::Result`) is easy to do in a `spawn` block that returns a Result. Replacing it prevents potential panics.
**Action:** Always prefer `.map_err(...)?` over `.unwrap()` in fallible code blocks that return a `Result`.
## 2026-06-28 - Async FS optimization
**Learning:** For complex synchronous filesystem traversals (like directory iteration) in a Tokio async context, wrap the entire synchronous block in `tokio::task::spawn_blocking` instead of replacing individual operations with `tokio::fs` equivalents to minimize async task overhead and executor stalling.
**Action:** Use `spawn_blocking` to wrap whole synchronous routines rather than paying the per-operation penalty of async wrappers when iterating over many items.
