#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FontFaceLoadStatus` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FontFaceLoadStatus`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFaceLoadStatus {
    Unloaded = "unloaded",
    Loading = "loading",
    Loaded = "loaded",
    Error = "error",
}
