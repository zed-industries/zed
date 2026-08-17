#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SourceBufferAppendMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SourceBufferAppendMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceBufferAppendMode {
    Segments = "segments",
    Sequence = "sequence",
}
