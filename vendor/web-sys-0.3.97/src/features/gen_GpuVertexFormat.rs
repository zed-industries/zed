#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `GpuVertexFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuVertexFormat`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVertexFormat {
    Uint8 = "uint8",
    Uint8x2 = "uint8x2",
    Uint8x4 = "uint8x4",
    Sint8 = "sint8",
    Sint8x2 = "sint8x2",
    Sint8x4 = "sint8x4",
    Unorm8 = "unorm8",
    Unorm8x2 = "unorm8x2",
    Unorm8x4 = "unorm8x4",
    Snorm8 = "snorm8",
    Snorm8x2 = "snorm8x2",
    Snorm8x4 = "snorm8x4",
    Uint16 = "uint16",
    Uint16x2 = "uint16x2",
    Uint16x4 = "uint16x4",
    Sint16 = "sint16",
    Sint16x2 = "sint16x2",
    Sint16x4 = "sint16x4",
    Unorm16 = "unorm16",
    Unorm16x2 = "unorm16x2",
    Unorm16x4 = "unorm16x4",
    Snorm16 = "snorm16",
    Snorm16x2 = "snorm16x2",
    Snorm16x4 = "snorm16x4",
    Float16 = "float16",
    Float16x2 = "float16x2",
    Float16x4 = "float16x4",
    Float32 = "float32",
    Float32x2 = "float32x2",
    Float32x3 = "float32x3",
    Float32x4 = "float32x4",
    Uint32 = "uint32",
    Uint32x2 = "uint32x2",
    Uint32x3 = "uint32x3",
    Uint32x4 = "uint32x4",
    Sint32 = "sint32",
    Sint32x2 = "sint32x2",
    Sint32x3 = "sint32x3",
    Sint32x4 = "sint32x4",
    Unorm1010102 = "unorm10-10-10-2",
    Unorm8x4Bgra = "unorm8x4-bgra",
}
