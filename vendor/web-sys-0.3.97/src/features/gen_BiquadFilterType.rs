#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BiquadFilterType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BiquadFilterType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiquadFilterType {
    Lowpass = "lowpass",
    Highpass = "highpass",
    Bandpass = "bandpass",
    Lowshelf = "lowshelf",
    Highshelf = "highshelf",
    Peaking = "peaking",
    Notch = "notch",
    Allpass = "allpass",
}
