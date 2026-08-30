#!/usr/bin/env bash

# Builds the example signed for a physical iOS/iPadOS device.
# Requires an Apple Development certificate; pass the team via
# DEVELOPMENT_TEAM (defaults to the first team found in the keychain).

set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
example_directory="$repository_root/crates/gpui_ios/examples/ios"

development_team="${DEVELOPMENT_TEAM:-$(
    security find-certificate -c "Apple Development" -p \
        | openssl x509 -noout -subject \
        | sed -n 's/.*OU *= *\([A-Z0-9]*\).*/\1/p'
)}"

if [ -z "$development_team" ]; then
    echo "No Apple Development certificate found; set DEVELOPMENT_TEAM." >&2
    exit 1
fi

IPHONEOS_DEPLOYMENT_TARGET=15.0 \
    cargo build -p gpui_ios_example --target aarch64-apple-ios
(
    cd "$example_directory/app"
    xcodegen generate
)
xcodebuild \
    -quiet \
    -project "$example_directory/app/GPUIIosExample.xcodeproj" \
    -scheme GPUIIosExample \
    -sdk iphoneos \
    -configuration Debug \
    -derivedDataPath "$repository_root/target/gpui-ios-example-xcode-device" \
    -allowProvisioningUpdates \
    -allowProvisioningDeviceRegistration \
    CODE_SIGN_STYLE=Automatic \
    DEVELOPMENT_TEAM="$development_team" \
    build

echo "App: $repository_root/target/gpui-ios-example-xcode-device/Build/Products/Debug-iphoneos/GPUIIosExample.app"
echo "Install with: xcrun devicectl device install app --device <UDID> <app path>"
