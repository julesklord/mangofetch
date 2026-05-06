# MangoFetch Demo Tape
# Run with: Get-Content .scratch/demo.tape | vhs

Set Shell powershell
Set FontSize 12
Set Width 1200
Set Height 700
Set Padding 40
Set Theme "Dark Pastel"
Set TypingSpeed 50ms
Set Framerate 60

# Start the show
Type "./target/release/mangofetch check"
Sleep 500ms
Enter
Sleep 7s

# Clear for the main event
Type "clear"
Enter
Sleep 1s

# Get Info
Type "./target/release/mangofetch info https://www.youtube.com/watch?v=dQw4w9WgXcQ"
Sleep 1s
Enter
Sleep 5s

# Download (Simulated, we stop it or let it run a bit)
Type "./target/release/mangofetch download https://www.youtube.com/watch?v=dQw4w9WgXcQ"
Sleep 1s
Enter
Sleep 20s

# Finish with a flourish
Type "./target/release/mangofetch --help"
Sleep 1s
Enter
Sleep 6s

Output demo.gif
