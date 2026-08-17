#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeyStatus` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeyStatus`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKeyStatus {
    Usable = "usable",
    Expired = "expired",
    Released = "released",
    OutputRestricted = "output-restricted",
    OutputDownscaled = "output-downscaled",
    StatusPending = "status-pending",
    InternalError = "internal-error",
}
