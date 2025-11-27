#!/bin/bash
# Build and run GPUI iOS example on iOS Simulator
#
# Usage: ./build_and_run.sh [device_name]
# Example: ./build_and_run.sh "iPhone 15 Pro"

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
APP_NAME="gpui_ios_example"
BUNDLE_ID="dev.zed.gpui-ios-example"

# Default to iPhone 15 Pro simulator if not specified
DEVICE_NAME="${1:-iPhone 15 Pro}"

echo "Building GPUI for iOS simulator..."
cd "$PROJECT_ROOT"

# Build the Rust library for iOS simulator
cargo build --target aarch64-apple-ios-sim -p gpui --features font-kit --release

echo "Build completed successfully!"

# The binary is at:
BINARY_PATH="$PROJECT_ROOT/target/aarch64-apple-ios-sim/release/libgpui.a"

if [ -f "$BINARY_PATH" ]; then
    echo "Library built at: $BINARY_PATH"
    echo ""
    echo "To run on iOS simulator, you need to:"
    echo "1. Create an Xcode project that links against this library"
    echo "2. Add the Info.plist and LaunchScreen.storyboard from this directory"
    echo "3. Implement a main() that calls GPUI's Application::new().run()"
    echo ""
    echo "Or use cargo-bundle to create an iOS app bundle:"
    echo "  cargo install cargo-bundle"
    echo "  cargo bundle --target aarch64-apple-ios-sim"
else
    echo "Warning: Expected library not found at $BINARY_PATH"
    echo "Check build output for errors."
fi

echo ""
echo "Available iOS simulators:"
xcrun simctl list devices available | grep -E "iPhone|iPad" | head -10
