Repository agent notes — high-signal, repo-specific guidance.

Keep this short. Only include things an automated agent would likely miss.

Quick facts
- Workspace: Cargo workspace at root. default-members = ["mangofetch-cli"] so `cargo run` from the repository root runs the CLI by default.
- Entry binaries: CLI binary name is `mangofetch` (package `mangofetch-cli`). GUI binary is `mangofetch-gui`.

How to run (exact commands)
- Run the CLI from the repo root (use this to iterate quickly):
  - `cargo run`  (runs the workspace default member -> mangofetch-cli)
  - or explicitly: `cargo run -p mangofetch-cli` or `cargo run --bin mangofetch`
- Build a release CLI binary:
  - `cargo build --release -p mangofetch-cli`
- Build the GUI (may require system deps on Linux/macOS):
  - `cargo build --release -p mangofetch-gui`

Testing & linting
- Run all tests across workspace: `cargo test`
- Run only core tests: `cargo test -p mangofetch-core`
- Run a single integration test binary (example):
  - `cargo test -p mangofetch-core --test queue_tests <test_name>`
- Lint/format: `cargo clippy` and `cargo fmt` (run from repo root or target package with `-p`)

Packaging / install notes
- The crate published to crates.io for the CLI is `mangofetch-cli`. Use that name for `cargo install`:
  - `cargo install mangofetch-cli`
  - (The binary produced is named `mangofetch`.)
- Releases are created by tagging `vMAJOR.MINOR.PATCH`. CI (/.github/workflows/release.yml) triggers on tags `v*.*.*` and builds multiple platform targets (some use cross).

Important repo-specific gotchas
- External tools: the engine auto-downloads runtimes like `yt-dlp` and `ffmpeg` into the app data dir on first run. Running the app or some tests may trigger network downloads.
- Env-vars: code intentionally removes `PYTHONHOME` and `PYTHONPATH` during setup to avoid interfering with bundled `yt-dlp`. Do not assume a Python env will be used by yt-dlp inside the runtime.
- Portable mode: presence of `portable.txt` or `.portable` next to the executable redirects app data to a local `data` folder. Useful for CI or ephemeral runs.
- CWD/workdir: because the workspace Cargo.toml defines default-members, many commands are intended to be run from the repo root. When using tooling or scripts, set the working directory explicitly to the repo root for cargo invocations.

Note: For CI and reproducible tests the repository now pins Rust via `rust-toolchain.toml` and CI test jobs set `MANGOFETCH_OFFLINE=1` to disable runtime auto-downloads. Use the same env var locally to run tests without network activity.
CI and cross-build notes agents should preserve
- The release workflow builds many targets and sometimes uses `cross` for platforms that require it. When reproducing CI locally for those targets, install `cross` rather than assuming native toolchains will work.
- On Ubuntu build runners the workflow installs several GTK/WebKit/etc system packages before building the GUI; building the GUI on Linux may need those dev packages.
- The release job updates installer manifests (scoop/winget) and commits them back to main — CI may attempt to push changes during release; expect that behaviour in the workflow.

Where to look next (source of truth)
- workspace manifest: Cargo.toml (root)
- CLI manifest: mangofetch-cli/Cargo.toml
- Core engine manifest and tests: mangofetch-core/Cargo.toml and mangofetch-core/tests/
- Release CI: .github/workflows/release.yml
- Developer-facing docs: README.md and docs/ (wiki pages)

If something is unclear
- Ask one short question. Prefer confirming package names or whether you should run networked tests. Avoid changing CI or release workflows without explicit instruction.
