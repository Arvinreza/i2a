#!/bin/bash
set -e

REPO="BlackTechX011/i2a"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="i2a"

# Detect OS and Arch
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        ASSET_NAME="i2a-linux-amd64.tar.gz"
        ;;
    Darwin)
        if [ "$ARCH" = "arm64" ]; then
            ASSET_NAME="i2a-macos-arm64.tar.gz"
        else
            ASSET_NAME="i2a-macos-amd64.tar.gz"
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Detected System: $OS ($ARCH)"
echo "Target Asset: $ASSET_NAME"

DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$ASSET_NAME"
TEMP_FILE="i2a_install.tar.gz"

echo "Downloading from $DOWNLOAD_URL..."
curl -L -f -o "$TEMP_FILE" "$DOWNLOAD_URL"

if [ ! -f "$TEMP_FILE" ]; then
    echo "Error: Download failed."
    exit 1
fi

echo "Extracting..."
tar -xzf "$TEMP_FILE"

if [ ! -f "$BINARY_NAME" ]; then
    echo "Error: Binary '$BINARY_NAME' not found in archive."
    exit 1
fi

echo "Installing to $INSTALL_DIR (requires sudo)..."
chmod +x "$BINARY_NAME"
sudo mv "$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

# Cleanup
rm "$TEMP_FILE" 2>/dev/null
rm Readme.md 2>/dev/null

echo "Success! Run '$BINARY_NAME --help' to get started."