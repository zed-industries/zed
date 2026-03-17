# crates/gpui/src/platform/ios/ — Stub redirect

The iOS platform layer does **not** live here. It is in a separate crate:

**`crates/gpui_ios/`** (~4900 lines, Phase 1 COMPLETE)

This directory only contains a `mod.rs` stub that re-exports the `gpui_ios` crate's
types so that `gpui` can reference them via `platform::ios::*`.

## What's implemented in gpui_ios

- Metal renderer (runtime shaders, MSAA, instance buffering) — `metal_renderer.rs`
- CoreText text system (font loading, shaping, glyph rasterization) — `text_system.rs`
- UIKit window (touch input, hardware/software keyboard, trackpad scroll) — `window.rs`
- GCD dispatcher (foreground/background, timers) — `dispatcher.rs`
- Display (UIScreen queries, scale factor) — `display.rs`
- CADisplayLink vsync driver — `display_link.rs`
- Platform trait impl (clipboard, dark mode, prompts, open_url, thermal state) — `platform.rs`

## Why a separate crate?

The `gpui_ios` crate has a `build.rs` that runs `cbindgen` to generate the C header
for the Swift bridging layer. Keeping it separate avoids polluting `gpui`'s build
with iOS-specific build scripts and dependencies (Metal, CoreText, UIKit via objc).
