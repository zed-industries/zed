//! GPUI iOS platform implementation.
//!
//! The iOS platform implementation lives in the `gpui_ios` crate
//! (`crates/gpui_ios/`) following the same split as `gpui_macos`, `gpui_linux`,
//! and `gpui_windows`. The split keeps platform-specific native dependencies out
//! of the core `gpui` crate.
//!
//! Entry point: `gpui_ios::IosPlatform` (wired into `gpui_platform::current_platform`).
//!
//! Module structure in `gpui_ios`:
//!   platform.rs        — IosPlatform : Platform (~40+ methods)
//!   window.rs          — IosWindow : PlatformWindow (~37+ methods)
//!   display.rs         — IosDisplay : PlatformDisplay (4 methods)
//!   dispatcher.rs      — GCD-backed foreground/background executor
//!   keyboard.rs        — IosKeyboardLayout stub; DummyKeyboardMapper
//!   events.rs          — UIEvent / UITouch / UIKey → PlatformInput (Phase 1.3)
//!
//! Reference implementations:
//!   gpui_macos/        — Closest code to fork (shares Metal + CoreText)
//!   gpui_web/          — Minimal Platform implementation, async patterns
//!
//! See: docs/ios-port-plan.md Phase 1 for full details.
