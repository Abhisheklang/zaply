#!/bin/sh
# Install Zaply on Linux.
#
# Downloads the latest Zaply release for Linux (x86_64) from GitHub and
# installs it under ~/.local, so no administrator rights are needed.
# Re-running the script upgrades an existing install in place.
#
# Usage:  ./install.sh
# Remove: delete ~/.local/share/zaply and ~/.local/bin/zaply

set -eu

REPO="Abhisheklang/zaply"
ASSET="zaply-linux-x86_64.tar.gz"
URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"

INSTALL_DIR="${HOME}/.local/share/zaply"
BIN_DIR="${HOME}/.local/bin"
APP_DIR_TARGET="${INSTALL_DIR}/zaply.app"

if [ "$(uname -s)" != "Linux" ]; then
    echo "This install script is for Linux. Windows and macOS builds are on the way." >&2
    exit 1
fi

if [ "$(uname -m)" != "x86_64" ]; then
    echo "This install script currently provides x86_64 builds only." >&2
    exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading Zaply from ${URL}"
if command -v curl >/dev/null 2>&1; then
    curl -fL --progress-bar -o "${TMP_DIR}/${ASSET}" "$URL"
elif command -v wget >/dev/null 2>&1; then
    wget -O "${TMP_DIR}/${ASSET}" "$URL"
else
    echo "Neither curl nor wget was found. Install one of them and try again." >&2
    exit 1
fi

echo "Extracting..."
tar -xzf "${TMP_DIR}/${ASSET}" -C "${TMP_DIR}"

mkdir -p "$INSTALL_DIR" "$BIN_DIR"
rm -rf "$APP_DIR_TARGET"
mv "${TMP_DIR}/zaply.app" "$APP_DIR_TARGET"

ln -sf "${APP_DIR_TARGET}/bin/zaply" "${BIN_DIR}/zaply"

# Desktop entry and icon for the current user, so Zaply shows up in the
# application menu.
if [ -d "${HOME}/.local/share/applications" ] || [ -n "${XDG_DATA_HOME:-}" ]; then
    APPS_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}/applications"
    ICONS_DIR="${XDG_DATA_HOME:-${HOME}/.local/share}/icons"
    mkdir -p "$APPS_DIR" "$ICONS_DIR/hicolor/512x512/apps"
    sed "s|^Exec=zaply|Exec=${APP_DIR_TARGET}/bin/zaply|" \
        "${APP_DIR_TARGET}/share/applications/io.github.Abhisheklang.zaply.desktop" \
        > "${APPS_DIR}/io.github.Abhisheklang.zaply.desktop"
    cp "${APP_DIR_TARGET}/share/icons/hicolor/512x512/apps/zaply.png" \
        "${ICONS_DIR}/hicolor/512x512/apps/zaply.png"
fi

echo ""
echo "Zaply is installed. Run it with:"
echo "  zaply"
case ":$PATH:" in
    *":${BIN_DIR}:"*) ;;
    *) echo "Note: ${BIN_DIR} is not on your PATH yet. Add it in your shell profile to run 'zaply' from anywhere." ;;
esac
