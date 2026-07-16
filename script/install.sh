#!/usr/bin/env bash
set -euo pipefail

REPO="ChxisB/zed"
VERSION="${1:-latest}"
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS-$ARCH" in
  Darwin-arm64)  TARGET="macos-aarch64"  ;;
  Darwin-x86_64) TARGET="macos-x86_64"   ;;
  Linux-arm64)   TARGET="linux-aarch64"  ;;
  Linux-x86_64)  TARGET="linux-x86_64"   ;;
  *)
    echo "Unsupported platform: $OS-$ARCH"
    echo "Windows users: run the PowerShell script instead:"
    echo "  irm https://raw.githubusercontent.com/$REPO/main/script/install.ps1 | iex"
    echo ""
    echo "Or build from source: https://github.com/$REPO#building-from-source"
    exit 1
    ;;
esac

URL="https://github.com/$REPO/releases/$VERSION/download/zed-$TARGET.tar.gz"
INSTALL_DIR="${ZED_INSTALL:-/usr/local/bin}"

echo "Downloading Zed ($TARGET)..."
mkdir -p "$INSTALL_DIR"
curl -#fL "$URL" | tar xz -C "$INSTALL_DIR" zed
chmod +x "$INSTALL_DIR/zed"

echo ""
echo "Installed to $INSTALL_DIR/zed"
echo "Run: zed"
