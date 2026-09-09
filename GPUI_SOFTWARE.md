# gpui_software: a lean CPU renderer for GPUI

## 1. Goal

Make Zed usable on machines with no GPU (Azure Virtual Desktop / RDP sessions, VMs, CI boxes, machines with broken drivers) by adding a CPU rendering backend to GPUI that:

- never touches Direct3D, Vulkan, OpenGL, or any software emulation of them (no WARP, no llvmpipe);
- rasterizes only what is necessary for a readable, correctly laid out UI: text, rectangles, borders, underlines, images, and vector paths;
- deliberately drops decorative work (rounded corners, anti-aliased edges, shadows, dashed borders, wavy underlines, pattern fills) because it costs CPU and adds nothing to function;
- keeps *metrics* identical to the GPU backends: layout, sizing, positioning, scale factor, pixel snapping, glyph placement, and glyph bitmaps are unchanged, because none of that lives in the renderer;
- keeps *font rendering* equivalent to the GPU backends: same platform-rasterized glyph bitmaps (DirectWrite on Windows), same subpixel (ClearType) support, same contrast/gamma correction math;
- redraws only the parts of the window that changed, in parallel across cores;
- is selected automatically when no hardware GPU is available, while users with a GPU keep using the existing GPU backends untouched.

Nothing above the `Scene` changes. All ~170 components keep emitting the same primitives through the same GPUI APIs.

## 2. Why this is possible without touching components

GPUI's platform boundary is already a retained scene:

- Every element paints into `Scene` (`crates/gpui/src/scene.rs`), which holds eight primitive kinds: `Shadow`, `Quad`, `Path`, `Underline`, `MonochromeSprite`, `SubpixelSprite`, `PolychromeSprite`, `PaintSurface` (macOS video only). Each primitive carries device-pixel `bounds` and an axis-aligned `content_mask`.
- `Scene::batches()` yields the primitives already sorted into draw order, grouped by kind and atlas texture.
- The platform window consumes it via `PlatformWindow::draw(&Scene)` and supplies glyph/image storage via `PlatformAtlas` (`crates/gpui/src/platform.rs:761`, `:1199`).
- Glyph bitmaps are produced on the CPU by the platform text system (`DirectWriteTextSystem::rasterize_glyph` in `crates/gpui_windows/src/direct_write.rs`); the GPU only composites them.

So a new backend is: an implementation of `draw(&Scene)` that rasterizes into a CPU framebuffer, an implementation of `PlatformAtlas` that keeps tiles in memory, and a per-platform presenter that copies the framebuffer to the window.

## 3. Fidelity contract ("lean mode")

The software backend recognizes primitives that request decorative work and lowers them to the cheapest equivalent that preserves layout and readability. This table is the contract; the lowering code in `lower.rs` implements exactly this and nothing more.

Quad (`Quad`):
- `bounds` and `content_mask`: snapped to the integer pixel grid using the same coverage rule as the GPU (a pixel is covered when its center is inside; i.e. edges are `round()`ed). No edge anti-aliasing.
- `corner_radii`: ignored. All quads are rectangles.
- `border_widths` / `border_color`: drawn as up to four axis-aligned rectangles inside `bounds` (top, bottom, left, right), each blended with `border_color`. `BorderStyle::Dashed` is drawn solid.
- `background` `Solid`: filled. Opaque fast path when alpha == 1, blended otherwise.
- `background` `LinearGradient`: filled using a 256-entry color LUT indexed by the same affine `t(x, y)` the shader computes (`gradient_color` in `crates/gpui_wgpu/src/shaders.wgsl:431`). Both `Srgb` and `Oklab` interpolation are supported at LUT build time; they cost nothing per pixel.
- `background` `PatternSlash`: filled solid with `solid.a * 0.5` (keeps diff/pending regions distinguishable without evaluating the stripe function per pixel).
- `background` `Checkerboard`: filled solid with the `solid` color.

Shadow (`Shadow`): not drawn at all, drop and inset alike.

Underline (`Underline`): a filled rectangle `thickness` pixels tall, vertically centered in `bounds`. `wavy` is ignored.

Monochrome sprite (glyphs and SVG icons): exact. Same bitmap, same color, same contrast/gamma correction as the GPU shader, blended per pixel. With a non-identity `transformation` (rotated icons) the tile is sampled nearest-neighbor through the inverse transform; no filtering.

