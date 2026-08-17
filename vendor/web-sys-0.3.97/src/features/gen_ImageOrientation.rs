#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ImageOrientation` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ImageOrientation`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageOrientation {
    FromImage = "from-image",
    FlipY = "flipY",
}
