1. **Add tests for `HlsDownloader`'s `with_user_agent_override`**
   - Edit `mangofetch-core/src/core/hls_downloader.rs` to include tests inside the `mod tests` block.
   - The test will verify the default user agent behavior.
   - The test will verify setting a custom user agent using `with_user_agent_override(Some("Custom UA".to_string()))`.
   - The test will verify resetting the user agent override by calling `with_user_agent_override(None)`.
2. **Complete pre commit steps**
   - Run the necessary pre commit instructions to ensure tests pass and the changes are valid.
3. **Submit the PR**
   - Create a PR with a suitable title and description.
