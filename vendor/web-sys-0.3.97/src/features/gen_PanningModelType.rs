#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PanningModelType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PanningModelType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanningModelType {
    Equalpower = "equalpower",
    Hrtf = "HRTF",
}
