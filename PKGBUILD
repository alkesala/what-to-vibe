# Maintainer: Your Name <your.email@example.com>
pkgname=what-to-vibe
pkgver=0.1.0
pkgrel=1
pkgdesc="A Rust CLI application that provides themed responses based on your current mood"
arch=('x86_64')
url="https://github.com/yourusername/what-to-vibe"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo' 'git')
source=("git+https://github.com/yourusername/what-to-vibe.git")
sha256sums=('SKIP')

pkgver() {
    cd "$pkgname"
    printf "0.1.0.r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
    cd "$pkgname"
    cargo build --release --locked
}

package() {
    cd "$pkgname"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/vibe"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
} 