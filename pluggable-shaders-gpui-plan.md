# Pluggable Shaders for GPUI

## Overview

This document outlines a plan for adding pluggable shader support to GPUI, allowing users to define custom shaders, pipelines, and primitives that integrate with GPUI's rendering system.

## Goals

- Allow users to define custom WGSL shaders for the Blade backend
- Allow users to register custom render pipelines with GPUI
- Allow users to define custom primitive types with `#[repr(C)]` structs for GPU data
- Integrate custom primitives into GPUI's batching and draw-order system
- Provide a clean, type-safe API for painting custom primitives during element rendering

## Non-Goals

- Supporting Metal or DirectX shader backends initially (focus on Blade/WGSL)
- Automatic shader hot-reloading
- Providing a shader authoring GUI
- Multi-pass rendering for custom primitives (can be added later)

## Current State

### Rendering Architecture

GPUI uses a multi-backend rendering system:
- **Blade** (Linux, FreeBSD, macOS with feature flag) - uses WGSL shaders
- **Metal** (macOS default) - uses Metal shaders
- **DirectX** (Windows) - uses HLSL shaders

### Key Components

1. **Scene** (`crates/gpui/src/scene.rs:24-36`): Holds all primitives to render
   - Contains typed vectors for each primitive type (quads, shadows, paths, etc.)
   - Manages draw ordering via `DrawOrder` (u32)
   - Provides `batches()` iterator for efficient GPU rendering

2. **Primitive Types** (`scene.rs:211-220`): 8 built-in types
   - Shadow, Quad, Path, Underline
   - MonochromeSprite, SubpixelSprite, PolychromeSprite
   - Surface (video surfaces, macOS only)

3. **BladeRenderer** (`blade_renderer.rs:369-386`):
   - Owns pipelines, instance buffers, atlas, textures
   - `draw()` method iterates batches and dispatches to appropriate pipelines

4. **BladePipelines** (`blade_renderer.rs:140-150`):
   - Struct holding all render pipelines
   - Created at window initialization with shader source

5. **Shader Data** (`blade_renderer.rs:56-123`):
   - Uses `#[derive(blade_macros::ShaderData)]` for GPU bindings
   - GlobalParams uniform, storage buffers for instance data

### Data Flow

```
Window::paint_*()
  → Scene::insert_primitive()
  → Scene::finish() sorts by order
  → BladeRenderer::draw()
    → scene.batches() iterator
    → For each batch: bind pipeline, upload data, draw
```

## Proposed Design

### Architecture

The design adds an extensible primitive system alongside the existing built-in primitives:

```
┌─────────────────────────────────────────────────────────────┐
│                      User Code                               │
│  - Define CustomPrimitive struct (#[repr(C)])               │
│  - Provide WGSL shader source                               │
│  - Register with GPUI via CustomShaderDescriptor           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  CustomShaderRegistry                        │
│  - Stores registered shader descriptors                     │
│  - Creates pipelines on renderer initialization             │
│  - Maps CustomShaderId → pipeline + metadata                │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Scene (Extended)                        │
│  - Existing primitive vectors                               │
│  - New: custom_primitives: HashMap<CustomShaderId, Vec<u8>> │
│  - BatchIterator yields CustomBatch alongside built-ins     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  BladeRenderer (Extended)                    │
│  - Existing pipelines                                       │
│  - New: custom_pipelines: HashMap<CustomShaderId, Pipeline> │
│  - draw() handles PrimitiveBatch::Custom                    │
└─────────────────────────────────────────────────────────────┘
```

### API Surface

#### 1. Custom Shader Descriptor

```rust
/// Describes a custom shader and its pipeline configuration.
pub struct CustomShaderDescriptor {
    /// Unique identifier for this shader
    pub name: &'static str,

    /// WGSL shader source code
    pub shader_source: &'static str,

    /// Vertex shader entry point name
    pub vertex_entry: &'static str,

    /// Fragment shader entry point name
    pub fragment_entry: &'static str,

    /// Size of each instance in bytes (must match #[repr(C)] struct)
    pub instance_size: usize,

    /// Blend mode for this shader
    pub blend_mode: CustomBlendMode,

    /// Primitive topology (typically TriangleStrip for instanced quads)
    pub topology: PrimitiveTopology,
}

pub enum CustomBlendMode {
    /// Standard alpha blending
    Alpha,
    /// Premultiplied alpha
    PremultipliedAlpha,
    /// Additive blending
    Additive,
    /// No blending (opaque)
    Opaque,
}

pub enum PrimitiveTopology {
    TriangleStrip,
    TriangleList,
}
```

