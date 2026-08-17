#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceTransportPolicy` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceTransportPolicy`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcIceTransportPolicy {
    Relay = "relay",
    All = "all",
}
