#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `CheckerboardReason` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckerboardReason {
    Severe = "severe",
    Recent = "recent",
}
