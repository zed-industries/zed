#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaSourceReadyState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaSourceReadyState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaSourceReadyState {
    Closed = "closed",
    Open = "open",
    Ended = "ended",
}
