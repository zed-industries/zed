#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `DomRequestReadyState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomRequestReadyState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomRequestReadyState {
    Pending = "pending",
    Done = "done",
}
