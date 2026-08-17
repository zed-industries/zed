#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PermissionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PermissionState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionState {
    Granted = "granted",
    Denied = "denied",
    Prompt = "prompt",
}
