#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MidiPortConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MidiPortConnectionState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiPortConnectionState {
    Open = "open",
    Closed = "closed",
    Pending = "pending",
}
