# Platform Crates Refactor Plan

## Goal

Split GPUI's platform implementations (macOS, Windows, Linux) into their own
crates to optimize compile times. Currently, any change to platform-specific
code (e.g. the Metal renderer) triggers a recompile of all of GPUI and
everything downstream. After this refactor, platform implementation changes only
recompile the affected platform crate and the final binary link.

## Current State

All platform code lives inside `crates/gpui/src/platform/`:

```
platform/
├── app_menu.rs          # cross-platform
├── keyboard.rs          # cross-platform
├── keystroke.rs         # cross-platform
├── mac/                 # macOS: Metal renderer, CoreText, cocoa bindings
├── linux/               # Linux: wayland/, x11/, headless/
├── windows/             # Windows: DirectX renderer, DirectWrite
├── blade/               # Shared GPU renderer (Linux + macOS-blade)
├── test/                # Test platform (TestPlatform, TestWindow, etc.)
├── visual_test.rs       # macOS-only, wraps MacPlatform with TestDispatcher
├── scap_screen_capture.rs  # Shared screen capture (Linux + Windows)
└── platform.rs          # Trait definitions + shared types
```

Key problems:
- `PaintSurface` in `scene.rs` has `#[cfg(target_os = "macos")] pub image_buffer: CVPixelBuffer`
- `ScreenCaptureFrame` wraps a `PlatformScreenCaptureFrame` type alias that varies per platform
- `current_platform()` in `platform.rs` constructs platforms via `#[cfg]`
- `Application::new()` calls `current_platform()` internally
- gpui's `Cargo.toml` carries all platform-specific dependencies (cocoa, metal, windows, wayland, etc.)
- gpui's `build.rs` has platform-specific shader compilation and bindgen

## Architecture After Refactor

### Dependency Graph

```
zed ──→ gpui_platform ─┬─→ gpui_macos_platform ──→ gpui
                        ├─→ gpui_windows_platform → gpui
                        └─→ gpui_linux_platform ──→ gpui
                                               └──→ gpui_blade_renderer → gpui

gpui_macos_platform (macos-blade feature) ────────→ gpui_blade_renderer → gpui
```

No circular dependencies. Platform crates depend on gpui for trait definitions
and shared types. gpui itself has zero knowledge of any platform implementation.

### Crate Descriptions

**`gpui`** (modified)
- All platform trait definitions (`Platform`, `PlatformWindow`, `PlatformDisplay`,
  `PlatformDispatcher`, `PlatformTextSystem`, `PlatformAtlas`, etc.)
- All shared types used in trait signatures (`WindowParams`, `ClipboardItem`,
  `Scene`, `Bounds`, atlas types, etc.)
- Cross-platform modules: `app_menu`, `keyboard`, `keystroke`
- Test platform (`TestPlatform`, `TestWindow`, `TestDispatcher`, `TestDisplay`)
- `NoopTextSystem`
- The new `PlatformPixelBuffer` trait
- Everything else (elements, entities, executor, window management, etc.)
- No platform-specific native dependencies
- No platform-specific build.rs code

**`gpui_blade_renderer`** (new)
- Current `platform/blade/` minus `apple_compat.rs`
- `BladeRenderer`, `BladeAtlas`, `BladeContext`, `BladeSurfaceConfig`
- WGSL shader and build.rs validation
- Depends on: `gpui`, `blade-graphics`, `blade-macros`, `blade-util`, `bytemuck`

**`gpui_macos_platform`** (new)
- Current `platform/mac/` plus `apple_compat.rs` from blade
- `MacPlatform`, `MacWindow`, `MacDisplay`, `MacDispatcher`, `MacTextSystem`
- Metal renderer and atlas
- Screen capture via ScreenCaptureKit
- macOS build.rs (bindgen for dispatch.h, metal shader compilation, cbindgen)
- `impl PlatformPixelBuffer for CVPixelBuffer`
- Depends on: `gpui`, optionally `gpui_blade_renderer` (behind `macos-blade` feature)
- Carries: `cocoa`, `core-foundation`, `core-graphics`, `core-text`, `metal`,
  `objc`, `font-kit`, `core-video`, `media`, etc.
