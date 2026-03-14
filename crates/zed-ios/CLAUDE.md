# crates/zed-ios/ — iOS Entry Point Crate

This crate produces a static library (`.a`) linked by the Swift host app in `ios/`.
It is the Rust-side entry point for everything iOS-specific that doesn't belong in GPUI.

## Responsibilities
- C FFI entry points (`zed_ios_main`, `zed_ios_open_window`, `zed_ios_close_window`)
- SSH key management via iOS Keychain (`keychain.rs`)
- Network connectivity monitoring via NWPathMonitor (`network_monitor.rs`)
- Embedded SSH transport using `russh` (`ssh_transport.rs`)
- Agent settings sync from remote host

## Build
```bash
cargo build -p zed-ios --target aarch64-apple-ios-sim --release --no-default-features --features ios
```

## This crate must NEVER
- Use `std::process::Command` or any subprocess spawning
- Access filesystem paths outside the app sandbox
- Depend on `node_runtime`, `lsp` (local), `task`, `dap`, `extension_host`, or `git` (CLI)
- Link AppKit or any macOS-only framework

## FFI pattern
All public functions use `#[no_mangle] pub extern "C" fn` with C-compatible types.
The Swift side imports them via `ZedApp-Bridging-Header.h`.
See `docs/ios-port-plan.md` "Rust ↔ iOS FFI Architecture" section for details.
