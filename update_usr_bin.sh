#!/usr/bin/env sh
set -eu

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

cargo build --release || exit 1

# ANSI colors
clearformat='\033[0m' # clear formatting
orange='\033[0;33m'

info() {
    printf >&2 "%b%s%b\n" "$orange" "${*}" "$clearformat"
}

util=git-util

echo cp -uv "$scriptdir/target/release/$util" "$HOME/bin/$util"
cp -uv "$scriptdir/target/release/$util" "$HOME/bin/$util" || exit 1

info Updated "$HOME/bin/$util"
