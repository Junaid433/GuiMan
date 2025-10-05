# Maintainer: Junaid Rahman <junaid.cloud2@gmail.com>
pkgname=guiman
pkgver=0.1.1
pkgrel=1
pkgdesc="The Ultimate Arch Linux Package Manager - GUI with 100% pacman & AUR feature parity"
arch=('x86_64')
url="https://github.com/Junaid433/guiman"
license=('MIT')
depends=(
    'webkit2gtk'
    'gtk3'
    'libayatana-appindicator'
    'pacman'
    'sudo'
)
makedepends=(
    'nodejs'
    'npm'
    'git'
    'ffmpeg'
    'pkgconf'
    'openssl'
    'base-devel'
)
optdepends=(
    'yay: AUR helper support'
    'paru: AUR helper support'
    'reflector: Mirror management'
    'polkit: Password-free operations'
)
provides=('guiman')
conflicts=('guiman-bin' 'guiman-git')
source=("git+$url.git#tag=v$pkgver")
sha256sums=('SKIP')

prepare() {
    cd "$pkgname"
    
    command -v ffmpeg >/dev/null 2>&1 || { echo "ffmpeg is required but not installed. Aborting." >&2; exit 1; }
    command -v npm >/dev/null 2>&1 || { echo "npm is required but not installed. Aborting." >&2; exit 1; }
    
    if ! command -v cargo >/dev/null 2>&1; then
        echo "Rust/Cargo not found. Installing rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        source ~/.cargo/env
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
    
    command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not available. Aborting." >&2; exit 1; }
    command -v rustc >/dev/null 2>&1 || { echo "rustc is required but not available. Aborting." >&2; exit 1; }
    
    if [ -f "src-tauri/icons/icon.png" ]; then
        ffmpeg -i src-tauri/icons/icon.png -vf "format=rgba" src-tauri/icons/icon_rgba.png -y -loglevel error
        mv src-tauri/icons/icon_rgba.png src-tauri/icons/icon.png
        echo "Icon converted to RGBA format"
    else
        echo "Warning: Icon file not found at src-tauri/icons/icon.png"
    fi
    
    if [ -f "src-tauri/tauri.conf.json" ]; then
        sed -i 's/"productName": "GuiMan"/"productName": "guiman"/' src-tauri/tauri.conf.json
        echo "Fixed productName in tauri.conf.json"
    else
        echo "Error: tauri.conf.json not found" >&2
        exit 1
    fi
    
    echo "Installing npm dependencies..."
    npm ci --only=production --silent || {
        echo "npm ci failed, trying npm install..." >&2
        npm install --silent || {
            echo "npm install failed" >&2
            exit 1
        }
    }
    
    [ -f "package.json" ] || { echo "package.json not found" >&2; exit 1; }
    [ -f "src-tauri/Cargo.toml" ] || { echo "Cargo.toml not found" >&2; exit 1; }
    [ -f "src-tauri/tauri.conf.json" ] || { echo "tauri.conf.json not found" >&2; exit 1; }
}

build() {
    cd "$pkgname"
    
    if [ -f ~/.cargo/env ]; then
        source ~/.cargo/env
        export PATH="$HOME/.cargo/bin:$PATH"
    fi
    
    export CARGO_TARGET_DIR="$srcdir/target"
    export CARGO_BUILD_JOBS="$(nproc)"
    
    export CFLAGS="-march=x86-64 -mtune=generic -O2 -pipe -fno-plt -fexceptions -Wp,-D_FORTIFY_SOURCE=3 -Wformat -Werror=format-security -fstack-clash-protection -fcf-protection -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer -g -flto=auto"
    export CXXFLAGS="$CFLAGS"
    
    export RUSTFLAGS="-C debuginfo=1 -C target-cpu=native -C opt-level=3"
    export CARGO_PROFILE_RELEASE_DEBUG=1
    export CARGO_PROFILE_RELEASE_LTO=true
    export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
    
    echo "Building application..."
    npm run tauri:build || {
        echo "Build failed" >&2
        exit 1
    }
    
    if [ ! -f "$srcdir/target/release/$pkgname" ]; then
        echo "Error: Binary not found at $srcdir/target/release/$pkgname" >&2
        exit 1
    fi
    
    echo "Build completed successfully"
}

package() {
    cd "$pkgname"
    
    if [ ! -f "$srcdir/target/release/$pkgname" ]; then
        echo "Error: Binary not found for packaging" >&2
        exit 1
    fi
    
    install -Dm755 "$srcdir/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    
    if [ -f "$pkgname.desktop" ]; then
        install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
    else
        echo "Warning: Desktop file not found"
    fi
    
    if [ -f "src-tauri/icons/icon.png" ]; then
        install -Dm644 "src-tauri/icons/icon.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"
    else
        echo "Warning: Icon file not found"
    fi
    
    if [ -f "polkit/com.guiman.pkexec.policy" ]; then
        install -Dm644 "polkit/com.guiman.pkexec.policy" "$pkgdir/usr/share/polkit-1/actions/com.guiman.pkexec.policy"
    else
        echo "Warning: Polkit policy not found"
    fi
    
    if [ -f "LICENSE" ]; then
        install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    else
        echo "Warning: LICENSE file not found"
    fi
    
    if [ -f "README.md" ]; then
        install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    else
        echo "Warning: README.md not found"
    fi
    
    echo "Package created successfully"
}