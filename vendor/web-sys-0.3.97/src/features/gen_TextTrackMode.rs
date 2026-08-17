#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `TextTrackMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TextTrackMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextTrackMode {
    Disabled = "disabled",
    Hidden = "hidden",
    Showing = "showing",
}
