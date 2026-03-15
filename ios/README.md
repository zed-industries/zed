# Zed for iPad

This directory contains the iOS host app for Zed on iPadOS. Zed on iPad is a
thin client: all editing and language intelligence runs on a remote host (your
Mac, a cloud VM, etc.) over SSH. The iPad renders the UI locally via GPUI and
Metal.

## Architecture

The app is structured as a minimal Swift host that bootstraps UIKit and
delegates immediately into Rust:

```
Swift (UIKit lifecycle)  →  C FFI  →  Rust (crates/zed-ios, crates/gpui)
```

`AppDelegate` and `SceneDelegate` handle the iOS scene lifecycle and call three
FFI functions: `zed_ios_main()`, `zed_ios_open_window()`, and
`zed_ios_close_window()`. Everything else — rendering, input, networking, editor
logic — lives in Rust.

## Why a Swift host instead of pure Rust?

The macOS Zed app is built entirely in Rust: GPUI uses the `objc` crate to
register `NSApplication` subclasses at runtime, and `cargo-bundle` assembles the
`.app` without Xcode. We considered the same approach for iOS.

The short answer is that the iOS toolchain makes this impractical:

- **Asset catalogs** — `actool` produces a proprietary compiled `.car` format.
  No open-source tool can generate it. Even a pure-Rust build needs to shell out
  to Apple's toolchain for this step.
- **App distribution** — App Store and TestFlight require a signed `.ipa`
  produced by `xcodebuild archive`. There is no Cargo-native path to a
  distributable iOS build.
- **Code signing** — iOS provisioning profiles, entitlements, and device
  registration are significantly more complex than macOS signing. Xcode manages
  this; reimplementing it in a build script is not worthwhile.
- **`cargo-bundle` has no iOS support** — and tools like `cargo-mobile2`
  (which Tauri uses) don't bypass Xcode either. They automate generating
  `project.pbxproj` and then call `xcodebuild`. We wrote that file once; it's
  done.

The Swift host is three files and about 60 lines of code. The maintenance burden
is negligible compared to what would be required to replicate the iOS build
pipeline without Xcode.

## Editor setup

For Swift language server support (UIKit completions, error highlighting) in Zed,
install `xcode-build-server` and run it once from the `ios/` directory:

```bash
brew install xcode-build-server
cd ios && xcode-build-server config -project Zed.xcodeproj -scheme Zed
```

This generates `ios/buildServer.json` (gitignored) which tells sourcekit-lsp to
use the iOS SDK instead of the macOS SDK.

**Re-run after any `clean build`**: `xcodebuild clean` wipes the build index
that sourcekit-lsp reads. If UIKit errors reappear in Swift files, run
`xcode-build-server config` again from `ios/`, then restart the language server
in your editor.

## Building

The Xcode project drives the full build. The `cargo build` step is wired in as
a shell script build phase that runs before Swift compilation.

```bash
# Build and run on the iPad Pro simulator
xcodebuild -project ios/Zed.xcodeproj -scheme Zed \
  -destination 'platform=iOS Simulator,name=iPad Pro 13-inch (M5)' \
  build

# For a physical device, open Zed.xcodeproj in Xcode and run from there
# (requires an Apple Developer account for code signing)
```

The Rust static library is built as part of the Xcode build phase — you do not
need to run `cargo build` separately unless you want to check for compile errors
without going through Xcode.

See `CLAUDE.md` in this directory for agent-facing build details and Swift
conventions.

## Project plan

See `docs/ios-port-plan.md` in the repository root for the full architecture,
phase plan, SSH transport design, and technical specifics.
