#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `DirectionSetting` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DirectionSetting`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionSetting {
    None = "",
    Rl = "rl",
    Lr = "lr",
}
