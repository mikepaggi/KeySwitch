#!/bin/bash
set -e

# Simplified macOS installer builder (builds for current architecture only)
# For universal binary, use build-mac-installer.sh instead

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
ARCH=$(uname -m)
PKG_NAME="KeySwitch-${VERSION}-${ARCH}.pkg"
BUILD_DIR="$PROJECT_ROOT/build/mac"

echo "Building KeySwitch macOS installer v${VERSION} for ${ARCH}..."

# Clean and create build directories
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/root/usr/local/bin"
mkdir -p "$BUILD_DIR/root/Library/LaunchAgents"
mkdir -p "$BUILD_DIR/scripts"
mkdir -p "$PROJECT_ROOT/dist"

# Build the release binary
echo "Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release

# Copy binary
echo "Copying binary..."
cp "$PROJECT_ROOT/target/release/keyswitch" "$BUILD_DIR/root/usr/local/bin/keyswitch"
chmod +x "$BUILD_DIR/root/usr/local/bin/keyswitch"

# Copy LaunchAgent plist
echo "Copying LaunchAgent configuration..."
cp "$PROJECT_ROOT/com.keyswitch.daemon.plist" "$BUILD_DIR/root/Library/LaunchAgents/"

# Create postinstall script
cat > "$BUILD_DIR/scripts/postinstall" << 'POSTINSTALL'
#!/bin/bash
# Post-installation script for KeySwitch

# Get the user who is installing (not root)
CURRENT_USER="${USER}"
if [ -z "$CURRENT_USER" ] || [ "$CURRENT_USER" = "root" ]; then
    CURRENT_USER=$(stat -f "%Su" /dev/console)
fi

# Copy plist to user's LaunchAgents directory
USER_HOME=$(eval echo ~$CURRENT_USER)
mkdir -p "$USER_HOME/Library/LaunchAgents"
cp "/Library/LaunchAgents/com.keyswitch.daemon.plist" "$USER_HOME/Library/LaunchAgents/" 2>/dev/null || true

# Set correct ownership
chown "$CURRENT_USER" "$USER_HOME/Library/LaunchAgents/com.keyswitch.daemon.plist" 2>/dev/null || true

# Try to load the LaunchAgent
if [ -n "$CURRENT_USER" ]; then
    sudo -u "$CURRENT_USER" launchctl load -w "$USER_HOME/Library/LaunchAgents/com.keyswitch.daemon.plist" 2>/dev/null || true
fi

echo "KeySwitch has been installed successfully!"
echo "The daemon will start automatically on next login."

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
    "$PROJECT_ROOT/dist/$PKG_NAME"

echo ""
echo "✅ Installer created successfully!"
echo "📦 Location: dist/$PKG_NAME"
echo ""
echo "To create a universal binary (Intel + Apple Silicon), use build-mac-installer.sh"
echo ""
echo "Users can install by double-clicking the .pkg file"
