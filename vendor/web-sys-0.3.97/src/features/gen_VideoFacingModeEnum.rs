#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `VideoFacingModeEnum` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoFacingModeEnum`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFacingModeEnum {
    User = "user",
    Environment = "environment",
    Left = "left",
    Right = "right",
}
