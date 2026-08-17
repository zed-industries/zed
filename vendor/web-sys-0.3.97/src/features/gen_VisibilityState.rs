#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `VisibilityState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VisibilityState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityState {
    Hidden = "hidden",
    Visible = "visible",
}
