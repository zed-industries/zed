# Crash Analysis: GPU surface validation failed when opening window with zero-size dimensions

## Crash Summary
- **Sentry Issue:** ZED-4HJ (https://sentry.io/organizations/zed-dev/issues/7204144901/)
- **Error:** `called Result<ip>unwrap() on an Err value: "validation failed"`
- **Crash Site:** `blade_graphics::hal::surface::<T>::reconfigure_surface`
- **Event Count:** 672 occurrences
- **First Seen:** 2025-12-28
- **Last Seen:** 2026-02-24
- **Affected Version:** 0.224.11+stable (Linux/Wayland with AMD Radeon Graphics)

## Root Cause

The crash occurs when creating a GPU surface with invalid dimensions (width or height of 0). GPU APIs require surface dimensions to be at least 1x1 pixel. The validation failure happens during window creation when:

1. A user opens a new window (e.g., the Settings window via `open_settings_editor`)
2. The window bounds are calculated, potentially resulting in 0-width or 0-height dimensions
3. The renderer creates a GPU surface with these invalid dimensions
4. The GPU driver rejects the surface configuration with "validation failed"
5. The old blade_graphics code called `unwrap()` on this error, causing a panic

The crash flow:
```
settings_ui::open_settings_editor
→ gpui::app::App::open_window
→ gpui::platform::linux::wayland::window::WaylandWindow::new
→ WaylandWindowState::new
→ BladeRenderer::new
→ create_surface_configured
→ reconfigure_surface  ← panics on validation error
```

This was an issue in the blade_graphics dependency which used `unwrap()` internally when configuring surfaces, rather than properly propagating errors.

## Status

**This crash has been addressed in the current codebase.** The blade_graphics renderer was completely replaced with a wgpu-based renderer in commit `af8ea0d6c2` (2026-02-13):

> gpui: Remove blade, reimplement linux renderer with wgpu (#46758)

The new wgpu implementation properly handles edge cases:
1. `update_drawable_size()` uses `.max(1)` to ensure minimum dimensions
2. Texture creation uses `.max(1)` for width and height
3. Surface errors are handled gracefully rather than panicking

However, there is a minor remaining issue: In `WgpuRenderer::new()`, the initial surface configuration doesn't apply the `.max(1)` safeguard that `update_drawable_size()` uses. While this is unlikely to cause crashes in practice (wgpu handles this more gracefully than blade did), it should be fixed for consistency.

## Reproduction

The original crash cannot be reproduced in the current codebase because:
1. The blade_graphics dependency has been removed
2. The wgpu renderer handles validation errors gracefully

A reproduction test would need to:
1. Create a window with 0x0 or very small dimensions
2. Attempt to create a GPU surface
3. Verify the error is handled without panicking

## Suggested Fix

1. **For stable users (0.224.x):** Upgrade to a version with the wgpu renderer (post-0.225.x)

2. **For current codebase:** Add a defensive `.max(1)` check in `WgpuRenderer::new()` for consistency with `update_drawable_size()`:

```rust
// In WgpuRenderer::new():
let surface_config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface_format,
    width: (config.size.width.0 as u32).max(1),   // Add .max(1)
    height: (config.size.height.0 as u32).max(1), // Add .max(1)
    // ...
};
```

This makes the initialization consistent with the resize handling and prevents any edge cases where invalid initial dimensions could cause issues.

## Test Command

```
cargo test -p gpui_wgpu
```

Note: A full reproduction test for this GPU-level issue would require mocking the GPU context, which is complex. The fix is defensive and ensures consistency with existing patterns.
