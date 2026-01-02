#!/bin/sh
set -e

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Map OS names
case "$OS" in
    linux) OS="linux" ;;
    darwin) OS="macos" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Configuration
REPO="ikok07/dxcli"
BINARY_NAME="dx"
VERSION="latest"

if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
fi

if [ "$OS" = "linux" ]; then
    FILENAME="${BINARY_NAME}-${OS}-${ARCH}.tar.gz"
else
    FILENAME="${BINARY_NAME}-${OS}-${ARCH}.tar.gz"
fi

URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"

echo "Downloading $BINARY_NAME $VERSION for $OS-$ARCH..."
echo "URL: $URL"

TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

if command -v curl > /dev/null; then
    curl -fsSL "$URL" -o "$FILENAME"
elif command -v wget > /dev/null; then
    wget -q "$URL" -O "$FILENAME"
else
    echo "Error: curl or wget is required"
    exit 1
fi

tar xzf "$FILENAME"

INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/"
    echo "✓ Installed to $INSTALL_DIR/$BINARY_NAME"
else
    echo "Installing to $INSTALL_DIR (requires sudo)..."
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/"
    echo "✓ Installed to $INSTALL_DIR/$BINARY_NAME"
fi

# Cleanup
cd -
rm -rf "$TMP_DIR"

echo ""
echo "Installation complete! Run '$BINARY_NAME --help' to get started."