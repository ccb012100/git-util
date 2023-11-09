#!/usr/bin/env bash
set -Eeou pipefail

cargo build || exit 1

scriptdir=$(dirname -- "$(readlink -f -- "$0")")

util="$scriptdir"/target/debug/git-util

echo git alias
$util alias || exit 1

echo 'git alias commit'
$util alias commit || exit 1

echo 'git l 1'
$util l 1 || exit 1

echo 'git l'
$util l || exit 1

echo 'git l -- -p'
$util l 1 -- -p || exit 1
