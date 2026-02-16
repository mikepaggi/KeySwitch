# KeySwitch Installers

This document explains how to build installers for KeySwitch that non-technical users can use.

## macOS Installer

### Quick Start (Current Architecture Only)

The simplest way to create a macOS installer:

```bash
chmod +x scripts/build-mac-installer-simple.sh
./scripts/build-mac-installer-simple.sh
```

This creates a `.pkg` file in the `dist/` directory that users can double-click to install.

**Output:** `dist/KeySwitch-0.1.0-arm64.pkg` (or `x86_64` on Intel Macs)

### Universal Binary (Intel + Apple Silicon)

For maximum compatibility, create a universal binary:

```bash
# Install required targets first
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build universal installer
chmod +x scripts/build-mac-installer.sh
./scripts/build-mac-installer.sh
```

**Output:** `dist/KeySwitch-0.1.0.pkg`

### What the Installer Does

- Installs `keyswitch` binary to `/usr/local/bin/`
- Installs LaunchAgent plist to `~/Library/LaunchAgents/`
- Automatically starts the daemon
- Configures auto-start on login

### For Users

1. Download the `.pkg` file
2. Double-click to install
3. Enter your password when prompted
4. Done! The daemon starts automatically

**No technical knowledge required.**

## Windows Installer

### Option 1: Simple ZIP Package (Recommended for Quick Distribution)

No dependencies required:

```powershell
.\scripts\build-windows-simple.ps1
```

**Output:** `dist/KeySwitch-0.1.0-Windows.zip`

**For users:**
1. Extract the ZIP file
2. Right-click `install.bat`
3. Select "Run as administrator"
4. Done!

### Option 2: Professional EXE Installer (Requires NSIS)

For a more polished installer experience:

1. Install NSIS from https://nsis.sourceforge.io/Download
2. Run:
   ```powershell
   .\scripts\build-windows-installer.ps1
   ```

**Output:** `dist/KeySwitch-0.1.0-Setup.exe`

**For users:**
1. Double-click the `.exe` file
2. Follow the installation wizard
3. Done!

### What the Installer Does

- Installs `keyswitch.exe` to `C:\Program Files\KeySwitch\`
- Creates startup shortcut for auto-start
- Starts the daemon immediately
- Creates uninstaller

## Testing Your Installers

### macOS

```bash
# Build installer
./scripts/build-mac-installer-simple.sh

# Install it
sudo installer -pkg dist/KeySwitch-*.pkg -target /

# Check if it's running
launchctl list | grep keyswitch

# View logs
tail -f /tmp/keyswitch.log

# Uninstall (if needed)
launchctl unload ~/Library/LaunchAgents/com.keyswitch.daemon.plist
sudo rm /usr/local/bin/keyswitch
rm ~/Library/LaunchAgents/com.keyswitch.daemon.plist
```

### Windows

```powershell
# Build package
.\scripts\build-windows-simple.ps1

# Extract and test
Expand-Archive dist\KeySwitch-*-Windows.zip -DestinationPath test-install

# Install (as admin)
cd test-install\KeySwitch
.\install.bat

# Check if running
tasklist | findstr keyswitch

# View logs
Get-Content $env:TEMP\keyswitch.log -Wait

# Uninstall
.\uninstall.bat
```

## Distribution Checklist

Before distributing installers:

- [ ] Test on clean macOS system (Intel and Apple Silicon if possible)
- [ ] Test on clean Windows system
- [ ] Verify daemon starts automatically after reboot
- [ ] Verify keyboard switching works correctly
- [ ] Test uninstallation process
- [ ] Update version number in `Cargo.toml`
- [ ] Create release notes
- [ ] Sign macOS installer (for public distribution)
- [ ] Sign Windows installer (for public distribution)

## Code Signing (For Public Distribution)

### macOS

For distribution outside of beta testing, you should sign the package:

```bash
# Sign the binary
codesign --sign "Developer ID Application: Your Name" \
  build/mac/root/usr/local/bin/keyswitch

# Sign the package
productsign --sign "Developer ID Installer: Your Name" \
  dist/KeySwitch-0.1.0.pkg \
  dist/KeySwitch-0.1.0-signed.pkg
```

### Windows

For public distribution, sign the executable:

```powershell
# Using signtool (requires code signing certificate)
signtool sign /f your-certificate.pfx /p password /tr http://timestamp.digicert.com dist/KeySwitch-0.1.0-Setup.exe
```

## Troubleshooting

### macOS: "Permission Denied" errors
Make sure scripts are executable: `chmod +x scripts/*.sh`

### macOS: Cross-compilation fails
Install required targets: `rustup target add x86_64-apple-darwin aarch64-apple-darwin`

### Windows: NSIS not found
Either install NSIS or use the simple ZIP installer (`build-windows-simple.ps1`)

### Daemon doesn't start
Check logs:
- macOS: `/tmp/keyswitch.log`
- Windows: `%TEMP%\keyswitch.log`

## Support

For issues or questions, file an issue on GitHub.
