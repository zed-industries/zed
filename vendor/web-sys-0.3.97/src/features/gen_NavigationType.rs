#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `NavigationType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NavigationType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationType {
    Navigate = "navigate",
    Reload = "reload",
    BackForward = "back_forward",
    Prerender = "prerender",
}
