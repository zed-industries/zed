#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WorkerType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WorkerType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerType {
    Classic = "classic",
    Module = "module",
}
