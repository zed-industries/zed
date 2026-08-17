#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MidiPortDeviceState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MidiPortDeviceState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiPortDeviceState {
    Disconnected = "disconnected",
    Connected = "connected",
}
