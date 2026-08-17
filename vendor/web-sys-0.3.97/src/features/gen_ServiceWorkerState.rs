#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ServiceWorkerState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ServiceWorkerState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceWorkerState {
    Parsed = "parsed",
    Installing = "installing",
    Installed = "installed",
    Activating = "activating",
    Activated = "activated",
    Redundant = "redundant",
}
