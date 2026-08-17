#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ScrollBehavior` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    Auto = "auto",
    Instant = "instant",
    Smooth = "smooth",
}
