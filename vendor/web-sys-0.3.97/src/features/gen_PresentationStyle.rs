#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PresentationStyle` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationStyle`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationStyle {
    Unspecified = "unspecified",
    Inline = "inline",
    Attachment = "attachment",
}
