#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
#[doc = "The `AudioSampleFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioSampleFormat`*"]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSampleFormat {
    U8 = "u8",
    S16 = "s16",
    S32 = "s32",
    F32 = "f32",
    U8Planar = "u8-planar",
    S16Planar = "s16-planar",
    S32Planar = "s32-planar",
    F32Planar = "f32-planar",
}
