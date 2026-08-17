#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SelectionMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SelectionMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Select = "select",
    Start = "start",
    End = "end",
    Preserve = "preserve",
}
