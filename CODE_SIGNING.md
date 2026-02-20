# Code Signing Guide for KeySwitch

## TL;DR - What You Need to Know

**For an open source project, you have options:**

1. ❌ **Don't sign** - Users build from source or accept warnings (common for OSS)
2. ✅ **Sign official releases only** - Maintainer signs, users trust GitHub releases
3. ✅ **GitHub Actions** - Automated signing for releases (recommended)
4. ⚠️ **Self-signed** - Better than nothing, but still shows warnings

**Reality check:**
- macOS signing requires Apple Developer account: **$99/year**
- Windows signing requires code signing certificate: **$200-400/year**
- Many open source projects don't sign at all

## The User Experience Without Signing

### macOS (without signing)
When users install an unsigned .pkg:
```
"KeySwitch-0.1.0-arm64.pkg" cannot be opened because it is from an unidentified developer.
```

**Workaround for users:**
```bash
# Right-click the .pkg → Open
# Or use command line:
sudo installer -pkg KeySwitch-0.1.0-arm64.pkg -target /
```

### Windows (without signing)
Windows Defender SmartScreen shows:
```
Windows protected your PC
Unknown publisher
```

**Workaround for users:**
Click "More info" → "Run anyway"

## Recommended Approach for Open Source

### Option 1: Sign Official Releases Only (Best for OSS)

**How it works:**
1. Maintainer (you) gets Apple Developer account + code signing cert
2. Use GitHub Actions to automatically sign releases
3. Only signed releases on GitHub Releases page
4. Users can verify via checksums

**Cost:** $99/year (macOS) + ~$300/year (Windows) = ~$400/year

**Benefit:**
- Official releases are trusted
- Users building from source can still do so
- Professional appearance

### Option 2: Fully Open (No Signing)

**How it works:**
1. Provide clear installation instructions with warnings
2. Users accept security warnings or build from source
3. Provide checksums for verification

**Cost:** $0

**Benefit:**
- Zero cost
- Common for many OSS projects
- Users who care about security can build from source

### Option 3: Self-Signed Certificates

**How it works:**
Create your own certificates (free but users still see warnings)

**Cost:** $0

**Benefit:**
- Shows you attempted to sign
- Helps with some automated tools
- Still shows warnings to users

## Setting Up Code Signing

### macOS Code Signing

#### Prerequisites
1. **Apple Developer Account** ($99/year)
   - Sign up at: https://developer.apple.com/programs/enroll/
   - Takes ~24 hours to activate

2. **Developer ID Application Certificate**
   - Go to: https://developer.apple.com/account/resources/certificates
   - Click "+" to create new certificate
   - Select "Developer ID Application"
   - Follow prompts to generate and download

3. **Developer ID Installer Certificate**
   - Same process, but select "Developer ID Installer"
   - Used for signing .pkg files

#### Signing the Binary

```bash
# List available signing identities
security find-identity -v -p codesigning

# Sign the binary
codesign --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --timestamp \
  --options runtime \
  --entitlements entitlements.plist \
  target/release/keyswitch

# Verify signature
codesign --verify --verbose target/release/keyswitch
spctl --assess --verbose target/release/keyswitch
```

#### Entitlements File

Create `entitlements.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
    <true/>
    <key>com.apple.security.cs.disable-library-validation</key>
    <true/>
    <!-- Required for USB/HID access -->
    <key>com.apple.security.device.usb</key>
    <true/>
</dict>
</plist>
```

#### Signing the PKG

```bash
# Sign the package
productsign --sign "Developer ID Installer: Your Name (TEAM_ID)" \
  dist/KeySwitch-0.1.0-arm64.pkg \
  dist/KeySwitch-0.1.0-arm64-signed.pkg

# Verify
pkgutil --check-signature dist/KeySwitch-0.1.0-arm64-signed.pkg
```

#### Notarization (Required for macOS 10.15+)

Apple requires notarization for Gatekeeper:

```bash
# Store credentials (one-time)
xcrun notarytool store-credentials "keyswitch-notary" \
  --apple-id "your-email@example.com" \
  --team-id "YOUR_TEAM_ID" \
  --password "app-specific-password"

# Submit for notarization
xcrun notarytool submit dist/KeySwitch-0.1.0-arm64-signed.pkg \
  --keychain-profile "keyswitch-notary" \
  --wait

# Staple the notarization ticket
xcrun stapler staple dist/KeySwitch-0.1.0-arm64-signed.pkg

# Verify
xcrun stapler validate dist/KeySwitch-0.1.0-arm64-signed.pkg
spctl --assess --type install dist/KeySwitch-0.1.0-arm64-signed.pkg
```

**Note:** App-specific password:
1. Go to https://appleid.apple.com
2. Sign in → Security → App-Specific Passwords
3. Generate new password
4. Use in notarytool command

### Windows Code Signing

#### Prerequisites

**Option A: Purchase Certificate (~$200-400/year)**
- Sectigo, DigiCert, GlobalSign, etc.
- Requires business verification (EV certificates) or identity verification

**Option B: Open Source Certificate (Free but limited)**
- SignPath.io offers free signing for OSS projects
- Must be public GitHub repo
- Apply at: https://signpath.io/oss

#### Using SignPath (Free for OSS)

1. **Apply for free OSS plan:**
   - https://about.signpath.io/product/open-source
   - Link your GitHub repository
   - Wait for approval (~1 week)

2. **Set up GitHub integration:**
   - SignPath provides GitHub Actions workflow
   - Automatically signs releases

