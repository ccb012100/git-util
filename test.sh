#!/usr/bin/env bash
set -Eeou pipefail

cargo build || exit 1

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

util="$scriptdir"/target/debug/git-util

echo git alias
$util alias || exit 1

echo 'git alias commit'
$util alias commit || exit 1

echo 'git ll 1'
$util ll 1 || exit 1

echo 'git ll'
$util ll || exit 1

echo 'git ll -- -p'
$util ll 1 -- -p || exit 1
