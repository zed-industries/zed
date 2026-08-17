#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `UserVerificationRequirement` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UserVerificationRequirement`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserVerificationRequirement {
    Required = "required",
    Preferred = "preferred",
    Discouraged = "discouraged",
}
