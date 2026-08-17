#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PresentationConnectionBinaryType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationConnectionBinaryType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationConnectionBinaryType {
    Blob = "blob",
    Arraybuffer = "arraybuffer",
}