3. **Configure:**
   ```yaml
   # .github/workflows/sign-windows.yml
   name: Sign Windows Executable
   on:
     release:
       types: [published]

   jobs:
     sign:
       runs-on: windows-latest
       steps:
         - uses: signpath/github-action-submit-signing-request@v1
           with:
             api-token: ${{ secrets.SIGNPATH_API_TOKEN }}
             organization-id: ${{ secrets.SIGNPATH_ORG_ID }}
             project-slug: 'keyswitch'
             signing-policy-slug: 'release-signing'
             artifact-path: 'target/release/keyswitch.exe'
   ```

#### Manual Signing (If you have a certificate)

```powershell
# Using signtool (comes with Windows SDK)
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 target/release/keyswitch.exe

# Verify
signtool verify /pa target/release/keyswitch.exe
```

## Automated Signing with GitHub Actions

### Complete GitHub Actions Workflow

Create `.github/workflows/release.yml`:

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-mac:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-apple-darwin,aarch64-apple-darwin

      - name: Build universal binary
        run: |
          cargo build --release --target x86_64-apple-darwin
          cargo build --release --target aarch64-apple-darwin
          lipo -create \
            target/x86_64-apple-darwin/release/keyswitch \
            target/aarch64-apple-darwin/release/keyswitch \
            -output keyswitch-universal

      - name: Import certificates
        env:
          CERTIFICATE_BASE64: ${{ secrets.APPLE_CERTIFICATE_BASE64 }}
          CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
        run: |
          # Create keychain
          security create-keychain -p actions temp.keychain
          security default-keychain -s temp.keychain
          security unlock-keychain -p actions temp.keychain

          # Import certificate
          echo "$CERTIFICATE_BASE64" | base64 --decode > certificate.p12
          security import certificate.p12 -k temp.keychain -P "$CERTIFICATE_PASSWORD" -T /usr/bin/codesign
          security set-key-partition-list -S apple-tool:,apple: -s -k actions temp.keychain
          rm certificate.p12

      - name: Sign binary
        run: |
          codesign --sign "${{ secrets.APPLE_SIGNING_IDENTITY }}" \
            --timestamp \
            --options runtime \
            --entitlements entitlements.plist \
            keyswitch-universal

      - name: Build PKG
        run: |
          ./scripts/build-mac-installer.sh

      - name: Sign PKG
        run: |
          productsign --sign "${{ secrets.APPLE_INSTALLER_IDENTITY }}" \
            dist/KeySwitch-*.pkg \
            dist/KeySwitch-signed.pkg

      - name: Notarize
        env:
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_PASSWORD: ${{ secrets.APPLE_APP_PASSWORD }}
          TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
        run: |
          xcrun notarytool submit dist/KeySwitch-signed.pkg \
            --apple-id "$APPLE_ID" \
            --password "$APPLE_PASSWORD" \
            --team-id "$TEAM_ID" \
            --wait
          xcrun stapler staple dist/KeySwitch-signed.pkg

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: macos-installer
          path: dist/KeySwitch-signed.pkg

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build
        run: cargo build --release

      # If using SignPath:
      - name: Submit for signing
        uses: signpath/github-action-submit-signing-request@v1
        with:
          api-token: ${{ secrets.SIGNPATH_API_TOKEN }}
          organization-id: ${{ secrets.SIGNPATH_ORG_ID }}
          project-slug: 'keyswitch'
          signing-policy-slug: 'release-signing'
          artifact-path: 'target/release/keyswitch.exe'
          output-artifact-path: 'target/release/keyswitch-signed.exe'

      - name: Build installer
        run: .\scripts\build-windows-simple.ps1

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: windows-installer
          path: dist/*.zip

  create-release:
    needs: [build-mac, build-windows]
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

### Required GitHub Secrets

If using the above workflow, add these secrets to your repository:

**Settings → Secrets and variables → Actions → New repository secret**

#### macOS Secrets:
- `APPLE_CERTIFICATE_BASE64` - Base64 encoded .p12 certificate
- `APPLE_CERTIFICATE_PASSWORD` - Certificate password
- `APPLE_SIGNING_IDENTITY` - "Developer ID Application: Your Name (TEAM_ID)"
- `APPLE_INSTALLER_IDENTITY` - "Developer ID Installer: Your Name (TEAM_ID)"
- `APPLE_ID` - Your Apple ID email
- `APPLE_APP_PASSWORD` - App-specific password
- `APPLE_TEAM_ID` - Your team ID (10 character string)

#### Windows Secrets (if using SignPath):
- `SIGNPATH_API_TOKEN` - From SignPath dashboard
- `SIGNPATH_ORG_ID` - From SignPath dashboard

**Export certificate to base64:**
```bash
# Export from Keychain Access as .p12
# Then encode:
base64 -i certificate.p12 -o certificate.txt
# Copy contents of certificate.txt to GitHub secret
```

## My Recommendation for KeySwitch

Given this is an open source project, here's what I'd suggest:

### Phase 1: MVP (Now)
- ✅ Don't sign initially
- ✅ Provide clear installation instructions
- ✅ Include checksums in releases
- ✅ Document the security warnings users will see
- ✅ Provide source for users to build themselves

**Cost: $0**

### Phase 2: When Users Grow (Later)
- 💰 Get Apple Developer account ($99/year)
- 💰 Apply for SignPath free OSS plan (Windows)
- ⚙️ Set up GitHub Actions for automated signing
- 🚀 Sign all official releases

**Cost: $99/year**

### Phase 3: Professional (If project becomes popular)
- 💰 Purchase Windows code signing certificate ($300/year)
- ⚙️ Full automated signing pipeline
- 🏆 Fully trusted on both platforms

**Cost: $400/year**

## For Now: Document the Warnings

I'll create a user-friendly guide explaining the security warnings.

Would you like me to:
1. Create a guide for users explaining how to install despite warnings?
2. Set up the GitHub Actions workflow (without signing for now)?
3. Create the entitlements file for when you're ready to sign?
