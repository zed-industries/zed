#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `CookieSameSite` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CookieSameSite`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookieSameSite {
    Strict = "strict",
    Lax = "lax",
    None = "none",
}