Subpixel sprite (ClearType glyphs): exact. Per-channel coverage from the same DirectWrite bitmap, same correction math, per-channel blend (the CPU does not need dual-source blending hardware).

Polychrome sprite (images, emoji): straight-alpha blit with `opacity` applied. `corner_radii` ignored (square avatars). `grayscale` applied (cheap luma per pixel, rare).

Path (`Path`): filled through `vello_cpu` with the path's `color` (solid or gradient) clipped to `content_mask`. Anti-aliasing is disabled when the pinned `vello_cpu` exposes an aliasing threshold; otherwise vello's default AA is accepted because paths are rare.

Surface (`PaintSurface`): ignored (macOS only).

Window background: always opaque. `WindowBackgroundAppearance::Transparent`, `Blurred`, `Mica*` are treated as `Opaque` in software mode (there is no compositor path without DirectComposition). This is also what makes subpixel text eligible (`Window::should_use_subpixel_rendering`, `crates/gpui/src/window.rs:4018`).

Color and blending semantics match the DirectX backend so that colors are bit-identical where geometry is identical: HSLA → RGBA via `Rgba::from(Hsla)`, values quantized to 8 bits, blending in gamma-encoded space (the GPU renders into a UNORM target with `SrcAlpha / InvSrcAlpha`, see `create_blend_state` in `crates/gpui_windows/src/directx_renderer.rs:1383`). Subpixel sprites use per-channel `src1 / (1 - src1)` blending like `create_blend_state_for_subpixel_rendering`.

## 4. Crate layout

New crate `crates/gpui_software` (the directory already exists and is empty):

```
crates/gpui_software/
  Cargo.toml                 [lib] path = "src/gpui_software.rs"
  src/gpui_software.rs       pub API: SoftwareRenderer, SoftwareAtlas, Framebuffer, Damage, FontCorrection
  src/framebuffer.rs         BGRX8 top-down framebuffer, band views
  src/atlas.rs               PlatformAtlas impl backed by Vec<u8> textures (etagere allocation)
  src/lower.rs               Scene primitive -> lean draw op lowering (the fidelity contract)
  src/bin_pass.rs            cell binning: per-cell op lists, per-cell hashes, opaque-cover cutoffs
  src/damage.rs              previous/current cell hash grids, dirty band/x-range computation
  src/raster.rs              band worker: walks a band's dirty cells and dispatches kernels
  src/kernels.rs             scalar reference kernels + runtime dispatch table
  src/kernels_avx2.rs        x86_64 AVX2 versions of the hot kernels
  src/text_correction.rs     DirectWrite-equivalent contrast/gamma LUTs
  src/paths.rs               vello_cpu integration for Path primitives
  src/stats.rs               optional per-frame timing/damage logging
  benches/frame.rs           synthetic editor frames (full redraw, one-line damage, scroll)
```

Dependencies: `gpui`, `gpui_util`, `anyhow`, `log`, `etagere` (already used by the atlases), `rayon` (workspace dep), `collections` (FxHash), `vello_cpu = { version = "0.2", default-features = false, features = ["std", "u8_pipeline"] }` (no `text`, no `png`, no `multithreading`: parallelism is ours). `bytemuck` for hashing primitive bytes.

The crate is platform independent and has no windowing code. Platform crates own presentation:

- `crates/gpui_windows`: `WindowsRenderer` enum, GDI presenter, DirectWrite changes, backend selection.
- `crates/gpui_linux`: `wl_shm` and X11 `put_image`/MIT-SHM presenters, backend selection (second phase).
- `crates/gpui_macos`: no user-facing software mode. Optionally a `CALayer.contents` presenter behind `GPUI_RENDERER=software` so the crate can be developed and eyeballed on a Mac.

## 5. gpui_software design

### 5.1 Framebuffer

```rust
pub struct Framebuffer {
    pixels: Vec<u32>,            // 0xXXRRGGBB (BGRA8 little-endian, alpha byte unused = 0xFF)
    size: Size<DevicePixels>,
}
```

Top-down, row-major, 32bpp, opaque. This is exactly the layout `BITMAPINFO` with negative height wants on Windows, `WL_SHM_FORMAT_XRGB8888` on Wayland, and depth-24 `ZPixmap` on X11, so presentation is a copy with no swizzle.

Bands: the framebuffer is split into horizontal bands of `BAND_HEIGHT = 32` rows. A band is a contiguous `&mut [u32]` obtained with `chunks_mut(width * BAND_HEIGHT)`, which is what allows rayon to hand disjoint mutable slices to worker threads without unsafe code.