- Exports: `MacPlatform`

**`gpui_windows_platform`** (new)
- Current `platform/windows/`
- `WindowsPlatform`, `WindowsWindow`, `WindowsDisplay`, `WindowsDispatcher`
- DirectX renderer and atlas, DirectWrite text system
- Windows build.rs (HLSL shader compilation, embed-resource, manifest)
- Depends on: `gpui`, `gpui_scap_screen_capture`
- Carries: `windows`, `windows-core`, `windows-numerics`, `windows-registry`,
  optionally `scap`
- Exports: `WindowsPlatform`

**`gpui_linux_platform`** (new)
- Current `platform/linux/` (wayland/, x11/, headless/ subdirs)
- `WaylandClient`, `X11Client`, `HeadlessClient`, `LinuxDispatcher`
- Depends on: `gpui`, `gpui_blade_renderer`, `gpui_scap_screen_capture`
- Carries: wayland deps, x11 deps, `cosmic-text`, `font-kit`, `ashpd`,
  `calloop`, `xkbcommon`, `oo7`, optionally `scap`
- Exports: platform factory functions for wayland/x11/headless selection

**`gpui_scap_screen_capture`** (new)
- Current `platform/scap_screen_capture.rs` (~250 lines)
- Shared screen capture logic using the `scap` crate
- `impl PlatformPixelBuffer for scap::frame::Frame`
- Depends on: `gpui`, `scap`
- Used by: `gpui_linux_platform`, `gpui_windows_platform`

**`gpui_platform`** (new)
- ~20 lines of actual code
- Uses `#[cfg]` to depend on the correct platform crate
- Re-exports a `current_platform(headless: bool) -> Rc<dyn Platform>` function
- Used by the application binary (e.g. zed) to construct the platform

## Phase 1: `PlatformPixelBuffer` Refactor

### Problem

Two types in gpui leak platform-specific dependencies through `#[cfg]` and
concrete platform types:

1. **`PaintSurface`** in `scene.rs` has a field
   `#[cfg(target_os = "macos")] pub image_buffer: core_video::pixel_buffer::CVPixelBuffer`

2. **`ScreenCaptureFrame`** in `platform.rs` wraps `PlatformScreenCaptureFrame`,
   a type alias that resolves to `CVImageBuffer` (macOS), `scap::frame::Frame`
   (Linux/Windows), or `()` (no screen capture).

These create direct dependencies from gpui core on `core-video` and `scap`.

### Solution: `PlatformPixelBuffer` trait

Define a new trait in `platform.rs` to abstract over platform-specific pixel
buffers:

```rust
/// An opaque, platform-specific pixel buffer for rendering video or screen
/// capture content.
///
/// Platform implementations provide concrete types that implement this trait.
/// Renderers downcast to the expected concrete type via `as_any()`.
pub trait PlatformPixelBuffer: 'static {
    /// Width in pixels.
    fn width(&self) -> u32;
    /// Height in pixels.
    fn height(&self) -> u32;
    /// Downcast to the concrete platform type.
    fn as_any(&self) -> &dyn Any;
    /// Downcast to the concrete platform type, consuming self.
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
```

#### Normalizing macOS CoreVideo types

In Apple's C API, `CVPixelBufferRef` is a typedef for `CVImageBufferRef` — they
are the same underlying CoreFoundation type. However, the Rust `core-video`
crate wraps them as distinct Rust types (`CVPixelBuffer` vs `CVImageBuffer`).
Since Rust's `Any` uses `TypeId`, a `downcast_ref::<CVPixelBuffer>()` on a
trait object wrapping `CVImageBuffer` would fail.

**Decision**: Normalize to `CVPixelBuffer` everywhere in platform code.
- The screen capture callback receives a `CVImageBuffer` and converts it via
  `image_buffer.downcast_into::<CVPixelBuffer>()` before boxing as
  `dyn PlatformPixelBuffer`.
- Renderers always `downcast_ref::<CVPixelBuffer>()`.
- Only one `impl PlatformPixelBuffer for CVPixelBuffer` is needed.

