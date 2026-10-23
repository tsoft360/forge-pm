pkgname=forge-pm
pkgver=1.1.3
pkgrel=1
pkgdesc="A Rust-based updater for Forge Desktop"
arch=('x86_64')
url="https://github.com/tsoft360/forge-pm"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')

package() {
    git pull "url"
    install -Dm755 "target/release/forge-pm" \
        "$pkgdir/usr/bin/forge-pm"

    install -Dm644 LICENSE \
        "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}