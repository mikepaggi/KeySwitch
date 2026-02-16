#!/bin/bash
set -e

# Build macOS installer package for KeySwitch
# This creates a .pkg file that non-technical users can double-click to install

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
PKG_NAME="KeySwitch-${VERSION}.pkg"
BUILD_DIR="$PROJECT_ROOT/build/mac"

echo "Building KeySwitch macOS installer v${VERSION}..."

# Clean and create build directories
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/root/usr/local/bin"
mkdir -p "$BUILD_DIR/root/Library/LaunchAgents"
mkdir -p "$BUILD_DIR/scripts"
mkdir -p "$PROJECT_ROOT/dist"

# Build the release binary
echo "Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary (supports both Intel and Apple Silicon)
echo "Creating universal binary..."
lipo -create \
    "$PROJECT_ROOT/target/x86_64-apple-darwin/release/keyswitch" \
    "$PROJECT_ROOT/target/aarch64-apple-darwin/release/keyswitch" \
    -output "$BUILD_DIR/root/usr/local/bin/keyswitch"

chmod +x "$BUILD_DIR/root/usr/local/bin/keyswitch"

# Copy LaunchAgent plist
echo "Copying LaunchAgent configuration..."
cp "$PROJECT_ROOT/com.keyswitch.daemon.plist" "$BUILD_DIR/root/Library/LaunchAgents/"

# Create postinstall script
cat > "$BUILD_DIR/scripts/postinstall" << 'POSTINSTALL'
#!/bin/bash
# Post-installation script for KeySwitch

# Load the LaunchAgent for the current user
CURRENT_USER="${USER}"
if [ -z "$CURRENT_USER" ]; then
    CURRENT_USER=$(stat -f "%Su" /dev/console)
fi

if [ -n "$CURRENT_USER" ]; then
    sudo -u "$CURRENT_USER" launchctl load -w "/Library/LaunchAgents/com.keyswitch.daemon.plist" 2>/dev/null || true
fi

echo "KeySwitch has been installed successfully!"
echo "The daemon will start automatically on next login."
echo "To start it now, run: launchctl load -w ~/Library/LaunchAgents/com.keyswitch.daemon.plist"

exit 0
POSTINSTALL

chmod +x "$BUILD_DIR/scripts/postinstall"

# Build the package
echo "Building package..."
pkgbuild \
    --root "$BUILD_DIR/root" \
    --scripts "$BUILD_DIR/scripts" \
    --identifier "com.keyswitch.daemon" \
    --version "$VERSION" \
    --install-location "/" \
    "$BUILD_DIR/KeySwitch-component.pkg"

# Create product package with better presentation
echo "Creating distribution package..."
productbuild \
    --package "$BUILD_DIR/KeySwitch-component.pkg" \
    --distribution "$PROJECT_ROOT/scripts/distribution.xml" \
    "$PROJECT_ROOT/dist/$PKG_NAME"

echo ""
echo "✅ Installer created successfully!"
echo "📦 Location: dist/$PKG_NAME"
echo ""
echo "Users can install by double-clicking the .pkg file"
