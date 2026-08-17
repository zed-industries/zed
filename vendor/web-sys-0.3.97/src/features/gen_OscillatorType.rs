#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `OscillatorType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `OscillatorType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OscillatorType {
    Sine = "sine",
    Square = "square",
    Sawtooth = "sawtooth",
    Triangle = "triangle",
    Custom = "custom",
}
