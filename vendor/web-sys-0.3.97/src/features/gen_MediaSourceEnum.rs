#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaSourceEnum` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaSourceEnum`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaSourceEnum {
    Camera = "camera",
    Screen = "screen",
    Application = "application",
    Window = "window",
    Browser = "browser",
    Microphone = "microphone",
    AudioCapture = "audioCapture",
    Other = "other",
}
