# crates/gpui/src/platform/ios/ — GPUI iOS Platform Layer

This is the largest single engineering effort in the iPad port. Nothing else works
until this module renders pixels on an iPad screen.

## What to implement
Implement `Platform`, `PlatformWindow`, and `PlatformDisplay` traits for iOS.
Mirror the structure of `../mac/` — that's the closest reference (shared Metal + CoreText).

## Code reuse from platform/mac/
- **metal_renderer.rs**: ~70-80% reusable. Shaders are 100% portable.
  Delta: CVDisplayLink → CADisplayLink, NSView → UIView layerClass override.
- **text_system.rs**: ~85-90% reusable. Same CoreText API.
  Delta: font discovery (no /Library/Fonts on iOS), coordinate flip (top-left origin).
- **dispatcher.rs**: Similar shape. Replace CFRunLoop with DispatchQueue.main.async.

## Key differences from macOS
- UIApplication not NSApplication — do NOT link AppKit
- UITextInput protocol (full implementation) instead of NSTextInputClient
- CADisplayLink instead of CVDisplayLink for frame timing
- pressesBegan:withEvent: for hardware keyboard (catches keys UIKeyCommand misses)
- UIPointerInteraction for trackpad/mouse hover and click
- UIScene lifecycle for multi-window (Stage Manager)
- No swap space — aggressive Metal texture cache management required

## For full details
Read `docs/ios-port-plan.md` Phase 1 section.
