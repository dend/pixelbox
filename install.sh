#!/usr/bin/env bash
set -euo pipefail

REPO="dend/pixelbox"
INSTALL_DIR="/usr/local/bin"

die() { echo "error: $*" >&2; exit 1; }

# Detect architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
    *)       die "unsupported architecture: $ARCH" ;;
esac

# Find the latest release tag
echo "Fetching latest release..."
TAG=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | head -1 | cut -d'"' -f4)

[ -n "$TAG" ] || die "could not determine latest release"
echo "Latest release: $TAG"

# Download
ARCHIVE="pixelbox-${TAG}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${TAG}/${ARCHIVE}"
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading ${ARCHIVE}..."
curl -fSL -o "${TMPDIR}/${ARCHIVE}" "$URL" \
    || die "download failed — check that a release exists for ${TARGET}"

# Extract
tar xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"
[ -f "${TMPDIR}/pixelbox" ] || die "archive did not contain pixelbox binary"

# Install
echo "Installing to ${INSTALL_DIR}/pixelbox (may need sudo)..."
if [ -w "$INSTALL_DIR" ]; then
    mv "${TMPDIR}/pixelbox" "${INSTALL_DIR}/pixelbox"
else
    sudo mv "${TMPDIR}/pixelbox" "${INSTALL_DIR}/pixelbox"
fi
chmod +x "${INSTALL_DIR}/pixelbox"

# Runtime dependencies
if command -v apt-get >/dev/null 2>&1; then
    if ! dpkg -s bluez >/dev/null 2>&1; then
        echo "Installing bluetooth runtime dependencies..."
        sudo apt-get update -qq
        sudo apt-get install -y -qq bluetooth bluez
    fi
    if ! systemctl is-active --quiet bluetooth; then
        sudo systemctl enable --now bluetooth
    fi
fi

echo ""
echo "pixelbox $(pixelbox --help 2>&1 | head -1 || true) installed to ${INSTALL_DIR}/pixelbox"
echo ""
echo "Run 'pixelbox scan' to find your Ditoo Pro."