### Changes

#### `platform.rs` — new trait + updated `ScreenCaptureFrame`

Add the `PlatformPixelBuffer` trait (shown above).

Replace:
```rust
pub struct ScreenCaptureFrame(pub PlatformScreenCaptureFrame);
```
With:
```rust
pub struct ScreenCaptureFrame(pub Box<dyn PlatformPixelBuffer>);
```

Delete the `PlatformScreenCaptureFrame` type aliases from `mac.rs`, `linux.rs`,
and `windows.rs`.

#### `scene.rs` — `PaintSurface`

Replace:
```rust
pub(crate) struct PaintSurface {
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    #[cfg(target_os = "macos")]
    pub image_buffer: core_video::pixel_buffer::CVPixelBuffer,
}
```
With:
```rust
pub(crate) struct PaintSurface {
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub pixel_buffer: Box<dyn PlatformPixelBuffer>,
}
```

#### `window.rs` — `paint_surface`

Replace:
```rust
#[cfg(target_os = "macos")]
pub fn paint_surface(&mut self, bounds: Bounds<Pixels>, image_buffer: CVPixelBuffer) {
```
With:
```rust
pub fn paint_surface(&mut self, bounds: Bounds<Pixels>, pixel_buffer: Box<dyn PlatformPixelBuffer>) {
```

Remove the `#[cfg(target_os = "macos")]` gate — the method is now
platform-agnostic.

#### `elements/surface.rs` — `Surface` element

Replace:
```rust
pub enum SurfaceSource {
    #[cfg(target_os = "macos")]
    Surface(CVPixelBuffer),
}
```
With:
```rust
pub enum SurfaceSource {
    PixelBuffer(Box<dyn PlatformPixelBuffer>),
}
```

Remove the `#[cfg(target_os = "macos")]` gate on `pub fn surface(...)`.

Update `From` impl to accept `Box<dyn PlatformPixelBuffer>` instead of
`CVPixelBuffer`.

#### Platform implementations — `impl PlatformPixelBuffer`

macOS (`gpui_macos_platform`):
```rust
impl PlatformPixelBuffer for CVPixelBuffer {
    fn width(&self) -> u32 { self.get_width() as u32 }
    fn height(&self) -> u32 { self.get_height() as u32 }
    fn as_any(&self) -> &dyn Any { self }
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
}
```

Linux/Windows (`gpui_scap_screen_capture`):
```rust
impl PlatformPixelBuffer for scap::frame::Frame {
    fn width(&self) -> u32 { /* extract from frame variant */ }
    fn height(&self) -> u32 { /* extract from frame variant */ }
    fn as_any(&self) -> &dyn Any { self }
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
}
```

#### Renderers — downcast to concrete type

Metal renderer (`draw_surfaces`):
```rust
for surface in surfaces {
    let image_buffer = surface.pixel_buffer
        .as_any()
        .downcast_ref::<CVPixelBuffer>()
        .expect("macOS Metal renderer requires CVPixelBuffer");
    // ... rest unchanged, use image_buffer as before
}
```

Same pattern for the Blade renderer's `#[cfg(target_os = "macos")]` surface
rendering path.

DirectX renderer: `draw_surfaces` is currently a no-op (surfaces not yet
implemented on Windows), so no changes needed.

#### macOS screen capture — normalize `CVImageBuffer` → `CVPixelBuffer`

In `platform/mac/screen_capture.rs`, where the capture callback wraps the frame:
```rust
// Before:
callback(ScreenCaptureFrame(buffer));

// After:
if let Some(pixel_buffer) = buffer.downcast_into::<CVPixelBuffer>() {
    callback(ScreenCaptureFrame(Box::new(pixel_buffer)));
}
```

#### `livekit_client` — updated frame access

macOS path in `video_frame_buffer_to_webrtc`:
```rust
// Before:
let pixel_buffer = frame.0.as_concrete_TypeRef();
std::mem::forget(frame.0);

// After:
let pixel_buffer = frame.0.into_any()
    .downcast::<CVPixelBuffer>()
    .expect("macOS screen capture produces CVPixelBuffer");
let raw = pixel_buffer.as_concrete_TypeRef();
std::mem::forget(*pixel_buffer);
```

