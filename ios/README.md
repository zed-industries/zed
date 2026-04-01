# Zed for iPad

Zed on iPad is a thin client: the iPad renders the UI locally via GPUI and Metal,
while all editing, language intelligence, terminals, and extensions run on a remote
host over SSH.

## Prerequisites

- macOS with Xcode 16+ installed
- Rust toolchain (`rustup`)
- iOS targets: `rustup target add aarch64-apple-ios aarch64-apple-ios-sim`
- An Apple Developer account (free or paid — needed for device signing)
- A remote host running `zed --headless` (your Mac, a Linux box, a cloud VM)

## Quick start

### Simulator

```bash
# Open in Xcode and hit Run (Cmd+R), selecting an iPad simulator
open ios/Zed.xcodeproj
```

Xcode's build phase script (`ios/script/cargo-build-ios`) handles the Rust
build automatically. The first build takes ~10 minutes; subsequent builds are
incremental (~30s for Rust, ~5s for Swift).

### Physical device

1. Open `ios/Zed.xcodeproj` in Xcode
2. Select your team in Signing & Capabilities (any Apple Developer account works)
3. Connect your iPad via USB or select it as a wireless destination
4. Hit Run (Cmd+R)

### Manual Rust build (optional)

If you want to check Rust compilation without Xcode:

```bash
# Simulator:
cargo check -p zed_ios --target aarch64-apple-ios-sim --no-default-features
# Device:
cargo check -p zed_ios --target aarch64-apple-ios --no-default-features
```

Do not pass `--features ios` — there is no such feature flag. iOS-specific code is
gated by `cfg(target_os = "ios")` automatically.

## Connecting to a remote host

On first launch you'll see the connection landing screen. Add your remote host
(hostname, port, username) and a project path. Zed connects over SSH using the
embedded `russh` library — no system `ssh` binary needed.

The remote host needs `zed --headless` running. On macOS or Linux:

```bash
# Install Zed CLI if you haven't
curl -f https://zed.dev/install.sh | sh

# The iPad will auto-install the headless server on first connect,
# or you can pre-install it:
zed --headless
```

SSH keys are loaded from `~/.ssh/` on the remote host. Password auth is also
supported (the iPad prompts interactively).

## Architecture

The app is a minimal Swift host that bootstraps UIKit and delegates into Rust:

```
Swift (UIKit lifecycle)  ->  C FFI  ->  Rust (crates/zed_ios, crates/gpui_ios)
```

`AppDelegate` and `SceneDelegate` handle the iOS scene lifecycle and call FFI
functions: `zed_ios_main()`, `zed_ios_open_window()`, `zed_ios_close_window()`.
Everything else — rendering, input, networking, editor logic — lives in Rust.

### Why a Swift host instead of pure Rust?

The macOS Zed app is built entirely in Rust. On iOS, a Swift host is needed
because asset catalogs (`actool`), code signing, and App Store distribution all
require Xcode's toolchain. The Swift host is three files (~60 lines). See
`ios/CLAUDE.md` for project structure details.

## Key directories

| Path | Purpose |
|---|---|
| `ios/` | Xcode project, Swift host app |
| `crates/zed_ios/` | Rust staticlib — app init, connection landing, edit prediction registry |
| `crates/gpui_ios/` | GPUI iOS platform layer — Metal renderer, CoreText, UIKit integration |
| `assets/keymaps/default-ios.json` | iPad keymap (standalone, not an overlay on macOS) |
| `ios/plan.md` | Full architecture and phase plan |
| `ios/checklist.md` | Working checklist of what's done and next |

## Editor setup for Swift files

For Swift language server support in Zed (UIKit completions, diagnostics):

```bash
brew install xcode-build-server
cd ios && xcode-build-server config -project Zed.xcodeproj -scheme Zed
```

Re-run after `xcodebuild clean` — clean wipes the build index that sourcekit-lsp reads.

## Asset changes

Assets (keymaps, themes, fonts) are embedded at Rust compile time via `RustEmbed`.
After changing an asset file, touch the embedding source to force a rebuild:

```bash
touch crates/assets/src/assets.rs
```

## Pushing files to device

```bash
# Pull settings from device:
xcrun devicectl device copy from --device <UDID> \
  --domain-type appDataContainer --domain-identifier dev.zed.ipad.app \
  --source "Library/Application Support/Zed/settings.json" \
  --destination /tmp/settings.json

# Push settings to device:
xcrun devicectl device copy to --device <UDID> \
  --domain-type appDataContainer --domain-identifier dev.zed.ipad.app \
  --source /tmp/settings.json \
  --destination "Library/Application Support/Zed/settings.json"
```
