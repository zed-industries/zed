#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `GpuFeatureName` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuFeatureName`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuFeatureName {
    CoreFeaturesAndLimits = "core-features-and-limits",
    DepthClipControl = "depth-clip-control",
    Depth32floatStencil8 = "depth32float-stencil8",
    TextureCompressionBc = "texture-compression-bc",
    TextureCompressionBcSliced3d = "texture-compression-bc-sliced-3d",
    TextureCompressionEtc2 = "texture-compression-etc2",
    TextureCompressionAstc = "texture-compression-astc",
    TextureCompressionAstcSliced3d = "texture-compression-astc-sliced-3d",
    TimestampQuery = "timestamp-query",
    IndirectFirstInstance = "indirect-first-instance",
    ShaderF16 = "shader-f16",
    Rg11b10ufloatRenderable = "rg11b10ufloat-renderable",
    Bgra8unormStorage = "bgra8unorm-storage",
    Float32Filterable = "float32-filterable",
    Float32Blendable = "float32-blendable",
    ClipDistances = "clip-distances",
    DualSourceBlending = "dual-source-blending",
    Subgroups = "subgroups",
    TextureFormatsTier1 = "texture-formats-tier1",
    TextureFormatsTier2 = "texture-formats-tier2",
    PrimitiveIndex = "primitive-index",
    TextureComponentSwizzle = "texture-component-swizzle",
}