Non-macOS path:
```rust
// Before:
match frame.0 { scap::frame::Frame::BGRx(frame) => { ... } ... }

// After:
let scap_frame = frame.0.into_any()
    .downcast::<scap::frame::Frame>()
    .expect("non-macOS screen capture produces scap Frame");
match *scap_frame { scap::frame::Frame::BGRx(frame) => { ... } ... }
```

`RemoteVideoFrame` on macOS is `CVPixelBuffer`, which now implements
`PlatformPixelBuffer`. The `surface()` call in the render method boxes it:
```rust
gpui::surface(Box::new(latest_frame.clone()) as Box<dyn PlatformPixelBuffer>)
```

### What Phase 1 Achieves

After Phase 1, gpui core has **zero `#[cfg(target_os)]` around pixel buffer
types** and **zero direct dependencies on `core_video` or `scap`**. The
`PlatformPixelBuffer` trait is the abstraction seam that Phase 2 builds on.

## Phase 2: Platform Crates Refactor

### Step 1: Make platform traits and types public

Currently `pub(crate)` items that platform crates need to implement/use:
- Traits: `Platform`, `PlatformWindow`, `PlatformTextSystem`, `PlatformAtlas`
- Types: `WindowParams`, `RequestFrameOptions`, `PlatformInputHandler`
- Atlas types: `AtlasKey`, `AtlasTile`, `AtlasTextureId`, `AtlasTextureKind`,
  `TileId`, `AtlasTextureList`
- `NoopTextSystem`

All become `pub`. These are legitimate extension points for platform
implementations and do not need `#[doc(hidden)]`.

### Step 2: Change `Application::new()` to accept a platform

```rust
// gpui — Application accepts a platform from outside:
impl Application {
    pub fn new(platform: Rc<dyn Platform>) -> Self {
        Self(App::new_app(
            platform,
            Arc::new(()),
            Arc::new(NullHttpClient),
        ))
    }
}
```

Remove `current_platform()` from gpui entirely. It moves to `gpui_platform`.

Calling code becomes:
```rust
// zed/main.rs:
let platform = gpui_platform::current_platform(false);
let app = Application::new(platform).with_assets(Assets);
```

### Step 3: Update the test macro

The `#[gpui::test]` macro currently generates calls to
`gpui::TestAppContext::build(dispatcher, fn_name)` which internally creates a
`TestPlatform`. Update to make platform construction explicit:

`TestAppContext::build` signature changes:
```rust
// Before:
pub fn build(dispatcher: TestDispatcher, fn_name: Option<&'static str>) -> Self

// After:
pub fn build(
    platform: Rc<TestPlatform>,
    dispatcher: TestDispatcher,
    fn_name: Option<&'static str>,
) -> Self
```

The macro generates:
```rust
let arc_dispatcher = std::sync::Arc::new(dispatcher.clone());
let background_executor = gpui::BackgroundExecutor::new(arc_dispatcher.clone());
let foreground_executor = gpui::ForegroundExecutor::new(arc_dispatcher);
let platform = gpui::TestPlatform::new(
    background_executor.clone(),
    foreground_executor.clone(),
);
let mut cx = gpui::TestAppContext::build(
    platform,
    dispatcher.clone(),
    Some(stringify!(#outer_fn_name)),
);
```

This is consistent with the principle that platform is always injected from
outside.

### Step 4: Handle `VisualTestPlatform`

`VisualTestPlatform` wraps `MacPlatform` and lives in
`platform/visual_test.rs`. After the split, `MacPlatform` lives in
`gpui_macos_platform`.

`VisualTestAppContext` needs to stay in gpui (it's used by gpui's own tests).
Rather than having gpui depend on `gpui_macos_platform`, the visual test context
accesses the real platform through downcasting:

```rust
cx.app.platform().downcast::<MacPlatform>()
```

