#!/bin/bash

set -e

REPO="mishaal79/loku"
BINARY_NAME="loku"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $OS in
    darwin)
        OS_NAME="macos"
        ;;
    linux)
        OS_NAME="linux"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case $ARCH in
    x86_64)
        ARCH_NAME="x86_64"
        ;;
    aarch64|arm64)
        ARCH_NAME="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

BINARY_NAME_WITH_PLATFORM="${BINARY_NAME}-${OS_NAME}-${ARCH_NAME}"

echo "Installing loku for $OS_NAME $ARCH_NAME..."

# Get the latest release
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Failed to get latest release"
    exit 1
fi

echo "Latest release: $LATEST_RELEASE"

# Download the binary
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$BINARY_NAME_WITH_PLATFORM"
TEMP_FILE="/tmp/$BINARY_NAME"

echo "Downloading from: $DOWNLOAD_URL"
curl -L -o "$TEMP_FILE" "$DOWNLOAD_URL"

# Make it executable
chmod +x "$TEMP_FILE"

# Install to /usr/local/bin (requires sudo)
echo "Installing to $INSTALL_DIR (requires sudo)..."
sudo mv "$TEMP_FILE" "$INSTALL_DIR/$BINARY_NAME"

echo "loku installed successfully!"
echo "You can now use: loku --help"