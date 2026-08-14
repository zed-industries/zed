#!/usr/bin/env bash

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
example_directory="$repository_root/crates/gpui_ios/examples/ios"

IPHONEOS_DEPLOYMENT_TARGET=15.0 \
    cargo build -p gpui_ios_example --target aarch64-apple-ios-sim
(
    cd "$example_directory/app"
    xcodegen generate
)
xcodebuild \
    -quiet \
    -project "$example_directory/app/GPUIIosExample.xcodeproj" \
    -scheme GPUIIosExample \
    -sdk iphonesimulator \
    -configuration Debug \
    -derivedDataPath "$repository_root/target/gpui-ios-example-xcode" \
    CODE_SIGNING_ALLOWED=NO \
    build
