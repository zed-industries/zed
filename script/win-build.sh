#!/usr/bin/env bash
#
# win-build.sh -- release build of Zed on Windows.
#
# Delegates into cmd so that script/win-setenv.cmd stays the single source of
# truth for the build environment. Importing the environment the other way
# round (parsing `set` output back into bash) does not survive variable names
# like ProgramFiles(x86) or the ';'-separated Windows PATH, so we run the build
# inside the cmd session that win-setenv.cmd just configured.
#
# Run from Git Bash:
#   script/win-build.sh                  # cargo build --release
#   script/win-build.sh --run            # ...then launch the binary
#   script/win-build.sh --locked         # extra args are forwarded to cargo
#
# The workspace sets default-members = ["crates/zed"], so a bare
# `cargo build --release` already builds just the editor.
#
# Companion scripts: script/win-init.sh (one-time toolchain install),
#                    script/win-setenv.cmd (env only).

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

RUN_AFTER=0
CARGO_ARGS=()

usage() {
  sed -n '3,20p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [ $# -gt 0 ]; do
  case "$1" in
    --run)
      RUN_AFTER=1
      shift
      ;;
    --help | -h)
      usage
      exit 0
      ;;
    --)
      shift
      CARGO_ARGS+=("$@")
      break
      ;;
    *)
      CARGO_ARGS+=("$1")
      shift
      ;;
  esac
done

info() { printf '[win-build] %s\n' "$*"; }

# Quote an argument for cmd.exe. Only needed for args carrying spaces or quotes.
cmd_quote() {
  case "$1" in
    *[[:space:]\"]*) printf '"%s"' "${1//\"/\\\"}" ;;
    *) printf '%s' "$1" ;;
  esac
}

CARGO_CMD="cargo build --release"
for arg in ${CARGO_ARGS[@]+"${CARGO_ARGS[@]}"}; do
  CARGO_CMD="$CARGO_CMD $(cmd_quote "$arg")"
done

info "Repository: $REPO_ROOT"
info "Command:    $CARGO_CMD"
echo

# `//c` rather than `/c`: under MSYS/Git Bash a lone `/c` is path-mangled into
# `C:/`. The doubled slash is passed through as a literal `/c`.
# `call` so that win-setenv.cmd returns control and && sees its exit code.
cmd //c "call script\\win-setenv.cmd && $CARGO_CMD"

BINARY="target/release/zed.exe"
echo
if [ -f "$BINARY" ]; then
  info "Build succeeded: $REPO_ROOT/$BINARY"
  if [ "$RUN_AFTER" -eq 1 ]; then
    info "Launching..."
    exec "$BINARY"
  fi
else
  # A build that produced no zed.exe but exited 0 means the args pointed
  # somewhere else (e.g. -p some-other-crate); say so rather than claiming
  # success for a file that is not there.
  info "Build finished, but $BINARY was not produced."
  info "If you passed -p/--target, look under target/ for the relevant output."
fi
