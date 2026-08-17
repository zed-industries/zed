#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `VideoMatrixCoefficients` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoMatrixCoefficients`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoMatrixCoefficients {
    Rgb = "rgb",
    Bt709 = "bt709",
    Bt470bg = "bt470bg",
    Smpte170m = "smpte170m",
    Bt2020Ncl = "bt2020-ncl",
}
