#!/bin/sh
# Fetches libretro/picodrive's source at COMMIT with the submodules its libretro build reads,
# which a GitHub archive of the repository leaves out, and packs them as one archive, picodrive-COMMIT/
# with the submodules in place, at OUT. PicoDrive's licence asks for its source to ship beside a
# build, and this is that source.
set -eu

EMU2413=a2dfc20ff507e4fd075cd325620bcea655e2c1f7
LIBCHDR=e62ac5995b1c7ef65ece35293914843b8ee57d49
DR_LIBS=dd762b861ecadf5ddd5fb03e9ca1db6707b54fbb
LIBPICOFE=9ed5822606dd7ff20a782a882e8fd611cb53ba88

usage() {
	echo "usage: $0 COMMIT OUT" >&2
	exit 2
}

[ $# -eq 2 ] && [ -n "$1" ] && [ -n "$2" ] || usage
commit="$1" out="$2"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

get() {
	# get URL DIRNAME DEST: unpacks the archive at URL into $work/picodrive-$commit/DEST
	mkdir -p "$work/dl"
	curl -fsSL -o "$work/dl/a.tar.gz" "$1"
	rm -rf "$work/dl/x"
	mkdir -p "$work/dl/x"
	tar -xzf "$work/dl/a.tar.gz" -C "$work/dl/x"
	rm -rf "$work/picodrive-$commit/$2"
	mkdir -p "$(dirname "$work/picodrive-$commit/$2")"
	mv "$work/dl/x/$3-$4" "$work/picodrive-$commit/$2"
}

mkdir -p "$work/dl/main"
curl -fsSL -o "$work/dl/main.tar.gz" "https://github.com/libretro/picodrive/archive/$commit.tar.gz"
tar -xzf "$work/dl/main.tar.gz" -C "$work"
[ -f "$work/picodrive-$commit/Makefile.libretro" ] || {
	echo "the archive does not hold picodrive-$commit/Makefile.libretro" >&2
	exit 1
}

get "https://github.com/digital-sound-antiques/emu2413/archive/$EMU2413.tar.gz" pico/sound/emu2413 emu2413 "$EMU2413"
get "https://github.com/irixxxx/libchdr/archive/$LIBCHDR.tar.gz" pico/cd/libchdr libchdr "$LIBCHDR"
get "https://github.com/mackron/dr_libs/archive/$DR_LIBS.tar.gz" platform/common/dr_libs dr_libs "$DR_LIBS"
get "https://github.com/irixxxx/libpicofe/archive/$LIBPICOFE.tar.gz" platform/libpicofe libpicofe "$LIBPICOFE"

mkdir -p "$(dirname "$out")"
tar -czf "$out" -C "$work" "picodrive-$commit"
