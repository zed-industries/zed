# gpui_wgpu: Ensure minimum surface dimensions during initialization

## Crash Summary

**Sentry Issue:** [ZED-4HJ](https://sentry.io/organizations/zed-dev/issues/7204144901/) (672 events)

The crash occurred when opening a new window (e.g., the Settings window) on Linux/Wayland with AMD Radeon Graphics. The crash message was:

```
called Result<ip>unwrap() on an Err value: "validation failed"
```

The crash happened in the blade_graphics library when configuring a GPU surface with invalid dimensions (width or height of 0), which caused GPU validation to fail.

## Root Cause

GPU surfaces require dimensions of at least 1x1 pixel. The blade_graphics library internally used `unwrap()` on the surface validation result, which panicked when the GPU driver rejected the configuration with "validation failed".

The primary fix was already applied in #46758 (merged 2026-02-13) which replaced blade_graphics with wgpu. The wgpu implementation handles surface errors more gracefully.

## Fix

This PR adds a defensive safeguard to ensure the wgpu renderer's initial surface configuration uses dimensions of at least 1x1, consistent with the existing pattern in `update_drawable_size()`:

```rust
// Before
width: config.size.width.0 as u32,
height: config.size.height.0 as u32,

// After
width: (config.size.width.0 as u32).max(1),
height: (config.size.height.0 as u32).max(1),
```

This matches the existing defensive pattern already used in:
- `update_drawable_size()` for resizing
- `create_path_intermediate()` for texture creation
- `create_msaa_if_needed()` for MSAA texture creation

## Validation

- [x] `cargo check -p gpui_wgpu` passes
- [x] `cargo clippy -p gpui_wgpu` passes with no warnings
- [x] `cargo test -p gpui_wgpu` passes (1 test)

## Potentially Related Issues

### High Confidence
- [#46758](https://github.com/zed-industries/zed/pull/46758) — gpui: Remove blade, reimplement linux renderer with wgpu (already merged, primary fix)

### Medium Confidence
- None

### Low Confidence
- [#43070](https://github.com/zed-industries/zed/pull/43070) — gpui: Implement GPU device loss recovery for Linux X11
- [#46281](https://github.com/zed-industries/zed/pull/46281) — Use transparent clear color for opaque windows on Linux

## Reviewer Checklist

- [ ] Confirm the `.max(1)` change matches the existing pattern in `update_drawable_size()`
- [ ] Verify this is a defensive change that doesn't alter normal behavior
- [ ] Confirm users on stable 0.224.x should upgrade to get the blade-to-wgpu fix

## Note for Stable Users

Users experiencing this crash on version 0.224.11 should upgrade to a version that includes PR #46758 (the blade-to-wgpu migration), which provides the primary fix for this issue.

Release Notes:

- Fixed a potential crash when creating GPU surfaces with zero-size dimensions on Linux
