#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeySystemStatus` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeySystemStatus`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKeySystemStatus {
    Available = "available",
    ApiDisabled = "api-disabled",
    CdmDisabled = "cdm-disabled",
    CdmNotSupported = "cdm-not-supported",
    CdmNotInstalled = "cdm-not-installed",
    CdmCreated = "cdm-created",
}
