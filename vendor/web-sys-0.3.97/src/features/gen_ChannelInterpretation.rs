#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ChannelInterpretation` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelInterpretation {
    Speakers = "speakers",
    Discrete = "discrete",
}
