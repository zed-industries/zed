#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `VideoColorPrimaries` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoColorPrimaries`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoColorPrimaries {
    Bt709 = "bt709",
    Bt470bg = "bt470bg",
    Smpte170m = "smpte170m",
    Bt2020 = "bt2020",
    Smpte432 = "smpte432",
}