The `VisualTestPlatform` type itself moves to `gpui_macos_platform`. The
`VisualTestAppContext` in gpui constructs it by accepting an `Rc<dyn Platform>`
and using it opaquely — it doesn't need to know it's a `MacPlatform`
internally.

This means `VisualTestAppContext` is not constructed via the standard test macro
path but rather by test code that explicitly depends on `gpui_macos_platform`
and passes in the visual test platform.

### Step 5: Create `gpui_blade_renderer` crate

Move from gpui:
- `platform/blade/blade_renderer.rs`
- `platform/blade/blade_atlas.rs`
- `platform/blade/blade_context.rs`
- `platform/blade/shaders.wgsl`

Move from gpui's `build.rs`:
- `check_wgsl_shaders()` function and naga build dependency

`apple_compat.rs` moves to `gpui_macos_platform` (it's the macOS-blade bridge).

Update imports in Linux platform code (wayland/x11 windows) and macOS blade
code to use `gpui_blade_renderer::*` instead of `crate::platform::blade::*`.

### Step 6: Create `gpui_macos_platform` crate

Move from gpui:
- `platform/mac/` (all files)
- `platform/blade/apple_compat.rs`

Move from gpui's `build.rs`:
- `mod macos` (bindgen for dispatch.h, metal shader compilation, cbindgen)

Move from gpui's `Cargo.toml`:
- `[target.'cfg(target_os = "macos")'.dependencies]` section
- `[target.'cfg(target_os = "macos")'.build-dependencies]` section
- `pathfinder_geometry` dependency (macOS + Linux shared, duplicated)

The crate exports `MacPlatform` and `VisualTestPlatform`.

The `impl PlatformPixelBuffer for CVPixelBuffer` lives here.

### Step 7: Create `gpui_scap_screen_capture` crate

Move from gpui:
- `platform/scap_screen_capture.rs`

The `impl PlatformPixelBuffer for scap::frame::Frame` lives here.

Small crate (~250 lines + the trait impl). Depends on `gpui` and `scap`.

### Step 8: Create `gpui_windows_platform` crate

Move from gpui:
- `platform/windows/` (all files)

Move from gpui's `build.rs`:
- `mod windows` (HLSL shader compilation, embed-resource, manifest)

Move from gpui's `Cargo.toml`:
- `[target.'cfg(target_os = "windows")'.dependencies]` section
- `[target.'cfg(target_os = "windows")'.build-dependencies]` section

Depends on `gpui_scap_screen_capture` for screen capture support.

### Step 9: Create `gpui_linux_platform` crate

Move from gpui:
- `platform/linux/` (all files including wayland/, x11/, headless/ subdirs)

Move from gpui's `Cargo.toml`:
- `[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]`
- Linux build-dependencies (naga for blade shader validation)
- Wayland and X11 optional dependency groups

Depends on `gpui_blade_renderer` and `gpui_scap_screen_capture`.

### Step 10: Create `gpui_platform` alias crate

Thin crate (~20 lines) that cfg-selects the right platform:

```rust
#[cfg(target_os = "macos")]
pub use gpui_macos_platform::MacPlatform;

#[cfg(target_os = "windows")]
pub use gpui_windows_platform::WindowsPlatform;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub use gpui_linux_platform::*;

pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    // cfg-gated construction, same logic as current current_platform()
}
```

### Step 11: Update all call sites

- `zed/main.rs`: `Application::new(gpui_platform::current_platform(false))`
- All other binaries/examples that call `Application::new()`
- Test macro (already updated in step 3)
- Remove deprecated old `Application::new()` if a temporary shim was added

### Step 12: Final cleanup

- Delete emptied `platform/mac/`, `platform/linux/`, `platform/windows/`,
  `platform/blade/` directories from gpui
- Delete `platform/visual_test.rs` and `platform/scap_screen_capture.rs`
- Remove all `[target.'cfg(...)'.dependencies]` for native deps from gpui's
  `Cargo.toml`
- Gut gpui's `build.rs` (remove macOS and Windows sections entirely)
- Remove `current_platform()` implementations from gpui's `platform.rs`
- Verify compile time improvements
