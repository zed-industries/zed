#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FlashClassification` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FlashClassification`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashClassification {
    Unclassified = "unclassified",
    Unknown = "unknown",
    Allowed = "allowed",
    Denied = "denied",
}
