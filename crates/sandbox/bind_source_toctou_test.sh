#!/usr/bin/env bash
set -u
command -v bwrap >/dev/null || { echo "bwrap not found" >&2; exit 2; }

ROOT="$(mktemp -d)"; trap 'rm -rf "$ROOT"' EXIT
READ_WRITE_DIR="$ROOT/foo/bar"
ATTACK_TARGET="$ROOT/baz"
mkdir -p "$READ_WRITE_DIR" "$ATTACK_TARGET"

# /foo (parent of the grant) is read-only; only /foo/bar is writable, so staging
# then renaming a symlink over /foo/bar fails.
# 
# Writable access to `/foo/bar` should not result in the ability to make
# `/foo/bar` into a symlink.
bwrap \
  --ro-bind / / \
  --bind "$READ_WRITE_DIR" "$READ_WRITE_DIR" \
  --unshare-user \
  --setenv READ_WRITE_DIR "$READ_WRITE_DIR" \
  --setenv ATTACK_TARGET "$ATTACK_TARGET" \
  /bin/sh -c '
    ln -s "$ATTACK_TARGET" "$READ_WRITE_DIR.link" && mv -fT "$READ_WRITE_DIR.link" "$READ_WRITE_DIR" && exit 7
    exit 0
  '
rc=$?

if [ "$rc" -eq 7 ] || [ -L "$READ_WRITE_DIR" ]; then echo "FAIL: grant redirected"; exit 1; fi
[ "$rc" -eq 0 ] || { echo "bwrap error (rc=$rc)" >&2; exit 2; }
echo "PASS: swap blocked"
