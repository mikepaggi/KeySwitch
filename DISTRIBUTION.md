# KeySwitch Distribution Strategy

This document outlines the complete distribution strategy for KeySwitch as an open source project.

## Quick Answer: Code Signing

**For an open source project like KeySwitch:**

### Phase 1: Start Without Signing ✅ (Recommended Now)

**Why:**
- Code signing costs $400+/year (macOS $99 + Windows $300)
- Many successful OSS projects don't sign initially
- Users understand security warnings for OSS
- Users can verify via checksums or build from source

**What to do:**
1. Build installers (already set up ✅)
2. Provide checksums for verification
3. Document security warnings in installation guide (done ✅)
4. Let users know it's safe despite warnings

**Cost: $0/year**

### Phase 2: Sign When Project Grows (Later)

**When to consider:**
- 1000+ downloads/month
- Users requesting signed builds
- Project has funding/sponsorship

**What to do:**
1. Get Apple Developer account ($99/year)
2. Apply for free SignPath.io OSS signing (Windows)
3. Set up automated GitHub Actions signing

**Cost: $99/year** (macOS only, Windows free via SignPath)

### Phase 3: Full Professional Signing (If Popular)

**When:**
- Enterprise users
- High download volume
- Professional reputation important

**What to do:**
1. Purchase Windows code signing certificate ($300/year)
2. Full CI/CD signing pipeline
3. Automatic notarization

**Cost: $400/year**

---

## Current Setup (✅ Complete)

You now have a complete distribution system:

### macOS
- ✅ Simple installer script (`build-mac-installer-simple.sh`)
- ✅ Universal binary script (`build-mac-installer.sh`)
- ✅ User-friendly .pkg installer
- ✅ Auto-start configuration
- ✅ Welcome/conclusion screens
- ✅ Entitlements file (ready for signing)

### Windows
- ✅ Simple ZIP package installer (`build-windows-simple.ps1`)
- ✅ Professional NSIS installer (`build-windows-installer.ps1`)
- ✅ Batch install/uninstall scripts
- ✅ Auto-start configuration

### Documentation
- ✅ Installation guide with security warning explanations
- ✅ Code signing guide (for future)
- ✅ Build instructions
- ✅ Troubleshooting guides
- ✅ Release creation script

---

## Recommended GitHub Release Process

### 1. Build Release Packages

**On macOS:**
```bash
./scripts/create-release.sh
```

**On Windows:**
```powershell
.\scripts\build-windows-simple.ps1
# Copy dist\KeySwitch-*-Windows.zip to your Mac's dist/ folder
```

### 2. Create Git Tag

```bash
VERSION="0.1.0"
git tag -a "v$VERSION" -m "Release v$VERSION"
git push origin "v$VERSION"
```

### 3. Create GitHub Release

1. Go to: https://github.com/YOUR_USERNAME/keyswitch/releases/new
2. Select the tag you just pushed
3. Title: `v0.1.0 - Initial Release`
4. Description:

```markdown
## KeySwitch v0.1.0

Background daemon that automatically sets Keychron keyboards to the correct layout (Mac/Windows) when connected.

### Features
- Automatic keyboard detection
- Sets Mac/Windows mode on connect
- Runs silently in background
- Auto-starts on login

### Installation

#### macOS
Download `KeySwitch-0.1.0-arm64.pkg` (Apple Silicon) or `KeySwitch-0.1.0-x86_64.pkg` (Intel)

**Important:** You will see a security warning because this package is unsigned. See [Installation Guide](INSTALLATION_GUIDE.md) for details.

1. Right-click the .pkg file → Open
2. Click "Open" again to bypass security warning
3. Follow installer prompts

#### Windows
Download `KeySwitch-0.1.0-Windows.zip`

1. Extract the ZIP file
2. Right-click `install.bat` → Run as administrator
3. Click "More info" → "Run anyway" if warned

### Verification

Compare SHA-256 checksums with `checksums.txt`:

```bash
# macOS/Linux
shasum -a 256 -c checksums.txt

# Windows
certutil -hashfile KeySwitch-0.1.0-Windows.zip SHA256
```

### Documentation
- [Installation Guide](INSTALLATION_GUIDE.md)
- [Building from Source](BUILD_INSTALLERS.md)

### Security Note

These packages are currently **unsigned** due to the cost of code signing certificates ($400+/year). This is common for open source projects.

**This is safe** - you can verify by:
- Checking the source code (it's open source!)
- Comparing checksums
- Building from source yourself

As the project grows, we may add code signing for a smoother installation experience.

### What's Next
- Code signing (if funding available)
- Linux support
- Configuration file support
- GUI settings panel
```

