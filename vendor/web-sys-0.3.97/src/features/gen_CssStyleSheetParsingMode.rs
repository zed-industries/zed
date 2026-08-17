#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `CssStyleSheetParsingMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CssStyleSheetParsingMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssStyleSheetParsingMode {
    Author = "author",
    User = "user",
    Agent = "agent",
}
