#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcPriorityType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcPriorityType {
    VeryLow = "very-low",
    Low = "low",
    Medium = "medium",
    High = "high",
}