Resize reallocates and marks the whole frame dirty.

### 5.2 Atlas

`SoftwareAtlas` implements `PlatformAtlas` with the same allocation policy as `DirectXAtlas` (`crates/gpui_windows/src/directx_atlas.rs`): one `AtlasTextureList` per `AtlasTextureKind`, `etagere::BucketedAtlasAllocator`, 1024x1024 default texture, `tiles_by_key: FxHashMap<AtlasKey, AtlasTile>`, ref-counted texture retirement in `remove`.

Texture storage is `Vec<u8>` with the same pixel formats the GPU atlases use, because `Window::paint_glyph` / `paint_emoji` / `paint_svg` / `paint_image` upload bytes in those formats today:
- Monochrome: 1 byte/pixel coverage (glyphs and SVGs).
- Subpixel: 4 bytes/pixel, RGB = per-channel coverage, A unused (see `rasterize_subpixel` in `direct_write.rs:839-875`); `is_bgr` swizzle happens at blit time like the shader.
- Polychrome: 4 bytes/pixel BGRA, straight alpha.

`get_or_insert_with` runs on the main thread during paint; `draw` locks the atlas once per frame and passes the immutable state to band workers. Tile ids are never reused with different content within a frame (etagere ids carry a generation), so `(texture_id, tile_id, bounds)` is a valid content identity for damage hashing.

Maximum texture size: 16384, same as DirectX, but there is no hardware limit on the CPU; keep the number for identical allocation behavior.

### 5.3 Lowering (`lower.rs`)

`draw` first converts every primitive in `scene.batches()` order into a flat `Vec<Op>` of lean operations. This is where the fidelity contract from section 3 is applied and where geometry is snapped to integers once, instead of per pixel or per band.

```rust
enum Op {
    FillOpaque { rect: IRect, color: u32 },
    FillBlend  { rect: IRect, color: u32 },           // color has alpha < 255
    FillLut    { rect: IRect, lut: LutId, t0: f32, dt_dx: f32, dt_dy: f32 },
    BlitMono   { rect: IRect, src: TileRef, color: u32, lut: LutId },
    BlitMonoXf { rect: IRect, src: TileRef, color: u32, lut: LutId, inverse: [f32; 6] },
    BlitSub    { rect: IRect, src: TileRef, color: u32, lut3: LutId },
    BlitPoly   { rect: IRect, src: TileRef, opacity: u8, grayscale: bool },
    Path       { path_index: u32, clip: IRect },
}
```

Every `Op` carries its final clipped device rect (`bounds ∩ content_mask`, both rounded). Ops whose rect is empty are dropped here. A `Quad` becomes zero to five ops (up to four border rects plus one background op); a `Shadow` becomes none; an `Underline` becomes one fill.

Lowering also records, per op, whether it is *opaque and rectangular* (`FillOpaque`, `BlitPoly` with opacity 255 is not, because the image may have alpha). This flag drives the occlusion cutoff below.

Snapping rule (matches GPU pixel-center coverage for axis-aligned edges): `x0 = round(bounds.origin.x)`, `x1 = round(bounds.origin.x + bounds.size.width)`, same for y; an op is empty if `x1 <= x0 || y1 <= y0`.

Color conversion: `Rgba::from(hsla)`, each channel `(c * 255.0 + 0.5) as u8`, packed as `0xAARRGGBB`.

### 5.4 Cell binning (`bin_pass.rs`)

The window is a grid of cells `CELL_WIDTH = 64` by `BAND_HEIGHT = 32` pixels. A single pass over the lowered ops produces, per cell:

- `ops: Vec<u32>` — indices of ops intersecting the cell, in draw order;
- `hash: u64` — an order-dependent fold of the hashes of those ops;
- `opaque_cutoff: u32` — index into `ops` of the last op that is opaque and fully covers the cell (0 if none).

Op hashes are computed once per op from its lowered bytes (`bytemuck::bytes_of`) plus, for blits, the tile identity, and for paths, a hash of the path's vertex bytes and color. Fold: `cell.hash = mix(cell.hash, op_hash)` (e.g. FxHasher `write_u64` twice), so reordering two ops inside a cell changes the hash.

The pass is linear in `sum(cells touched)`. A full-window background quad touches every cell (~1000 at 1080p); glyphs touch one or two. This costs well under a millisecond for a normal frame and can be parallelized per op chunk later if it shows up in profiles.

