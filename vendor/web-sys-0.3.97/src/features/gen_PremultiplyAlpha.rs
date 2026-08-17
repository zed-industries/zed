#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PremultiplyAlpha` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PremultiplyAlpha`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PremultiplyAlpha {
    None = "none",
    Premultiply = "premultiply",
    Default = "default",
}
