#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ServiceWorkerUpdateViaCache` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ServiceWorkerUpdateViaCache`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceWorkerUpdateViaCache {
    Imports = "imports",
    All = "all",
    None = "none",
}
