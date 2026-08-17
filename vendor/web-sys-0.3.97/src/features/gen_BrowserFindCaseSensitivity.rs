#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BrowserFindCaseSensitivity` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BrowserFindCaseSensitivity`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserFindCaseSensitivity {
    CaseSensitive = "case-sensitive",
    CaseInsensitive = "case-insensitive",
}
