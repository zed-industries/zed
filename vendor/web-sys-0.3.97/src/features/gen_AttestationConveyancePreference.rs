#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AttestationConveyancePreference` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestationConveyancePreference {
    None = "none",
    Indirect = "indirect",
    Direct = "direct",
    Enterprise = "enterprise",
}
