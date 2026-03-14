//! GPUI iOS platform implementation.
//!
//! This module implements the `Platform`, `PlatformWindow`, and `PlatformDisplay`
//! traits for iPadOS, using UIKit + Metal + CoreText.
//!
//! Module structure mirrors `platform/mac/`:
//!   platform.rs        — IosPlatform : Platform (~40+ methods)
//!   window.rs          — IosWindow : PlatformWindow (~37+ methods)
//!   display.rs         — IosDisplay : PlatformDisplay (4 methods)
//!   dispatcher.rs      — GCD-backed foreground/background executor
//!   text_system.rs     — CoreText font loading + CTTypesetter shaping
//!   events.rs          — UIEvent / UITouch / UIKey → PlatformInput mapping
//!   metal_renderer.rs  — Thin wrapper reusing mac/metal_renderer.rs logic
//!
//! Reference implementations:
//!   platform/mac/   — Closest code to fork (shares Metal + CoreText)
//!   platform/wasm/  — Minimal Platform implementation, async patterns
//!
//! Third-party reference:
//!   github.com/itsbalamurali/gpui-mobile — Community iOS GPUI implementation
//!
//! See: docs/ios-port-plan.md Phase 1 for full details.

// TODO Phase 1: Implement each submodule.
// mod platform;
// mod window;
// mod display;
// mod dispatcher;
// mod text_system;
// mod events;
// mod metal_renderer;
