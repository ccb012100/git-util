#!/usr/bin/env bash
set -Eeou pipefail

cargo build || exit 1

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

util="$scriptdir"/target/debug/git-util

echo 'git alias commit'
$util alias commit || exit 1

echo 'git ll 1'
$util ll 1 || exit 1

echo 'git last'
$util last || exit 1

echo 'git show 1'
$util show || exit 1
