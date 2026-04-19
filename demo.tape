Output demo.gif

Set Shell "pwsh"
Set FontSize 16
Set Width 1200
Set Height 800
Set Padding 20
Set Theme "Catppuccin Macchiato"

# Setup: Clear any noise
Hide
Type "clear" Enter
Show

# --- Version Check Demo ---
Type "# Checking the new versioning system" Enter
Sleep 1s
Type "./target/release/omniget-cli.exe --version" Enter
Sleep 3s

# --- Media Info Demo ---
Type "# 1. Inspecting media information with the new UI" Enter
Sleep 1s
Type "./target/release/omniget-cli.exe info 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'" Enter
Sleep 5s

# --- Config List Demo ---
Type "# 2. Pretty-printed configuration with colors" Enter
Sleep 1s
Type "./target/release/omniget-cli.exe config list" Enter
Sleep 5s

# --- Queue List Demo ---
# Using debug for list since we want to see items if release queue is empty
Type "# 3. Formatted queue with status icons and platform colors" Enter
Sleep 1s
Type "./target/release/omniget-cli.exe list" Enter
Sleep 5s

# --- Dependency Check Demo ---
Type "# 4. Clean system dependency verification" Enter
Sleep 1s
Type "./target/release/omniget-cli.exe check" Enter
Sleep 5s

# --- Download Demo ---
Type "# 5. Beautiful progress bars with ETA and speed" Enter
Sleep 1s
Type "./target/release/omniget-cli.exe download 'https://www.youtube.com/watch?v=dQw4w9WgXcQ' -q 720p" Enter
Sleep 10s

Type "# Done! v0.10.2-release is looking sharp. 🚀" Enter
Sleep 3s
