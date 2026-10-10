#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
example_directory="$repository_root/crates/gpui_ios/examples/ios"
target_directory="${CARGO_TARGET_DIR:-"$repository_root/target"}"
if [[ "$target_directory" != /* ]]; then
    target_directory="$repository_root/$target_directory"
fi
app_directory="$target_directory/gpui-ios-example/GPUIIosExample.app"

cd "$repository_root"
mkdir -p "$target_directory"
build_output="$(mktemp "$target_directory/gpui-ios-build.XXXXXX")"
trap 'rm -f "$build_output"' EXIT

# Corgi does not currently support iOS targets. Extra arguments are passed to Cargo.
IPHONEOS_DEPLOYMENT_TARGET=15.0 \
    cargo build -p gpui_ios_example --target aarch64-apple-ios-sim \
    --message-format=json-render-diagnostics "$@" > "$build_output"

executable="$(python3 - "$build_output" <<'PY'
import json
import sys

with open(sys.argv[1]) as output:
    executables = [
        event["executable"]
        for event in map(json.loads, output)
        if event.get("reason") == "compiler-artifact"
        and event.get("target", {}).get("name") == "gpui_ios_example"
        and event.get("executable")
    ]
if len(executables) != 1:
    sys.exit("Expected one gpui_ios_example executable in Cargo's build output")
print(executables[0])
PY
)"
mkdir -p "$app_directory"
cp "$executable" "$app_directory/gpui_ios_example"
cp "$example_directory/app/Info.plist" "$app_directory/Info.plist"
codesign --force --sign - "$app_directory"
printf 'Built simulator app: %s\n' "$app_directory"
