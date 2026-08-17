#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PresentationConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationConnectionState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationConnectionState {
    Connecting = "connecting",
    Connected = "connected",
    Closed = "closed",
    Terminated = "terminated",
}
