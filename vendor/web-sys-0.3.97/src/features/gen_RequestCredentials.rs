#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RequestCredentials` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestCredentials`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestCredentials {
    Omit = "omit",
    SameOrigin = "same-origin",
    Include = "include",
}
