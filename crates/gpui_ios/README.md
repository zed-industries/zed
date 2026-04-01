# gpui_ios — GPUI Platform Layer for iOS

Implements the GPUI `Platform`, `PlatformWindow`, `PlatformDisplay`, and
`PlatformTextSystem` traits for iPadOS. This is the iOS equivalent of
`crates/gpui_macos/`.

## What's here (~6K lines)

- `metal_renderer.rs` — Metal rendering (runtime shaders, MSAA, instance buffering)
- `text_system.rs` — CoreText font loading, shaping, glyph rasterization
- `window.rs` — UIKit window, touch/keyboard/trackpad input, UIPointerInteraction
- `dispatcher.rs` — GCD-backed foreground/background executors
- `display.rs` — UIScreen queries, scale factor
- `display_link.rs` — CADisplayLink vsync driver
- `platform.rs` — Platform trait impl (clipboard, dark mode, prompts, thermal state)
- `open_type.rs` — OpenType feature queries via CoreText

## Why a separate crate?

The `gpui` crate selects its platform backend via `cfg(target_os)` in
`src/platform/mod.rs`. On iOS, it re-exports types from this crate. Keeping the
iOS implementation separate avoids polluting `gpui`'s build with iOS-specific
dependencies and build scripts.

## See also

- `ios/plan.md` — Phase 1 covers the platform layer in detail
- `crates/gpui_macos/` — macOS sibling (AppKit + Metal)
