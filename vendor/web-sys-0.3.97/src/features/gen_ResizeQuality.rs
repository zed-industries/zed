#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ResizeQuality` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResizeQuality`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeQuality {
    Pixelated = "pixelated",
    Low = "low",
    Medium = "medium",
    High = "high",
}
