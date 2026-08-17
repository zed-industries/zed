#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AlignSetting` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AlignSetting`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignSetting {
    Start = "start",
    Center = "center",
    End = "end",
    Left = "left",
    Right = "right",
}
