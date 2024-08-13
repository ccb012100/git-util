#!/usr/bin/env sh
set -eu

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

cargo build --release || exit 1

# ANSI colors
clearformat='\033[0m' # clear formatting
orange='\033[0;33m'
green='\e[0;32m'

info() {
    printf >&2 "%b%s%b\n" "$orange" "${*}" "$clearformat"
}
success() {
    printf >&2 "%b%s%b\n" "$green" "${*}" "$clearformat"
}

util=git-util

# add -n/--dry-run flag if testing
info Copying "$scriptdir/target/release/$util" to "$HOME/bin/$util"
rsync --recursive --times --progress --protect-args "$scriptdir/target/release/$util" "$HOME/bin/$util"
success "$HOME/bin/$util" is up to date
