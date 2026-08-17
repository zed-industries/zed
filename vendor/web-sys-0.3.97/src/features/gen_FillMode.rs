#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FillMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FillMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillMode {
    None = "none",
    Forwards = "forwards",
    Backwards = "backwards",
    Both = "both",
    Auto = "auto",
}
