#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `VideoPixelFormat` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoPixelFormat`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoPixelFormat {
    I420 = "I420",
    I420p10 = "I420P10",
    I420p12 = "I420P12",
    I420a = "I420A",
    I420ap10 = "I420AP10",
    I420ap12 = "I420AP12",
    I422 = "I422",
    I422p10 = "I422P10",
    I422p12 = "I422P12",
    I422a = "I422A",
    I422ap10 = "I422AP10",
    I422ap12 = "I422AP12",
    I444 = "I444",
    I444p10 = "I444P10",
    I444p12 = "I444P12",
    I444a = "I444A",
    I444ap10 = "I444AP10",
    I444ap12 = "I444AP12",
    Nv12 = "NV12",
    Rgba = "RGBA",
    Rgbx = "RGBX",
    Bgra = "BGRA",
    Bgrx = "BGRX",
}