`opaque_cutoff` is the overdraw eliminator: the band worker starts drawing a cell at that index, so the window background, pane background, editor background, and current-line highlight under a piece of text are drawn once, not four times. In lean mode most cells begin with a single opaque fill.

### 5.5 Damage tracking (`damage.rs`)

Keep the previous frame's cell hash grid. After binning, a cell is dirty when its hash differs from the previous frame's, when the grid dimensions changed, or when the frame is forced (`RequestFrameOptions::force_render`, resize, first frame after presenter (re)creation).

For each band, the dirty rect is `[min_dirty_col * 64, (max_dirty_col + 1) * 64)` clipped to the framebuffer. Bands with no dirty cells are skipped entirely. `draw` returns the list of `(band_y, x0, x1)` damage rects so the presenter can copy only those rows and columns. Typing one character dirties one or two cells; a caret blink dirties one; scrolling dirties the editor area but not the panels.

Cells that are dirty only because their hash changed but whose pixel result is identical (e.g. a tooltip timer element repainting the same pixels) still get redrawn. That is acceptable; correctness never depends on the heuristic.

### 5.6 Band workers (`raster.rs`)

```rust
dirty_bands.par_iter_mut().for_each(|band| {
    for cell in band.dirty_cells() {
        let start = cell.opaque_cutoff;
        for op_index in &cell.ops[start..] {
            kernels.dispatch(&ops[op_index], cell.rect, band.pixels, ...);
        }
    }
});
```

Each op is clipped to the cell rect before the kernel runs, so an op spanning many cells is drawn piecewise; kernels operate on row spans and do not care. Because a band is a contiguous slice of the framebuffer, no unsafe aliasing is needed for parallelism.

Rayon's global pool is used (Zed already links rayon). Bands are the unit of parallelism, so a 1080p window has 34 potential tasks and a 4K window 68; on a many-core EPYC that is enough to saturate the cores during full redraws, and partial redraws simply run fewer tasks.

The first-cell-op fast path: when `opaque_cutoff` points at a `FillOpaque` that covers the whole cell, the cell is filled with a straight row store loop; nothing is read from the framebuffer.

### 5.7 Kernels (`kernels.rs`, `kernels_avx2.rs`)

Hot kernels, each with a scalar reference implementation and an AVX2 implementation:

- `fill_opaque(rows, color)`: 32-bit stores.
- `fill_blend(rows, color, alpha)`: `dst = dst + ((src - dst) * a + 128) >> 8` per channel, 8 pixels per AVX2 iteration using 16-bit lanes.
- `fill_lut(rows, lut, t0, dt_dx)`: per row, compute `t` per pixel as a fixed-point accumulator, index the 256-entry premultiplied LUT, blend. Gradients are rare; AVX2 version optional.
- `blit_mono(rows, coverage_row, color, lut)`: `a = lut[coverage]`, then the blend above. This is the glyph kernel and the single most executed loop in an editor frame. AVX2: gather-free by using `vpshufb`-based 16-entry table lookups is not enough for a 256-entry LUT, so use `vpgatherdd` on the LUT (fine on Zen 3/4) or process coverage through a 2-level lookup; benchmark both, keep the faster.
- `blit_subpixel(rows, coverage_rgb_row, color, lut)`: per-channel `a_c = lut[coverage_c]`, `dst_c = dst_c + (src_c - dst_c) * a_c`.
- `blit_poly(rows, bgra_row, opacity)`: straight-alpha over with `a = src.a * opacity`.
- `blit_mono_transformed`: scalar only (rare).

Dispatch: a `Kernels` struct of function pointers initialized once via `std::sync::OnceLock`, choosing AVX2 when `is_x86_feature_detected!("avx2")` (and `fma` where useful), otherwise scalar. AVX2 functions are `#[target_feature(enable = "avx2")] unsafe fn` with a documented safety contract (only called through the dispatcher after detection). On aarch64 the scalar versions autovectorize to NEON at baseline; add explicit NEON only if profiling on Apple/Ampere hardware asks for it.

Testing: every AVX2 kernel is checked against its scalar twin on randomized inputs in the crate's unit tests, including unaligned row starts and short rows (< 8 pixels).

### 5.8 Text correction (`text_correction.rs`)

