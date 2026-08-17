#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ScreenColorGamut` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScreenColorGamut`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenColorGamut {
    Srgb = "srgb",
    P3 = "p3",
    Rec2020 = "rec2020",
}
