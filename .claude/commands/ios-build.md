Build the Zed iOS target. Run these steps in order, stopping on the first failure:

1. Ensure iOS targets are installed:
   ```
   rustup target add aarch64-apple-ios aarch64-apple-ios-sim
   ```

2. Build the Rust static library for simulator:
   ```
   cargo build -p zed-ios --target aarch64-apple-ios-sim --release --no-default-features --features ios
   ```

3. Build the Xcode project for simulator:
   ```
   xcodebuild -project ios/ZedApp.xcodeproj -scheme ZedApp -destination 'platform=iOS Simulator,name=iPad Pro 13-inch (M4)' build
   ```

If step 2 fails with compilation errors, check for:
- Missing `#[cfg(target_os = "ios")]` gates on platform-specific code
- Crates that should be excluded from the iOS build (node_runtime, lsp local spawn, task, dap, extension_host, git CLI)
- build.rs scripts with hardcoded `-sdk macosx` that need iOS SDK paths

If $ARGUMENTS contains "device", replace step 2 target with `aarch64-apple-ios` and skip step 3 (device builds require Xcode signing).

Report the result of each step.
