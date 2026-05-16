## 2026-05-14 - Using `impl std::ops::Deref<Target = [u8]>` instead of explicit crate types for HTTP body chunk streams

**Learning:** When refactoring async byte streams (like the output of `reqwest::Response::bytes_stream()`), it's often better to use a generic trait bound like `impl std::ops::Deref<Target = [u8]>` (or `impl AsRef<[u8]>`) for the stream items rather than hardcoding crate-specific types like `bytes::Bytes`. This avoids needing to add new dependencies or manage complex imports across different modules, keeping the workspace dependency tree clean.

**Action:** When extracting generic byte/buffer stream logic, default to generic trait bounds for byte arrays/slices rather than specifying the exact underlying buffer type provided by the networking crate.
