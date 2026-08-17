#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MidiPortType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MidiPortType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiPortType {
    Input = "input",
    Output = "output",
}
