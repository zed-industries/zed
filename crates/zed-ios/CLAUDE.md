# crates/zed-ios/ — iOS Entry Point Crate

This crate produces a static library (`.a`) linked by the Swift host app in `ios/`.
It is the Rust-side entry point for everything iOS-specific that doesn't belong in GPUI.

## Current state

- `ios_main()` initializes GPUI with the iOS platform + embedded assets, then boots the
  real Zed workspace (settings, themes, fonts, client, workspace) via `init_zed()`.
- `Session::new` is async, so workspace window opening is deferred to a `cx.spawn`.
- The old `TextSmokeView` demo is still in the file but unused.
- `keychain.rs`, `network_monitor.rs`, `ssh_transport.rs` are planned for Phase 2 (commented out).

## Responsibilities

- C FFI entry points (`zed_ios_main`, `zed_ios_open_window`, `zed_ios_close_window`)
- Full Zed app initialization (settings, theme, fonts, client, workspace)
- (Phase 2) SSH key management via iOS Keychain
- (Phase 2) Network connectivity monitoring via NWPathMonitor
- (Phase 2) Embedded SSH transport using `russh`

## Build

```bash
# Simulator:
cargo build -p zed-ios --target aarch64-apple-ios-sim --release --no-default-features
# Device:
cargo build -p zed-ios --target aarch64-apple-ios --release --no-default-features
```

Note: do not pass `--features ios` — there is no such feature flag; iOS-specific code
is gated by `cfg(target_os = "ios")` automatically when targeting `aarch64-apple-ios*`.

## This crate must NEVER

- Use `std::process::Command` or any subprocess spawning
- Access filesystem paths outside the app sandbox
- Depend on `node_runtime`, `lsp` (local), `task`, `dap`, `extension_host`, or `git` (CLI)
- Link AppKit or any macOS-only framework

## FFI pattern

All public functions use `#[no_mangle] pub extern "C" fn` with C-compatible types.
The Swift side imports them via `Zed-Bridging-Header.h`.
See `docs/ios-port-plan.md` for full architecture details.
