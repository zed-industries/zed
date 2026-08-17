#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeySessionType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeySessionType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKeySessionType {
    Temporary = "temporary",
    PersistentLicense = "persistent-license",
}
