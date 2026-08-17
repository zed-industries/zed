#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ReferrerPolicy` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferrerPolicy {
    None = "",
    NoReferrer = "no-referrer",
    NoReferrerWhenDowngrade = "no-referrer-when-downgrade",
    Origin = "origin",
    OriginWhenCrossOrigin = "origin-when-cross-origin",
    UnsafeUrl = "unsafe-url",
    SameOrigin = "same-origin",
    StrictOrigin = "strict-origin",
    StrictOriginWhenCrossOrigin = "strict-origin-when-cross-origin",
}
