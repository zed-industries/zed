#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `TokenBindingStatus` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TokenBindingStatus`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenBindingStatus {
    Present = "present",
    Supported = "supported",
}
