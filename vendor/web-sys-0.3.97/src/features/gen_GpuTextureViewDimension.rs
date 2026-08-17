#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `GpuTextureViewDimension` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuTextureViewDimension`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuTextureViewDimension {
    N1d = "1d",
    N2d = "2d",
    N2dArray = "2d-array",
    Cube = "cube",
    CubeArray = "cube-array",
    N3d = "3d",
}
