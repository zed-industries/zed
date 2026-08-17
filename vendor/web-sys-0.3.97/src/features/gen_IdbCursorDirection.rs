#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `IdbCursorDirection` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdbCursorDirection {
    Next = "next",
    Nextunique = "nextunique",
    Prev = "prev",
    Prevunique = "prevunique",
}
