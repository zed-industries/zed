#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcBundlePolicy` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcBundlePolicy {
    Balanced = "balanced",
    MaxCompat = "max-compat",
    MaxBundle = "max-bundle",
}
