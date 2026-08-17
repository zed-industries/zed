#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcPeerConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcPeerConnectionState {
    Closed = "closed",
    Failed = "failed",
    Disconnected = "disconnected",
    New = "new",
    Connecting = "connecting",
    Connected = "connected",
}
