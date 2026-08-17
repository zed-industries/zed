#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RequestCache` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestCache`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestCache {
    Default = "default",
    NoStore = "no-store",
    Reload = "reload",
    NoCache = "no-cache",
    ForceCache = "force-cache",
    OnlyIfCached = "only-if-cached",
}