5. Drag and drop files from `dist/`:
   - KeySwitch-0.1.0-arm64.pkg
   - KeySwitch-0.1.0-x86_64.pkg (if built)
   - KeySwitch-0.1.0-Windows.zip
   - checksums.txt

6. Click "Publish release"

---

## Example README Section

Add this to your main README.md:

```markdown
## Installation

### Download

Get the latest release from the [releases page](https://github.com/YOUR_USERNAME/keyswitch/releases).

### Quick Install

**macOS:**
1. Download the .pkg file for your Mac (arm64 or x86_64)
2. Right-click → Open (to bypass security warning)
3. Follow installer

**Windows:**
1. Download and extract the .zip file
2. Right-click install.bat → Run as administrator
3. Click "Run anyway" if warned

**Security Note:** Installers are currently unsigned (common for OSS). See [Installation Guide](INSTALLATION_GUIDE.md).

### Building from Source

Prefer to build yourself?

```bash
git clone https://github.com/YOUR_USERNAME/keyswitch.git
cd keyswitch
cargo build --release
```

See [BUILD_INSTALLERS.md](BUILD_INSTALLERS.md) for details.
```

---

## Future: Automated Releases with GitHub Actions

When you're ready to automate, you can use GitHub Actions to build releases automatically when you push a tag.

**Basic workflow** (no signing):

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --release
      - name: Create installer
        run: ./scripts/build-mac-installer-simple.sh
      - uses: actions/upload-artifact@v3
        with:
          name: macos-installer
          path: dist/*.pkg

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --release
      - name: Create package
        run: .\scripts\build-windows-simple.ps1
      - uses: actions/upload-artifact@v3
        with:
          name: windows-installer
          path: dist/*.zip

  release:
    needs: [build-macos, build-windows]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            macos-installer/*
            windows-installer/*
          generate_release_notes: true
```

Then releasing is as simple as:
```bash
git tag v0.2.0
git push origin v0.2.0
# GitHub automatically builds and creates release!
```

---

## Cost Comparison

| Approach | macOS | Windows | Total/Year | User Experience |
|----------|-------|---------|------------|-----------------|
| **No signing** (now) | $0 | $0 | $0 | Security warnings, workarounds needed |
| **Basic signing** | $99 | Free (SignPath) | $99 | macOS: smooth, Windows: warnings |
| **Full signing** | $99 | $300 | $399 | Both platforms smooth |
| **Enterprise** | $299 | $500 | $799 | Instant trust, auto-updates |

---

## Recommendations

### For Now (v0.1 - v0.5)
1. ✅ **Don't sign** - focus on features, not infrastructure costs
2. ✅ Provide excellent installation docs (done!)
3. ✅ Make checksums available (script created!)
4. ✅ Encourage building from source for security-conscious users

### When You Hit 1,000 Downloads
1. 💰 Get Apple Developer account ($99/year)
2. 🆓 Apply for SignPath.io free OSS signing
3. ⚙️ Set up GitHub Actions automated signing
4. 📝 Update installation guide (warnings will be reduced)

### When You Hit 10,000 Downloads
1. 💰 Consider Windows code signing certificate
2. 🏢 Look into GitHub Sponsors to cover costs
3. 🎯 Full professional signing pipeline

---

## Support & Transparency

Be transparent with users about why packages are unsigned:

**Good message to users:**
> KeySwitch is an open source project maintained by volunteers. Code signing certificates cost $400+/year, which is not currently feasible. As the project grows and if funding becomes available through sponsorship, we'll add code signing for a smoother installation experience. In the meantime, you can verify the packages via checksums or build from source yourself!

This shows:
- You understand the issue
- You have a plan
- You respect user security concerns
- You're transparent about costs

---

## Questions?

- See [CODE_SIGNING.md](CODE_SIGNING.md) for technical details
- See [INSTALLATION_GUIDE.md](INSTALLATION_GUIDE.md) for user-facing docs
- See [BUILD_INSTALLERS.md](BUILD_INSTALLERS.md) for build instructions
