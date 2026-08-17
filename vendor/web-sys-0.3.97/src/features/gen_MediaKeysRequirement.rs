#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeysRequirement` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeysRequirement`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKeysRequirement {
    Required = "required",
    Optional = "optional",
    NotAllowed = "not-allowed",
}
