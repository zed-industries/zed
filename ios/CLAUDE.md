# ios/ — Zed iPad App (Swift Host)

This directory contains the Xcode project and Swift source for the Zed iPad app.
The Swift layer is intentionally thin — it bootstraps UIKit, creates scenes, and
calls into Rust via C FFI. All substantive logic lives in `crates/zed-ios/` and
`crates/gpui_ios/`.

## Bundle ID

`dev.zed.ipad.app`

## Build

The iOS build is a two-step process: build the Rust static library, then build
the Swift app in Xcode which links against it.

```bash
# 1. Build the Rust static library
# Device:
cargo build -p zed-ios --target aarch64-apple-ios --release --no-default-features
# Simulator:
cargo build -p zed-ios --target aarch64-apple-ios-sim --release --no-default-features

# 2. Build the Swift app (Xcode picks up the .a from target/)
# Device: build+run from Xcode with a connected device and valid signing
# Simulator:
xcodebuild -project ios/Zed.xcodeproj -scheme Zed \
  -destination 'platform=iOS Simulator,name=iPad Pro 13-inch (M5)' build
```

Do not pass `--features ios` — there is no such feature flag. iOS-specific code
is gated by `cfg(target_os = "ios")` automatically when targeting `aarch64-apple-ios*`.

### Stale binary warning

Xcode incremental builds can silently use a stale Rust `.a` file if the Rust build
fails but Xcode's build system doesn't detect the change. If behavior doesn't match
your code changes, do a clean Rust build before rebuilding in Xcode.

### Asset changes

`RustEmbed` embeds assets (keymaps, themes, fonts) at Rust compile time. When you
change an asset file (e.g. `assets/keymaps/default-ios.json`), touch the embedding
source to force a rebuild:
```bash
touch crates/assets/src/assets.rs
```

## Xcode project structure

```
Zed/
  AppDelegate.swift          — UIApplicationDelegate, lifecycle hooks
  SceneDelegate.swift        — UIWindowSceneDelegate, creates GPUI windows via FFI
  Supporting Files/
    Info.plist               — UIApplicationSceneManifest, deployment target
    Zed-Bridging-Header.h   — C FFI declarations for Rust entry points
  Entitlements/
    Zed.entitlements         — keychain-access-groups
```

## Swift LSP (sourcekit-lsp / xcode-build-server)

UIKit errors in Swift files mean sourcekit-lsp is using the macOS SDK.
Fix: re-run `xcode-build-server config` from `ios/`, then restart the language server.

```bash
cd ios && xcode-build-server config -project Zed.xcodeproj -scheme Zed
```

**Always re-run this after `xcodebuild clean`** — clean wipes the build index
that the BSP reads. Prefer `build` over `clean build` unless a clean is needed.

## Swift conventions

- Deployment target: **iPadOS 17.0**
- Swift 6 strict concurrency where possible
- All UI work on `@MainActor`
- Bridge to Rust exclusively via C FFI (`pub extern "C" fn` in Rust, bridging header in Swift)
- Never use `Process()` / `NSTask` — iOS prohibits subprocess spawning

## FFI entry points

The bridging header declares these Rust functions:
- `zed_ios_main()` — initialize GPUI and Zed, called from AppDelegate
- `zed_ios_open_window(scene_id)` — create a GPUI window for a UIWindowScene
- `zed_ios_close_window(scene_id)` — tear down a GPUI window
- `zed_ios_build_menus(builder)` — populate iPadOS menu bar
- `zed_ios_will_resign_active()` — persist SSH sessions before backgrounding

## Scene lifecycle

Each `UIWindowScene` = one Zed workspace connection. `SceneDelegate` calls
`zed_ios_open_window(scene_id)` on activation. `UIApplicationSupportsMultipleScenes = YES`
in Info.plist for Stage Manager multi-window support.

## Fonts

Fonts are embedded in the Rust static library via `RustEmbed` (the `assets` crate).
Loaded by `assets::Assets.load_fonts(cx)` during `init_zed()`. No `UIAppFonts` in
Info.plist or bundle resource copying needed.

## Pushing files to device

Use `xcrun devicectl` to push/pull files from the app container:
```bash
# Pull:
xcrun devicectl device copy from --device <UDID> \
  --domain-type appDataContainer --domain-identifier dev.zed.ipad.app \
  --source "Library/Application Support/Zed/settings.json" \
  --destination /tmp/settings.json

# Push:
xcrun devicectl device copy to --device <UDID> \
  --domain-type appDataContainer --domain-identifier dev.zed.ipad.app \
  --source /tmp/settings.json \
  --destination "Library/Application Support/Zed/settings.json"
```
