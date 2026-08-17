#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AnimationPlayState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AnimationPlayState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayState {
    Idle = "idle",
    Running = "running",
    Paused = "paused",
    Finished = "finished",
}
