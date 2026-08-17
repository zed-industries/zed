#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GamepadHand` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GamepadHand`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadHand {
    None = "",
    Left = "left",
    Right = "right",
}
