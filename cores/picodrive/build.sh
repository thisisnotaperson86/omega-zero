#!/bin/sh
set -eu

here="$(cd "$(dirname "$0")" && pwd)"

device_cflags="-mcpu=cortex-a53"
# PicoDrive is C and asks BaseOS for nothing past glibc.
device_ldflags=""

usage() {
	echo "usage: $0 stamp COMMIT | build COMMIT TARBALL WORKDIR OUT" >&2
	exit 2
}

sha256() {
	if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1"; else shasum -a 256 "$1"; fi |
		cut -d' ' -f1
}

stamp() {
	echo "commit=$1"
	echo "source=https://github.com/libretro/picodrive/archive/$1.tar.gz, with its submodules (see fetch.sh)"
	echo "recipe=make -f Makefile.libretro platform=aarch64"
	echo "device_cflags=$device_cflags"
	echo "device_ldflags=$device_ldflags"
	for p in "$here"/*.patch; do
		[ -f "$p" ] || continue
		echo "patch=$(basename "$p") sha256:$(sha256 "$p")"
	done
}

# Applies the patches with git kept out of any repository.
apply_patches() {
	ceiling="$(cd "$1/.." && pwd)"
	for p in "$here"/*.patch; do
		[ -f "$p" ] || continue
		GIT_CEILING_DIRECTORIES="$ceiling" git -C "$1" apply -p1 "$p"
	done
}

# Builds for the device on an aarch64 linux box, or on an x86_64 linux box with CROSS set to an
# aarch64 toolchain's prefix (aarch64-linux-gnu-).
build() {
	commit="$1" tarball="$2" work="$3" out="$4"
	src="$work/picodrive-$commit"

	rm -rf "$work"
	mkdir -p "$work"
	tar -xzf "$tarball" -C "$work"
	if [ ! -f "$src/Makefile.libretro" ]; then
		echo "$tarball does not hold picodrive-$commit/Makefile.libretro" >&2
		exit 1
	fi

	apply_patches "$src"

	tools=""
	case "$(uname -s)-$(uname -m)" in
	Linux-aarch64) ;;
	Linux-x86_64)
		if [ -z "${CROSS:-}" ]; then
			echo "set CROSS to an aarch64 toolchain's prefix to build PicoDrive here" >&2
			exit 1
		fi
		tools="CC=${CROSS}gcc CXX=${CROSS}g++ AR=${CROSS}ar"
		;;
	*)
		echo "build PicoDrive on an aarch64 linux box, not $(uname -s)-$(uname -m)" >&2
		exit 1
		;;
	esac

	# shellcheck disable=SC2086
	CFLAGS="$device_cflags" LDFLAGS="$device_ldflags" make -C "$src" -f Makefile.libretro \
		platform=aarch64 ARCH=aarch64 $tools -j"$(getconf _NPROCESSORS_ONLN)"

	mkdir -p "$(dirname "$out")"
	cp "$src/picodrive_libretro.so" "$out"
	stamp "$commit" >"$out.meta"
}

case "${1:-}" in
stamp)
	[ $# -eq 2 ] && [ -n "$2" ] || usage
	stamp "$2"
	;;
build)
	[ $# -eq 5 ] && [ -n "$2" ] && [ -n "$3" ] && [ -n "$4" ] && [ -n "$5" ] || usage
	build "$2" "$3" "$4" "$5"
	;;
*)
	usage
	;;
esac
