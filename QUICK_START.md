# Quick Start: Distribution & Code Signing

## TL;DR

**Q: Should I sign the installers?**
**A:** No, not yet. Start unsigned (it's free and common for OSS).

**Q: What does this cost?**
```
Now:     $0/year  ✅ Recommended
Later:   $99/year (when project grows)
Pro:     $400/year (if very popular)
```

**Q: How do I create a release?**
```bash
# On Mac:
./scripts/create-release.sh

# On Windows:
.\scripts\build-windows-simple.ps1

# Then create GitHub release with the files in dist/
```

---

## Files Created for You

### Build Scripts
- ✅ `scripts/build-mac-installer-simple.sh` - macOS (current arch)
- ✅ `scripts/build-mac-installer.sh` - macOS universal binary
- ✅ `scripts/build-windows-simple.ps1` - Windows ZIP package
- ✅ `scripts/build-windows-installer.ps1` - Windows EXE (needs NSIS)
- ✅ `scripts/create-release.sh` - Creates release with checksums

### Documentation
- ✅ `INSTALLATION_GUIDE.md` - For users (explains security warnings)
- ✅ `CODE_SIGNING.md` - Complete code signing guide
- ✅ `BUILD_INSTALLERS.md` - How to build installers
- ✅ `DISTRIBUTION.md` - Overall distribution strategy
- ✅ `LICENSE` - MIT License

### Supporting Files
- ✅ `entitlements.plist` - macOS entitlements (for when you sign)
- ✅ `scripts/welcome.html` - macOS installer welcome screen
- ✅ `scripts/conclusion.html` - macOS installer completion screen
- ✅ `scripts/distribution.xml` - macOS installer configuration

---

## What Users Will See (Unsigned)

### macOS
```
⚠️  "KeySwitch.pkg" cannot be opened because it is
    from an unidentified developer.

    [Cancel]  [OK]
```

**User solution:** Right-click → Open → Open again

### Windows
```
⚠️  Windows protected your PC
    Unknown publisher

    [More info]  [Don't run]
```

**User solution:** More info → Run anyway

**This is normal for unsigned OSS projects!** Your installation guide explains this clearly.

---

## Creating Your First Release

### Step 1: Build Packages

```bash
# macOS
./scripts/create-release.sh

# Windows (on Windows PC)
.\scripts\build-windows-simple.ps1
```

### Step 2: Create GitHub Release

```bash
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

Then on GitHub:
1. Go to Releases → Create new release
2. Select tag v0.1.0
3. Upload files from `dist/`
4. Copy template from DISTRIBUTION.md
5. Publish!

---

## When to Consider Code Signing

### Now (v0.1)
- ❌ Don't sign yet
- Focus on features
- Let users know it's safe but unsigned

### At 1,000 Downloads
- 💰 Get Apple Developer ($99/year)
- 🆓 Apply for free Windows signing (SignPath.io)
- Users get smoother macOS experience

### At 10,000 Downloads
- 💰 Consider Windows cert ($300/year)
- 🏆 Professional experience on both platforms
- Look into GitHub Sponsors to cover costs

---

## Cost Breakdown

| What | Cost | When |
|------|------|------|
| No signing | $0/year | ✅ Now |
| Apple Developer | $99/year | When popular |
| Windows cert | $300/year | If very popular |
| **Total** | **$0-400/year** | Scales with growth |

**Alternative:** Free Windows signing via [SignPath.io](https://signpath.io/oss) for OSS projects!

---

## Testing Checklist

Before releasing:

- [ ] Build macOS installer: `./scripts/build-mac-installer-simple.sh`
- [ ] Test install on clean Mac (or VM)
- [ ] Verify daemon starts: `launchctl list | grep keyswitch`
- [ ] Test keyboard switching
- [ ] Build Windows package: `.\scripts\build-windows-simple.ps1`
- [ ] Test install on Windows
- [ ] Verify daemon starts: `tasklist | findstr keyswitch`
- [ ] Test keyboard switching
- [ ] Generate checksums: `./scripts/create-release.sh`
- [ ] Test uninstall on both platforms
- [ ] Write release notes
- [ ] Create git tag
- [ ] Upload to GitHub releases

---

## Getting Help

- **Build issues:** See [BUILD_INSTALLERS.md](BUILD_INSTALLERS.md)
- **Code signing:** See [CODE_SIGNING.md](CODE_SIGNING.md)
- **User installation:** See [INSTALLATION_GUIDE.md](INSTALLATION_GUIDE.md)
- **Strategy:** See [DISTRIBUTION.md](DISTRIBUTION.md)

---

## Key Takeaway

**You're ready to release!**

The installers work great unsigned. Focus on building an awesome product first, worry about code signing later when the project grows.

Your comprehensive installation guide already explains the security warnings, so users will understand what's happening and how to proceed safely.

🚀 **Ship it!**
