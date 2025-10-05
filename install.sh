#!/usr/bin/env bash
# install.sh - Manual installer for guiman (based on PKGBUILD)
# Maintainer: Junaid Rahman <junaid.cloud2@gmail.com>
set -e

PKGNAME="guiman"
PKGVER="0.1.1"
REPO_URL="https://github.com/Junaid433/guiman"
BUILD_DIR="$(pwd)/${PKGNAME}-build"
INSTALL_DIR="/usr/local/bin"
DESKTOP_DIR="/usr/share/applications"
ICON_DIR="/usr/share/pixmaps"
POLKIT_DIR="/usr/share/polkit-1/actions"
DOC_DIR="/usr/share/doc/${PKGNAME}"
LICENSE_DIR="/usr/share/licenses/${PKGNAME}"

echo "==> Installing ${PKGNAME} v${PKGVER}"

# ------------------------------------------------------------------------------
# Step 1: Install dependencies
# ------------------------------------------------------------------------------
echo "==> Checking dependencies..."

DEPS=(webkit2gtk gtk3 libayatana-appindicator pacman sudo)
MAKEDEPS=(nodejs npm git ffmpeg pkgconf openssl base-devel)

for dep in "${DEPS[@]}" "${MAKEDEPS[@]}"; do
    if ! pacman -Qi "$dep" &>/dev/null; then
        echo "  -> Missing dependency: $dep"
        MISSING_PKGS=1
    fi
done

if [ "$MISSING_PKGS" = "1" ]; then
    echo "==> Installing missing dependencies..."
    sudo pacman -S --needed --noconfirm "${DEPS[@]}" "${MAKEDEPS[@]}"
fi

# ------------------------------------------------------------------------------
# Step 2: Prepare build environment
# ------------------------------------------------------------------------------
echo "==> Preparing build environment..."

rm -rf "$BUILD_DIR"
git clone --branch "v${PKGVER}" "$REPO_URL" "$BUILD_DIR"
cd "$BUILD_DIR"

# Ensure ffmpeg and npm
command -v ffmpeg >/dev/null 2>&1 || { echo "ffmpeg not installed"; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "npm not installed"; exit 1; }

# Ensure Rust installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "==> Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source ~/.cargo/env
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# Fix icon and product name
if [ -f "src-tauri/icons/icon.png" ]; then
    ffmpeg -i src-tauri/icons/icon.png -vf "format=rgba" src-tauri/icons/icon_rgba.png -y -loglevel error
    mv src-tauri/icons/icon_rgba.png src-tauri/icons/icon.png
    echo "  -> Icon converted to RGBA format"
fi

if [ -f "src-tauri/tauri.conf.json" ]; then
    sed -i 's/"productName": "GuiMan"/"productName": "guiman"/' src-tauri/tauri.conf.json
    echo "  -> Product name fixed"
fi

# NPM dependencies
echo "==> Installing npm dependencies..."
npm ci --only=production --silent || npm install --silent

# ------------------------------------------------------------------------------
# Step 3: Build application
# ------------------------------------------------------------------------------
echo "==> Building guiman..."

export CARGO_BUILD_JOBS="$(nproc)"
export RUSTFLAGS="-C debuginfo=1 -C target-cpu=native -C opt-level=3"
npm run tauri:build

if [ ! -f "src-tauri/target/release/${PKGNAME}" ]; then
    echo "Build failed: binary not found"
    exit 1
fi

# ------------------------------------------------------------------------------
# Step 4: Install files
# ------------------------------------------------------------------------------
echo "==> Installing files..."

sudo install -Dm755 "src-tauri/target/release/${PKGNAME}" "${INSTALL_DIR}/${PKGNAME}"

[ -f "${PKGNAME}.desktop" ] && \
    sudo install -Dm644 "${PKGNAME}.desktop" "${DESKTOP_DIR}/${PKGNAME}.desktop"

[ -f "src-tauri/icons/icon.png" ] && \
    sudo install -Dm644 "src-tauri/icons/icon.png" "${ICON_DIR}/${PKGNAME}.png"

[ -f "polkit/com.guiman.pkexec.policy" ] && \
    sudo install -Dm644 "polkit/com.guiman.pkexec.policy" "${POLKIT_DIR}/com.guiman.pkexec.policy"

[ -f "LICENSE" ] && \
    sudo install -Dm644 "LICENSE" "${LICENSE_DIR}/LICENSE"

[ -f "README.md" ] && \
    sudo install -Dm644 "README.md" "${DOC_DIR}/README.md"

echo "==> Installation complete!"
echo
echo "✅ ${PKGNAME} has been installed successfully."
echo "You can now run it using:  ${PKGNAME}"
echo

# ------------------------------------------------------------------------------
# Step 5: Optional tools
# ------------------------------------------------------------------------------
echo "==> Optional: Install additional tools for extended functionality"
echo "  - yay / paru : AUR helper support"
echo "  - reflector  : Mirror management"
echo "  - polkit     : Password-free privilege operations"
echo
echo "You can install them anytime with:"
echo "  sudo pacman -S yay reflector polkit"
