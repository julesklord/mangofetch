Output docs/assets/demo.gif

# TUI without borders/margins
Set Margin 2
Set WindowBar ""
Set Padding 2

Set Width 1100
Set Height 700

# Tropical warm theme
Set Theme "GruvboxDark"
Set FontSize 14
Set FontFamily "FiraCode Nerd Font"
Set LineHeight 1.1

Set TypingSpeed 50ms

# Launch the compiled binary into TUI mode
Type "mangofetch tui"
Sleep 1000ms
Enter

# Wait for TUI to render
Sleep 3s

# Open Add Modal
Type "a"
Sleep 1s

# Type URL
Type "https://youtube.com/playlist?list=PLz2d8D0PSa_hkFsgQPkJMoLbPbLo3zBbi"
Sleep 1s
Enter

# Wait for the preview to be fetched and rendered (Playlists take longer)
Sleep 15s

# Confirm the download
Enter

# Let the downloads run for a bit
Sleep 10s

# Navigate tabs
Tab
Sleep 2s
Tab
Sleep 2s
Tab
Sleep 5s

# Quit
Type "q"
Sleep 1s
