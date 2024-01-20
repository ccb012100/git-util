#!/usr/bin/env sh
set -eu

cargo build || exit 1

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

util="$scriptdir"/target/debug/git-util

echo GIT_AUTHOR_EMAIL='397636+ccb012100@users.noreply.github.com' \
    GIT_UTIL_USER_EMAIL='397636+ccb012100@users.noreply.github.com' \
    GIT_UTIL_DISALLOWED_STRINGS="let|match" \
    "$util" --verbose --print-command hook pre-commit

GIT_AUTHOR_EMAIL='397636+ccb012100@users.noreply.github.com' \
    GIT_UTIL_USER_EMAIL='397636+ccb012100@users.noreply.github.com' \
    GIT_UTIL_DISALLOWED_STRINGS="let|match" \
    $util -vvv --print-command hook pre-commit
