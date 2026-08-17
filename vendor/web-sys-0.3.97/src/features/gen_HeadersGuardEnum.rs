#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `HeadersGuardEnum` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HeadersGuardEnum`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadersGuardEnum {
    None = "none",
    Request = "request",
    RequestNoCors = "request-no-cors",
    Response = "response",
    Immutable = "immutable",
}
