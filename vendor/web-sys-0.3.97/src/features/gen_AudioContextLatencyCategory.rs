#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AudioContextLatencyCategory` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioContextLatencyCategory`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioContextLatencyCategory {
    Balanced = "balanced",
    Interactive = "interactive",
    Playback = "playback",
}