#### 2. Custom Shader Handle

```rust
/// Opaque handle to a registered custom shader.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomShaderId(u32);
```

#### 3. Custom Primitive Trait

```rust
/// Trait for types that can be rendered as custom primitives.
///
/// # Safety
/// The implementing type must be `#[repr(C)]` and match the expected
/// layout in the WGSL shader.
pub unsafe trait CustomPrimitive: Pod + Zeroable + Clone {
    /// Returns the bounds of this primitive for clipping/culling.
    fn bounds(&self) -> Bounds<ScaledPixels>;

    /// Returns the content mask for clipping.
    fn content_mask(&self) -> ContentMask<ScaledPixels>;

    /// Returns the draw order for sorting.
    fn order(&self) -> DrawOrder;

    /// Sets the draw order.
    fn set_order(&mut self, order: DrawOrder);
}
```

#### 4. Registration API (on App)

```rust
impl App {
    /// Register a custom shader with GPUI.
    ///
    /// This should be called during app initialization before any windows
    /// are created. The shader will be compiled when windows are opened.
    pub fn register_custom_shader(
        &mut self,
        descriptor: CustomShaderDescriptor,
    ) -> CustomShaderId;
}
```

#### 5. Painting API (on Window)

```rust
impl Window {
    /// Paint a custom primitive into the scene.
    ///
    /// The primitive type must match the shader it was registered with.
    /// This method should only be called during the paint phase.
    pub fn paint_custom<P: CustomPrimitive>(
        &mut self,
        shader_id: CustomShaderId,
        primitive: P,
    );

    /// Paint multiple custom primitives of the same type.
    ///
    /// More efficient than calling paint_custom repeatedly.
    pub fn paint_custom_batch<P: CustomPrimitive>(
        &mut self,
        shader_id: CustomShaderId,
        primitives: &[P],
    );
}
```

### Example Usage

```rust
// 1. Define the primitive struct (must match shader layout)
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GlowQuad {
    order: u32,
    _pad: u32,
    bounds: Bounds<ScaledPixels>,
    content_mask: ContentMask<ScaledPixels>,
    color: Hsla,
    glow_radius: f32,
    glow_intensity: f32,
}

unsafe impl CustomPrimitive for GlowQuad {
    fn bounds(&self) -> Bounds<ScaledPixels> { self.bounds }
    fn content_mask(&self) -> ContentMask<ScaledPixels> { self.content_mask }
    fn order(&self) -> DrawOrder { self.order }
    fn set_order(&mut self, order: DrawOrder) { self.order = order; }
}

// 2. Define the shader
const GLOW_SHADER: &str = r#"
struct GlowQuad {
    order: u32,
    pad: u32,
    bounds: Bounds,
    content_mask: Bounds,
    color: Hsla,
    glow_radius: f32,
    glow_intensity: f32,
}

var<storage, read> b_glow_quads: array<GlowQuad>;

struct GlowVarying {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec4<f32>,
    // ... more fields
}

@vertex
fn vs_glow(@builtin(vertex_index) vertex_id: u32,
           @builtin(instance_index) instance_id: u32) -> GlowVarying {
    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
    let quad = b_glow_quads[instance_id];
    // ... transform and output
}

@fragment
fn fs_glow(input: GlowVarying) -> @location(0) vec4<f32> {
    // ... glow effect calculation
}
"#;

// 3. Register at app startup
fn main() {
    App::new().run(|cx| {
        let glow_shader = cx.register_custom_shader(CustomShaderDescriptor {
            name: "glow_quad",
            shader_source: GLOW_SHADER,
            vertex_entry: "vs_glow",
            fragment_entry: "fs_glow",
            instance_size: std::mem::size_of::<GlowQuad>(),
            blend_mode: CustomBlendMode::PremultipliedAlpha,
            topology: PrimitiveTopology::TriangleStrip,
        });

        // Store shader_id somewhere accessible (e.g., Global)
        cx.set_global(GlowShader(glow_shader));

        // ... rest of app setup
    });
}

