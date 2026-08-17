#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceConnectionState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcIceConnectionState {
    New = "new",
    Checking = "checking",
    Connected = "connected",
    Completed = "completed",
    Failed = "failed",
    Disconnected = "disconnected",
    Closed = "closed",
}
