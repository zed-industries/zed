#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `OverSampleType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OverSampleType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverSampleType {
    None = "none",
    N2x = "2x",
    N4x = "4x",
}