// 4. Use during element painting
impl Render for MyElement {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child("Hello")
            .on_paint(cx.listener(|this, _bounds, window, cx| {
                let shader_id = cx.global::<GlowShader>().0;
                window.paint_custom(shader_id, GlowQuad {
                    order: 0,
                    _pad: 0,
                    bounds: /* ... */,
                    content_mask: window.content_mask().scale(window.scale_factor()),
                    color: hsla(0.5, 1.0, 0.5, 1.0),
                    glow_radius: 10.0,
                    glow_intensity: 0.8,
                });
            }))
    }
}
```

### Implementation Steps

1. **[DONE] Add CustomShaderDescriptor and CustomShaderId types** (`crates/gpui/src/custom_shader.rs`)
   - Define the descriptor struct
   - Define the ID type
   - Define CustomPrimitive trait
   - Added `GPUI_SHADER_HEADER` constant with common shader utilities

2. **[DONE] Add CustomShaderRegistry** (`crates/gpui/src/custom_shader.rs`)
   - Store registered descriptors
   - Provide lookup by ID
   - Track which shaders need pipelines created

3. **[DONE] Extend App with registration API**
   - Add `register_custom_shader()` method
   - Store registry in App state (`custom_shader_registry` field)

4. **[DONE] Extend Scene with custom primitive storage** (`crates/gpui/src/scene.rs`)
   - Add `custom_primitives: HashMap<CustomShaderId, CustomPrimitiveBuffer>`
   - Add `insert_custom_primitive()` method
   - Add `custom_batches()` method to yield custom batches separately

5. **[DONE] Add PrimitiveBatch::Custom variant** (`crates/gpui/src/scene.rs`)
   ```rust
   Custom {
       shader_id: CustomShaderId,
       data: &'a [u8],
       instance_count: u32,
       vertices_per_instance: u32,
   }
   ```

6. **[DONE] Extend BladeRenderer** (`crates/gpui/src/platform/blade/blade_renderer.rs`)
   - Add `custom_pipelines: HashMap<CustomShaderId, CustomPipelineEntry>`
   - Add `register_custom_shader()` method to create pipelines
   - Handle `PrimitiveBatch::Custom` in `draw()` via `custom_batches()` iterator
   - Add `ShaderCustomData` struct for GPU bindings

7. **[DONE] Add Window painting API** (`crates/gpui/src/window.rs`)
   - Add `paint_custom()` method

8. **[DONE] Create helper shader functions module**
   - Export common WGSL functions via `GPUI_SHADER_HEADER` constant
   - Includes: GlobalParams, Bounds, Hsla, to_device_position, hsla_to_rgba, blend_color, etc.

9. **[TODO] Add tests and examples**
   - Unit tests for registration and painting
   - Example custom shader (e.g., glow effect, noise pattern)

10. **[TODO] Wire up shader registration from App to Renderer**
    - Currently renderer has `register_custom_shader()` but needs to be called
    - Need to plumb shader registry to window creation

## Open Questions

1. **Global vs Per-Window Shader Registration?**
   - Current design: Global registration, per-window pipeline creation
   - Alternative: Per-window registration (more flexible, more complex)
   - **Decision**: Global registration is simpler and matches most use cases

2. **How to handle shader compilation errors?**
   - Option A: Panic on invalid shader (current built-in behavior)
   - Option B: Return Result from registration
   - Option C: Log error and skip rendering
   - **Recommendation**: Return Result, let users handle errors

3. **Should custom primitives participate in layer ordering?**
   - Built-in primitives use `layer_stack` for z-ordering
   - Custom primitives should integrate with this
   - **Decision**: Yes, use the same DrawOrder system

4. **Texture access for custom shaders?**
   - Some custom effects need textures (e.g., noise, patterns)
   - Could expose atlas access or allow custom texture uploads
   - **Recommendation**: Add optional texture binding support in v2

5. **Metal/DirectX support?**
   - Initial implementation is Blade-only
   - Could add abstractions for cross-platform shaders later
   - **Recommendation**: Start with Blade, add others based on demand

## References

- `crates/gpui/src/scene.rs` - Scene and primitive definitions
- `crates/gpui/src/platform/blade/blade_renderer.rs` - Blade rendering implementation
- `crates/gpui/src/platform/blade/shaders.wgsl` - Built-in WGSL shaders
- `crates/gpui/src/window.rs` - Window painting API
- [blade_graphics crate](https://docs.rs/blade-graphics) - GPU abstraction layer
- [WGSL specification](https://www.w3.org/TR/WGSL/) - WebGPU Shading Language
