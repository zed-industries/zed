#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AudioContextState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioContextState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioContextState {
    Suspended = "suspended",
    Running = "running",
    Closed = "closed",
}
