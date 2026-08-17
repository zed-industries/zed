#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ProfileTimelineWorkerOperationType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ProfileTimelineWorkerOperationType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileTimelineWorkerOperationType {
    SerializeDataOffMainThread = "serializeDataOffMainThread",
    SerializeDataOnMainThread = "serializeDataOnMainThread",
    DeserializeDataOffMainThread = "deserializeDataOffMainThread",
    DeserializeDataOnMainThread = "deserializeDataOnMainThread",
}
