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
    'rust'
    'cargo'
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
source=("git+$url.git")
sha256sums=('SKIP')

# Automatically detect version from Git tags
pkgver() {
    cd "$srcdir/guiman"
    # Get the latest tag like v0.1.2
    git describe --tags --abbrev=0 | sed 's/^v//'
}

prepare() {
    cd "$srcdir/guiman"

    # Convert icon to RGBA if it exists
    if [ -f "src-tauri/icons/icon.png" ]; then
        ffmpeg -y -loglevel error -i src-tauri/icons/icon.png -vf "format=rgba" src-tauri/icons/icon_rgba.png
        mv src-tauri/icons/icon_rgba.png src-tauri/icons/icon.png
        echo "Icon converted to RGBA format"
    fi

    # Fix productName in tauri config
    if [ -f "src-tauri/tauri.conf.json" ]; then
        sed -i 's/"productName": "GuiMan"/"productName": "guiman"/' src-tauri/tauri.conf.json
        echo "Fixed productName in tauri.conf.json"
    fi
}

build() {
    cd "$srcdir/guiman"

    # Install npm dependencies
    npm ci --only=production --silent || npm install --silent

    # Build Tauri app
    npm run tauri:build || {
        echo "Build failed" >&2
        exit 1
    }
}

package() {
    cd "$srcdir/guiman"

    # Install binary
    install -Dm755 "src-tauri/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

    # Desktop file
    [ -f "$pkgname.desktop" ] && install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"

    # Icon
    [ -f "src-tauri/icons/icon.png" ] && install -Dm644 "src-tauri/icons/icon.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"

    # Polkit policy
    [ -f "polkit/com.guiman.pkexec.policy" ] && install -Dm644 "polkit/com.guiman.pkexec.policy" "$pkgdir/usr/share/polkit-1/actions/com.guiman.pkexec.policy"

    # License and README
    [ -f LICENSE ] && install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    [ -f README.md ] && install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
