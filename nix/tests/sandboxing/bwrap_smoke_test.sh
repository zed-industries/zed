#!/usr/bin/env bash
# Ad-hoc smoke test for the static `bwrap` binary. Exercises filesystem
# read/write isolation and network isolation by trying to "break out" of the
# sandbox in ways that must fail, alongside controls that must succeed.
#
# Usage: bwrap_smoke_test.sh /path/to/bwrap
set -u

BWRAP=${1:?usage: bwrap_smoke_test.sh /path/to/bwrap}

# Resolve real binaries (works on NixOS, where /bin is mostly empty).
CAT=$(command -v cat)
TOUCH=$(command -v touch)
SH=$(command -v sh)
PY=$(command -v python3)

# Bind enough of the host (read-only) for dynamically-linked helpers to run.
# On NixOS that means the closure under /nix/store; /usr/bin elsewhere.
BASE=(--ro-bind /nix /nix --ro-bind /run /run --proc /proc --dev /dev --unshare-user)
[ -d /usr ] && BASE+=(--ro-bind /usr /usr)
[ -e /bin ] && BASE+=(--ro-bind /bin /bin)
[ -e /lib ] && BASE+=(--ro-bind /lib /lib)
[ -e /lib64 ] && BASE+=(--ro-bind /lib64 /lib64)
[ -e /etc ] && BASE+=(--ro-bind /etc /etc)

PASS=0
FAIL=0
ok()   { echo "PASS: $1"; PASS=$((PASS + 1)); }
bad()  { echo "FAIL: $1"; FAIL=$((FAIL + 1)); }

# expect_ok  <desc> <cmd...>   -> command must exit 0
expect_ok() {
  desc=$1; shift
  if "$@"; then ok "$desc"; else bad "$desc (expected success, got exit $?)"; fi
}
# expect_fail <desc> <cmd...>  -> command must exit non-zero
expect_fail() {
  desc=$1; shift
  if "$@"; then bad "$desc (expected failure, but it succeeded)"; else ok "$desc"; fi
}

# ---------------------------------------------------------------------------
# Set up a host scratch tree:
#   $ROOT/secret/secret.txt   -> must NOT be visible unless explicitly bound
#   $ROOT/allowed/note.txt    -> bound read-only into the sandbox
#   $ROOT/writable/           -> bound read-write into the sandbox
# ---------------------------------------------------------------------------
ROOT=$(mktemp -d /tmp/bwrap-smoke.XXXXXX)
trap 'rm -rf "$ROOT"' EXIT
mkdir -p "$ROOT/secret" "$ROOT/allowed" "$ROOT/writable"
echo "TOP SECRET" > "$ROOT/secret/secret.txt"
echo "you may read me" > "$ROOT/allowed/note.txt"

echo "=================================================================="
echo "bwrap: $BWRAP"
"$BWRAP" --version
echo "=================================================================="
echo "## Filesystem isolation"

# The host secret must be invisible: sandbox sees a fresh tmpfs at $ROOT,
# so the secret path simply doesn't exist.
expect_fail "host secret file is NOT readable from sandbox" \
  "$BWRAP" "${BASE[@]}" --tmpfs "$ROOT" "$CAT" "$ROOT/secret/secret.txt"

# Likewise nothing under /tmp leaks in by default (no bind for it).
expect_fail "host /tmp is NOT visible from sandbox" \
  "$BWRAP" "${BASE[@]}" --tmpfs /tmp "$CAT" "$ROOT/allowed/note.txt"

# An explicitly read-only bind IS readable...
expect_ok "explicitly ro-bound file IS readable" \
  "$BWRAP" "${BASE[@]}" --ro-bind "$ROOT/allowed" /data "$CAT" /data/note.txt

# ...but writing to a read-only bind must fail.
expect_fail "writing to a ro-bound dir is denied" \
  "$BWRAP" "${BASE[@]}" --ro-bind "$ROOT/allowed" /data "$TOUCH" /data/evil

# Confirm the host file was not modified/created behind the sandbox's back.
expect_fail "ro-bind write did not leak to host" test -e "$ROOT/allowed/evil"

# A read-write bind allows writes...
expect_ok "writing to a rw-bound dir succeeds" \
  "$BWRAP" "${BASE[@]}" --bind "$ROOT/writable" /data "$TOUCH" /data/hello

# ...and the write really lands on the host file (proves the bind, not a tmpfs).
expect_ok "rw-bind write is visible on host" test -e "$ROOT/writable/hello"

# Writing to a tmpfs overlay succeeds inside, but must NOT touch the host dir.
expect_ok "writing into a tmpfs works inside sandbox" \
  "$BWRAP" "${BASE[@]}" --tmpfs "$ROOT/writable" "$TOUCH" "$ROOT/writable/ephemeral"
expect_fail "tmpfs write did not leak to host" test -e "$ROOT/writable/ephemeral"

echo "------------------------------------------------------------------"
echo "## Network isolation"

NETCHECK='import socket; s=socket.socket(socket.AF_INET,socket.SOCK_STREAM); s.settimeout(5); s.connect(("1.1.1.1",53)); print("connected")'

# With --unshare-net the sandbox has only loopback: outbound TCP must fail.
expect_fail "outbound TCP is blocked with --unshare-net" \
  "$BWRAP" "${BASE[@]}" --unshare-net "$PY" -c "$NETCHECK"

# Loopback still exists inside the isolated netns.
expect_ok "loopback is up inside --unshare-net" \
  "$BWRAP" "${BASE[@]}" --unshare-net "$PY" -c \
    'import socket; socket.socket().bind(("127.0.0.1",0)); print("lo ok")'

# Without --unshare-net the host network is shared, so the same connect works
# (skipped automatically if the host itself has no outbound connectivity).
if timeout 6 "$PY" -c "$NETCHECK" >/dev/null 2>&1; then
  expect_ok "outbound TCP works when network is shared" \
    "$BWRAP" "${BASE[@]}" "$PY" -c "$NETCHECK"
else
  echo "SKIP: host has no outbound TCP; cannot test shared-network case"
fi

echo "=================================================================="
echo "Summary: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
