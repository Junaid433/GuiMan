# Maintainer: Junaid Rahman <junaid433@example.com>
pkgname=guiman
pkgver=0.1.0
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
    'rust'
    'cargo'
    'nodejs'
    'npm'
    'git'
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
    
    # Install npm dependencies
    npm install
}

build() {
    cd "$pkgname"
    
    # Build the application
    npm run tauri build
}

package() {
    cd "$pkgname"
    
    # Install the binary
    install -Dm755 "src-tauri/target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    
    # Install desktop file
    install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
    
    # Install icon
    install -Dm644 "src-tauri/icons/icon.png" \
        "$pkgdir/usr/share/pixmaps/$pkgname.png"
    
    # Install polkit policy
    install -Dm644 "polkit/com.guiman.pkexec.policy" \
        "$pkgdir/usr/share/polkit-1/actions/com.guiman.pkexec.policy"
    
    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    
    # Install documentation
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
