#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FontFaceSetLoadStatus` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadStatus`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFaceSetLoadStatus {
    Loading = "loading",
    Loaded = "loaded",
}
