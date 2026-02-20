# KeySwitch Installation Guide

A step-by-step guide for installing KeySwitch on macOS and Windows.

## macOS Installation

### Download

Download the latest `KeySwitch-X.X.X-arm64.pkg` (Apple Silicon) or `KeySwitch-X.X.X-x86_64.pkg` (Intel) from the [releases page](https://github.com/YOUR_USERNAME/keyswitch/releases).

### Installation Steps

1. **Locate the downloaded .pkg file** (usually in your Downloads folder)

2. **Open the installer**
   - Double-click the .pkg file

3. **Handle the security warning** (if the package is unsigned)

   You may see: *"KeySwitch-X.X.X.pkg" cannot be opened because it is from an unidentified developer.*

   **To proceed:**
   - Right-click (or Control+click) the .pkg file
   - Select "Open" from the menu
   - Click "Open" again in the dialog

   **Alternative (Terminal):**
   ```bash
   sudo installer -pkg ~/Downloads/KeySwitch-*.pkg -target /
   ```

4. **Follow the installer**
   - Click "Continue"
   - Accept the license (if shown)
   - Click "Install"
   - Enter your password when prompted

5. **Verify installation**
   ```bash
   # Check if daemon is running
   launchctl list | grep keyswitch

   # View logs
   tail -f /tmp/keyswitch.log
   ```

### What Gets Installed

- Binary: `/usr/local/bin/keyswitch`
- LaunchAgent: `~/Library/LaunchAgents/com.keyswitch.daemon.plist`
- Logs: `/tmp/keyswitch.log` and `/tmp/keyswitch.err.log`

### Troubleshooting

**Daemon not starting:**
```bash
# Manually load the LaunchAgent
launchctl load -w ~/Library/LaunchAgents/com.keyswitch.daemon.plist

# Check for errors
cat /tmp/keyswitch.err.log
```

**Permission issues:**
```bash
# Ensure binary is executable
chmod +x /usr/local/bin/keyswitch

# Test manually
/usr/local/bin/keyswitch
```

**Keyboard not detected:**
- Check logs: `tail -f /tmp/keyswitch.log`
- Ensure keyboard is Keychron brand
- Try unplugging and replugging the keyboard

### Uninstalling

```bash
# Stop the daemon
launchctl unload ~/Library/LaunchAgents/com.keyswitch.daemon.plist

# Remove files
sudo rm /usr/local/bin/keyswitch
rm ~/Library/LaunchAgents/com.keyswitch.daemon.plist

# Clean up logs (optional)
rm /tmp/keyswitch.log /tmp/keyswitch.err.log
```

---

## Windows Installation

### Download

Download the latest `KeySwitch-X.X.X-Windows.zip` from the [releases page](https://github.com/YOUR_USERNAME/keyswitch/releases).

### Installation Steps

1. **Extract the ZIP file**
   - Right-click the downloaded ZIP file
   - Select "Extract All..."
   - Choose a location and click "Extract"

2. **Run the installer**
   - Navigate to the extracted folder
   - Right-click `install.bat`
   - Select "Run as administrator"

3. **Handle the security warning** (if unsigned)

   You may see: *"Windows protected your PC"*

   **To proceed:**
   - Click "More info"
   - Click "Run anyway"

4. **Follow the prompts**
   - The installer will:
     - Copy files to `C:\Program Files\KeySwitch\`
     - Create a startup shortcut
     - Start the daemon

5. **Verify installation**
   ```cmd
   # Check if daemon is running
   tasklist | findstr keyswitch

   # View logs
   type %TEMP%\keyswitch.log
   ```

### What Gets Installed

- Binary: `C:\Program Files\KeySwitch\keyswitch.exe`
- Startup: `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\KeySwitch.lnk`
- Logs: `%TEMP%\keyswitch.log`

### Troubleshooting

**Daemon not starting:**
```cmd
# Check if it's running
tasklist | findstr keyswitch

# Start manually
"C:\Program Files\KeySwitch\keyswitch.exe" --daemon

# Check logs
type %TEMP%\keyswitch.log
```

**Permission denied errors:**
- Ensure you ran the installer as administrator
- Check Windows Defender/antivirus isn't blocking it

**Keyboard not detected:**
- Check logs: `type %TEMP%\keyswitch.log`
- Ensure keyboard is Keychron brand
- Try unplugging and replugging the keyboard

### Uninstalling

**Option 1: Use the uninstaller**
- Navigate to `C:\Program Files\KeySwitch\`
- Right-click `uninstall.bat`
- Select "Run as administrator"

**Option 2: Manual removal**
```cmd
# Stop the daemon
taskkill /F /IM keyswitch.exe

# Remove startup shortcut
del "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\KeySwitch.lnk"

# Remove program files (run as administrator)
rmdir /S "C:\Program Files\KeySwitch"
```

---

## Security Warnings Explained

### Why am I seeing security warnings?

KeySwitch is currently unsigned, which means:
- **macOS Gatekeeper** doesn't recognize the developer
- **Windows SmartScreen** doesn't have reputation data

This is common for open source projects due to the cost of code signing certificates:
- macOS: $99/year for Apple Developer account
- Windows: $200-400/year for code signing certificate

### Is it safe?

Yes, if you download from the official GitHub releases page. You can verify:

1. **Check the source code** (it's open source!)
2. **Verify checksums:**
   ```bash
   # macOS/Linux
   shasum -a 256 KeySwitch-*.pkg

   # Windows
   certutil -hashfile KeySwitch-*.zip SHA256
   ```
   Compare with checksums in the release notes.

3. **Build from source yourself:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/keyswitch.git
   cd keyswitch
   cargo build --release
   ```

### Will this be signed in the future?

Possibly! As the project grows and if funding becomes available, we may add code signing to provide a smoother installation experience.

---

## Building from Source

If you prefer to build from source:

### Prerequisites
- [Rust](https://rustup.rs/) installed
- macOS or Windows

### Build Steps

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/keyswitch.git
cd keyswitch

# Build release version
cargo build --release

# The binary will be in:
# macOS/Linux: target/release/keyswitch
# Windows: target\release\keyswitch.exe
```

### Manual Installation

**macOS:**
```bash
# Copy binary
sudo cp target/release/keyswitch /usr/local/bin/

# Copy LaunchAgent plist
cp com.keyswitch.daemon.plist ~/Library/LaunchAgents/

# Load LaunchAgent
launchctl load -w ~/Library/LaunchAgents/com.keyswitch.daemon.plist
```

**Windows:**
```cmd
# Copy binary (run as administrator)
copy target\release\keyswitch.exe "C:\Program Files\KeySwitch\"

# Create startup shortcut (use PowerShell as administrator)
$WshShell = New-Object -ComObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Startup\KeySwitch.lnk")
$Shortcut.TargetPath = "C:\Program Files\KeySwitch\keyswitch.exe"
$Shortcut.Arguments = "--daemon"
$Shortcut.WindowStyle = 7
$Shortcut.Save()

# Start daemon
Start-Process "C:\Program Files\KeySwitch\keyswitch.exe" -ArgumentList "--daemon" -WindowStyle Hidden
```

---

## Support

- **Issues:** https://github.com/YOUR_USERNAME/keyswitch/issues
- **Discussions:** https://github.com/YOUR_USERNAME/keyswitch/discussions

If KeySwitch isn't working:
1. Check the logs (see platform-specific sections above)
2. Search existing issues
3. Create a new issue with log output and system info
