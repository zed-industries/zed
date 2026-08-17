#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BinaryType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BinaryType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryType {
    Blob = "blob",
    Arraybuffer = "arraybuffer",
}