For text to look the same as on the GPU, the compositing must reproduce `apply_contrast_and_gamma_correction` / `apply_contrast_and_gamma_correction3` from the shaders (`crates/gpui_wgpu/src/shaders.wgsl:34-78`, `crates/gpui_windows/src/alpha_correction.hlsl`), parameterized by the platform's `FontCorrection { gamma_ratios: [f32; 4], grayscale_enhanced_contrast: f32, subpixel_enhanced_contrast: f32, is_bgr: bool }` (on Windows: `DirectXRenderer::get_font_info`, `directx_renderer.rs:737`, moved to a shared place).

For a given text color the correction is a pure function of the 8-bit coverage sample, so it is a 256-entry table:

```
brightness  = 0.30 r + 0.59 g + 0.11 b                      (color in 0..1)
k           = enhanced_contrast * saturate(4 * (0.75 - brightness))
contrasted  = s * (k + 1) / (s * k + 1)                      (s = coverage / 255)
adj         = g.x * brightness + g.y
corr        = adj * contrasted + (g.z * brightness + g.w)
alpha       = contrasted + contrasted * (1 - contrasted) * corr
lut[c]      = round(alpha * color.a * 255)
```

For subpixel sprites the shader passes the per-channel `color` vector into `apply_alpha_correction3`, i.e. channel `c` uses `b = color_c` instead of the brightness scalar; build three tables (`lut3`) per color.

Tables are cached per frame in an `FxHashMap<u32 /* packed color */, LutId>`; a frame has a few dozen distinct text colors. A unit test compares the tables against a straight float port of the shader for a sweep of colors with tolerance ±1.

### 5.9 Paths via vello_cpu (`paths.rs`)

`Path<ScaledPixels>` arrives as a triangle list (`crates/gpui/src/scene.rs:755`). Triangles produced by `PathBuilder` (lyon tessellation) all carry `st = (0, 1)`; triangles from `Path::curve_to` carry `st = (0,0), (0.5,0), (1,1)` and encode a quadratic Bézier bulge. Convert to a `kurbo::BezPath`:

