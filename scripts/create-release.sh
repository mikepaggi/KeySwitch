#!/bin/bash
set -e

# Create release packages with checksums
# Run this to prepare files for GitHub release

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
DIST_DIR="$PROJECT_ROOT/dist"

cd "$PROJECT_ROOT"

echo "======================================"
echo "Creating KeySwitch v$VERSION Release"
echo "======================================"
echo ""

# Create dist directory
mkdir -p "$DIST_DIR"

# Build macOS installer
echo "📦 Building macOS installer..."
./scripts/build-mac-installer-simple.sh
echo ""

# Generate checksums
echo "🔐 Generating checksums..."
cd "$DIST_DIR"

# Create checksums file
CHECKSUM_FILE="checksums.txt"
rm -f "$CHECKSUM_FILE"

echo "KeySwitch v$VERSION" > "$CHECKSUM_FILE"
echo "==================" >> "$CHECKSUM_FILE"
echo "" >> "$CHECKSUM_FILE"

for file in KeySwitch-*.pkg KeySwitch-*.zip 2>/dev/null; do
    if [ -f "$file" ]; then
        echo "Computing checksum for $file..."
        shasum -a 256 "$file" >> "$CHECKSUM_FILE"
    fi
done

echo "" >> "$CHECKSUM_FILE"
echo "Verify with:" >> "$CHECKSUM_FILE"
echo "  shasum -a 256 -c checksums.txt" >> "$CHECKSUM_FILE"

cd "$PROJECT_ROOT"

echo ""
echo "✅ Release packages created!"
echo ""
echo "📁 Files in dist/:"
ls -lh "$DIST_DIR"
echo ""
echo "📋 Checksums:"
cat "$DIST_DIR/checksums.txt"
echo ""
echo "Next steps:"
echo "1. Test the installers on clean systems"
echo "2. Create a git tag: git tag -a v$VERSION -m 'Release v$VERSION'"
echo "3. Push the tag: git push origin v$VERSION"
echo "4. Create GitHub release and upload files from dist/"
echo ""
echo "For Windows builds, run on a Windows machine:"
echo "  .\scripts\build-windows-simple.ps1"
echo "  Copy the .zip to dist/ and update checksums"
