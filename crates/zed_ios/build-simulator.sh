#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
shell_directory="$repository_root/crates/zed_ios"

IPHONEOS_DEPLOYMENT_TARGET=15.0 \
    cargo build -p zed_ios --target aarch64-apple-ios-sim
(
    cd "$shell_directory/app"
    xcodegen generate
)
xcodebuild \
    -quiet \
    -project "$shell_directory/app/ZedIos.xcodeproj" \
    -scheme ZedIos \
    -sdk iphonesimulator \
    -configuration Debug \
    -derivedDataPath "$repository_root/target/zed-ios-xcode" \
    CODE_SIGNING_ALLOWED=NO \
    build

echo "App: $repository_root/target/zed-ios-xcode/Build/Products/Debug-iphonesimulator/ZedIos.app"