- for each triangle with constant `st`: `move_to(v0) line_to(v1) line_to(v2) close`;
- for each curve triangle: `move_to(v0) quad_to(v1, v2) close` (this region is exactly the area between chord and curve that the shader's `s² - t` test fills);
- normalize each subpath to positive signed area before emitting, so shared interior edges cancel under the nonzero rule and the union renders without seams.

Per band worker, keep a thread-local `vello_cpu::RenderContext` sized `width x BAND_HEIGHT` (reused across frames via `reset_and_resize`). When the cell loop meets a `Path` op: `set_transform(Affine::translate((0, -band_y)))`, `push_clip_path(clip_rect)`, `set_paint(color)`, `fill_path(&bez_path)`, `pop_clip_path()`. Consecutive path ops accumulate; on the next non-path op or at band end: `flush()`, then `render_with` compositing source-over onto a `PixmapMut` view of the band slice, then `reset()`. If the pinned `vello_cpu` cannot target BGRA8 directly, render into an RGBA8 scratch pixmap and composite with a small swizzle kernel. If `RenderContext::set_aliasing_threshold` exists in the pinned version, set it so path edges are not anti-aliased.

Gradient path colors reuse the LUT machinery via `set_paint` with a `peniko` linear gradient built from the same two stops; if that turns out fiddly, a solid fill with the first stop is acceptable under the fidelity contract.

Since a `Path`'s hash includes its vertices, a path that does not move is not redrawn.

### 5.10 Public API

```rust
pub struct SoftwareRenderer { .. }

impl SoftwareRenderer {
    pub fn new(size: Size<DevicePixels>, font_correction: FontCorrection) -> Self;
    pub fn atlas(&self) -> Arc<dyn PlatformAtlas>;
    pub fn resize(&mut self, size: Size<DevicePixels>);
    pub fn set_font_correction(&mut self, correction: FontCorrection);
    /// Rasterizes `scene`, returns the damage rects that changed since the last draw.
    pub fn draw(&mut self, scene: &Scene, force_full: bool) -> Damage;
    pub fn framebuffer(&self) -> &Framebuffer;
    pub fn gpu_specs(&self) -> GpuSpecs;
}

pub struct Damage { pub rects: Vec<Bounds<DevicePixels>> } // empty = nothing to present
```

`gpu_specs()` returns `GpuSpecs { is_software_emulated: false, device_name: "GPUI software renderer", driver_name: "gpui_software", driver_info: "<simd level>, <thread count>" }`. `is_software_emulated` is false on purpose: that flag means "a GPU API is being emulated on the CPU", which is the situation this backend exists to avoid, and it is what triggers the "awful performance" warning in Zed.

## 6. Windows integration (`crates/gpui_windows`)

### 6.1 Backend selection

Add `enum RendererKind { DirectX, Software }` and `fn select_renderer_kind() -> (RendererKind, Option<DirectXDevices>)` called from `WindowsPlatform::new` (`crates/gpui_windows/src/platform.rs:102`), replacing the unconditional `DirectXDevices::new()`:

1. `GPUI_RENDERER=software` forces software; `GPUI_RENDERER=directx` forces DirectX (even on WARP, to keep the old behavior reachable for diagnosis).
2. Otherwise try `DirectXDevices::new()`. On error: software.
3. On success, read `IDXGIAdapter1::GetDesc1`. If `Flags & DXGI_ADAPTER_FLAG_SOFTWARE` (WARP / "Microsoft Basic Render Driver"): drop the device and use software.
4. Otherwise DirectX, unchanged.

Log the decision at info level. The result is stored on `WindowsPlatform` so that every window created later uses the same backend; mixing backends per window is not supported.

`WindowsPlatformState.directx_devices` becomes `Option`, which it already is in the headless case, and the device-lost machinery (`WM_GPUI_GPU_DEVICE_LOST`, `handle_gpu_device_lost` in `platform.rs:1302`, the `invalidate_devices` flag polled by the vsync thread) is skipped entirely in software mode.

### 6.2 `WindowsRenderer` enum

`WindowsWindowState.renderer: RefCell<DirectXRenderer>` (`crates/gpui_windows/src/window.rs:65`) becomes `RefCell<WindowsRenderer>`:

```rust
pub(crate) enum WindowsRenderer {
    DirectX(DirectXRenderer),
    Software(SoftwareWindowRenderer),
}
```

with methods forwarding the existing call surface so call sites change only in type:
- `draw(&Scene, WindowBackgroundAppearance)` (`window.rs:977`)
- `sprite_atlas()` (`window.rs:985`)
- `gpu_specs()` (`window.rs:993`)
- `resize(Size<DevicePixels>)` (`events.rs:213`)
- `handle_device_lost` / `mark_drawable` (`events.rs:1202-1246`): no-ops for software.

`SoftwareWindowRenderer` (new file `crates/gpui_windows/src/software_renderer.rs`) owns a `gpui_software::SoftwareRenderer` and an `HWND`. Its `draw` calls `renderer.draw(scene, force)` and then presents the returned damage rects.

`WindowsWindowState::new` (`window.rs:107`) takes the platform's `RendererKind` and the optional devices and constructs the right variant.

### 6.3 GDI presenter

Presentation is one GDI call per damage rect, straight from the framebuffer `Vec<u32>`:

```
BITMAPINFO { biWidth = width, biHeight = -height (top-down), biPlanes = 1, biBitCount = 32, biCompression = BI_RGB }
hdc = GetDC(hwnd)
for rect in damage:
    SetDIBitsToDevice(hdc, rect.x, rect.y, rect.w, rect.h,   // dest
                      rect.x, height - rect.y - rect.h,       // src (DIB coordinates are bottom-up in this call)
                      0, height, framebuffer.as_ptr(), &bmi, DIB_RGB_COLORS)
ReleaseDC(hwnd, hdc)
```

If profiling shows the per-scanline copy in `SetDIBitsToDevice` to matter, switch to a `CreateDIBSection` bitmap used as the framebuffer storage plus `BitBlt` from a memory DC; the framebuffer type supports external storage for that case.

Presenting only damage rects is what makes this good over RDP/AVD: the remote graphics pipeline encodes exactly the GDI regions that changed, instead of a full flip-model frame every time the caret blinks.

`WM_PAINT` handling stays as is (`draw_window` → `request_frame` → GPUI decides whether it is dirty). Windows may also send `WM_PAINT` for areas uncovered by another window; when `wParam`/`GetUpdateRect` reports a region, the software presenter re-presents that region from the framebuffer without re-rasterizing (the framebuffer always holds the complete last frame).

Software mode ignores `set_background_appearance` (`window.rs:~870`): no DWM composition attributes, no DirectComposition target, `background_appearance()` always reports `Opaque`. `is_subpixel_rendering_supported()` stays `true`.

Vsync is unchanged: `VSyncProvider` (`vsync.rs`) uses `DwmFlush`, not D3D, and GPUI only redraws when its invalidator is dirty.

Resize: `handle_size_change` (`events.rs:203`) calls `renderer.resize(device_size)`; for software that reallocates the framebuffer and forces a full draw on the next frame.

### 6.4 DirectWrite without a D3D device

`DirectWriteTextSystem::new(&DirectXDevices)` (`direct_write.rs:166`) uses D3D only for COLR emoji (`GPUState`, `rasterize_color` at `direct_write.rs:878`, `color_text_raster.hlsl`, `alpha_correction.hlsl`). Change it to `new(Option<&DirectXDevices>)` with `gpu_state: Option<GPUState>`. Add a CPU path in `rasterize_color`:

- for each color layer, `CreateAlphaTexture(DWRITE_TEXTURE_ALIASED_1x1)` as today gives an 8-bit mask and a `runColor`;
- composite into a BGRA straight-alpha buffer the size of `glyph_bounds`: `out = over(out, run_color * mask / 255)` (premultiplied accumulation, unpremultiply at the end), reproducing what `color_text_raster.hlsl` does on the GPU.

Everything else in the text system (font loading, layout, metrics, grayscale/ClearType rasterization) already runs on the CPU and is unchanged, which is what guarantees identical glyph metrics and bitmaps.

`handle_gpu_lost` becomes a no-op when `gpu_state` is `None`.

### 6.5 Font correction parameters

Move `FontInfo` / `get_font_info()` (`directx_renderer.rs:31`, `:737`) out of the DirectX renderer into a shared module in `gpui_windows` and expose it as `gpui_software::FontCorrection`, so both backends read the same DirectWrite rendering parameters (`GetGamma`, `GetGrayscaleEnhancedContrast`, `GetEnhancedContrast`, `GetPixelGeometry`).

## 7. Linux integration (`crates/gpui_linux`, second phase)

Selection: in the Wayland and X11 window constructors where `WgpuRenderer::new` is called (`crates/gpui_linux/src/linux/wayland/window.rs:574`, `x11/window.rs:733`), use `WgpuContext::new_rejecting_software` (already exists, `crates/gpui_wgpu/src/wgpu_context.rs:36`); if no non-CPU adapter is found or `GPUI_RENDERER=software`, construct the software renderer instead. The same enum-wrapper approach as Windows applies to the `renderer` fields.

Presenters:
- Wayland: a `wl_shm` pool with two `XRGB8888` buffers the size of the surface (double buffering is required because the compositor owns a buffer until `wl_buffer.release`); after `draw`, copy damage rects from the framebuffer into the free buffer, `wl_surface.attach`, `wl_surface.damage_buffer` per rect, `commit`. Fractional scale/viewport handling stays as in the wgpu path.
- X11: `xproto::put_image` (`ZPixmap`, depth 24) per damage rect, chunked to stay under the maximum request length; upgrade to MIT-SHM (`x11rb` `shm` feature) if profiling shows the copy matters.

Font correction on Linux: the wgpu renderer uses fixed `gamma_ratios` and contrast values (see `rendering_params` in `wgpu_renderer.rs`); the software renderer takes the same constants.

## 8. Required changes in `crates/gpui`

Small, additive:

- `Background` (`crates/gpui/src/color.rs:779`) has `pub(crate)` fields. Add public read accessors (`tag()`, `color_space()`, `solid()`, `gradient_angle_or_pattern_height()`, `colors()`) and make `BackgroundTag` public, so an out-of-tree renderer can lower it. The GPU renderers read the struct bytes directly and are unaffected.
- `TransformationMatrix::apply` exists; add `inverse()` for the transformed mono blit (or compute it in `gpui_software`).
- `Scene`, `PrimitiveBatch`, all primitive structs, `AtlasTile`, `ContentMask` are already public.
- `GpuSpecs` (`crates/gpui/src/gpui.rs:337`): no change; software mode reports as described in 5.10. Optionally add `pub renderer: &'static str` for diagnostics; not required.
- Optional, behind a `scene-recording` feature: `serde` derives on primitives so `gpui_software` can dump and replay real Zed scenes for benchmarks (section 10).

No changes to layout, text layout, pixel snapping, or element code.

## 9. Zed integration (`crates/zed`)

- `show_software_emulation_warning_if_needed` (`crates/zed/src/zed.rs:725`) keeps its logic; because the software renderer reports `is_software_emulated: false`, users on AVD no longer see the "Unsupported GPU" prompt. The prompt still appears when someone forces `GPUI_RENDERER=directx` on WARP.
- `zed --system-specs` / the "Copy System Specs" action already print `GpuSpecs`; the new `device_name`/`driver_info` make software mode visible in bug reports.
- Docs (`docs/src/windows.md`, `docs/src/linux.md`): describe automatic software mode, `GPUI_RENDERER`, and what lean mode drops.
- No settings-file switch initially; the env var is enough for testing and for users who want to force a mode. A `"renderer"` setting can follow once the mode is proven.

## 10. Testing and benchmarking

Headless pixel tests in `gpui_software` (no window needed; `Scene` is constructible directly with `insert_primitive` + `finish`):
- quad fill/border lowering produces the expected integer rects and colors;
- shadows, corner radii, wavy underlines produce no extra ops;
- text LUTs match a float port of the shader within ±1;
- subpixel blend, `is_bgr` swizzle, polychrome opacity;
- damage: changing one glyph dirties only its cells; unchanged frames produce empty damage; resize dirties everything;
- opaque cutoff: ops under a full-cell opaque fill are not executed (assert via a counting kernel table);
- AVX2 kernels vs scalar kernels on random rows.

Benchmarks (`benches/frame.rs`): synthetic 1080p and 4K "editor" scenes (about 8k monochrome glyphs, 200 quads, a few borders, 20 paths) measuring full redraw, single-line change, and full editor scroll, with thread count 1, 4, 16. Target on a modern EPYC core: full 1080p redraw under 8 ms single-threaded and under 2 ms on 8 threads; single-line change under 0.3 ms.

Scene replay (optional): with the `scene-recording` feature, `GPUI_RECORD_SCENES=<dir>` in the software renderer writes each frame's primitives to disk; a bench loads them to measure real Zed frames on the target VM.

Manual verification: `cargo run -p gpui --example text` and `--example hello_world`, `painting`, `gradient` with `GPUI_RENDERER=software`; then Zed itself on a Windows VM without a GPU and over RDP. `GPUI_SOFTWARE_STATS=1` logs per-frame lowering/bin/raster/present times and the damaged pixel count.

## 11. Delivery phases

Phase 1 — crate skeleton and full-frame rendering
- `gpui_software` with framebuffer, atlas, lowering, scalar kernels, text LUTs, vello_cpu paths; whole-window redraw every frame; headless tests and bench.
- `Background` accessors in `gpui`.

Phase 2 — Windows wiring behind an env var
- `WindowsRenderer` enum, `SoftwareWindowRenderer`, GDI presenter, `select_renderer_kind`, DirectWrite CPU COLR path, `FontCorrection` extraction.
- Ship enabled only via `GPUI_RENDERER=software`.

Phase 3 — incremental rendering
- Cell binning, hashes, damage rects, band-parallel raster with rayon, opaque cutoff. Presenter copies only damage.

Phase 4 — kernels
- AVX2 versions of `fill_opaque`, `fill_blend`, `blit_mono`, `blit_subpixel`, `blit_poly`; dispatch table; equivalence tests; benchmark-driven choice of LUT strategy in `blit_mono`.

Phase 5 — automatic selection and docs
- Software by default when no hardware adapter exists on Windows; warning behavior in `zed.rs` verified; docs.

Phase 6 — Linux
- Selection in Wayland/X11 window creation, `wl_shm` and `put_image` presenters.

Phases 1–2 produce something usable on AVD; phases 3–4 are where it becomes fast; phase 5 makes it the default; phase 6 extends it to llvmpipe-only Linux boxes.

## 12. Risks and open points

- vello_cpu API churn: 0.0.9 → 0.1 → 0.2 were breaking releases in quick succession. Confine all vello usage to `paths.rs` and pin the version; expect to touch that file on upgrades.
- `PixelFormat`/`PixmapMut` availability for compositing into a BGRA band slice must be confirmed against the pinned version; the RGBA scratch fallback is the safety net.
- `blit_mono` LUT strategy under AVX2 (gather vs. two-level shuffle) is a benchmark decision, not a design decision.
- Dropping shadows removes a depth cue for popovers and modals; all Zed popovers also draw a border, so readability is preserved. If it proves confusing in practice, a 1px darker border for shadowed elements is a two-line addition in `lower.rs`.
- Elements that animate every frame (spinners, progress indicators) keep dirtying their cells; that is expected and cheap.
- Transparent/blurred window appearances are not available in software mode by design.
- macOS is not a target; a dev-only presenter is optional and must not change macOS behavior for users.
