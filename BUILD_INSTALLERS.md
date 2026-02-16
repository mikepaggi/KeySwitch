# Quick Start: Building Installers

## macOS (you are here!)

**Simplest method** - builds for your current Mac:
```bash
./scripts/build-mac-installer-simple.sh
```

Result: `dist/KeySwitch-0.1.0-arm64.pkg` (or x86_64 on Intel)

**Universal binary** - works on both Intel and Apple Silicon:
```bash
# First time only:
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Then build:
./scripts/build-mac-installer.sh
```

Result: `dist/KeySwitch-0.1.0.pkg`

## Windows

You'll need to build these on a Windows machine with Rust installed.

**Simplest method** - no extra tools needed:
```powershell
.\scripts\build-windows-simple.ps1
```

Result: `dist/KeySwitch-0.1.0-Windows.zip`

**Professional installer** - requires NSIS:
```powershell
.\scripts\build-windows-installer.ps1
```

Result: `dist/KeySwitch-0.1.0-Setup.exe`

---

See [INSTALLER_README.md](INSTALLER_README.md) for detailed documentation.
