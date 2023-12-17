#!/usr/bin/env sh
set -eu

cargo build || exit 1

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

util="$scriptdir"/target/debug/git-util

echo GIT_UTIL_USER_EMAIL='397636+ccb012100@users.noreply.github.com' "$util" --verbose --print-command hook precommit
GIT_UTIL_USER_EMAIL='397636+ccb012100@users.noreply.github.com' $util --verbose --print-command hook precommit
