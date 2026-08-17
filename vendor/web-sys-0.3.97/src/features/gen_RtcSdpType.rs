#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcSdpType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcSdpType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcSdpType {
    Offer = "offer",
    Pranswer = "pranswer",
    Answer = "answer",
    Rollback = "rollback",
}
