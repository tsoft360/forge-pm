# Maintainer: Thijs Boelens thijsboelens2@gmail.com

pkgname=forge-pm
pkgver=1.1.4
pkgrel=1
pkgdesc="A Rust-based updater for Forge Desktop"
arch=('x86_64')
url="https://github.com/tsoft360/forge-pm"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
options=('!lto')

source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo fetch --locked
}

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --frozen --release
}

check() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo test --frozen
}

package() {
    cd "$srcdir/$pkgname-$pkgver"

    install -Dm755 "target/release/forge-pm" \
        "$pkgdir/usr/bin/forge-pm"

    install -Dm644 LICENSE \
        "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}