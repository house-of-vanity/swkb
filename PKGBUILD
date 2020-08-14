# Maintainer: 

pkgname=
pkgver=
pkgrel=1
pkgdesc=
url=
arch=($CARCH)
license=
depends=()
makedepends=()
source=("git+https://github.com/house-of-vanity/$pkgname")
sha512sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  git describe --long --tags | awk -F '-' '{print $1}'| sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

prepare() {
  cd "$srcdir/$pkgname"
  cargo fetch --target $CARCH-unknown-linux-gnu
}

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --frozen --all-targets --all-features
}

package() {
  cd "$srcdir/$pkgname"
  install -Dt "$pkgdir/usr/bin" target/release/$pkgname
}
