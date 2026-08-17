#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BitrateMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BitrateMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitrateMode {
    Constant = "constant",
    Variable = "variable",
}
