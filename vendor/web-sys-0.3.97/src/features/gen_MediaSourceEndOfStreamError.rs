#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaSourceEndOfStreamError` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaSourceEndOfStreamError`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaSourceEndOfStreamError {
    Network = "network",
    Decode = "decode",
}
