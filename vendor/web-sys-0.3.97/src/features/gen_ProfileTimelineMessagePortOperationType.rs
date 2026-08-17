#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ProfileTimelineMessagePortOperationType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ProfileTimelineMessagePortOperationType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileTimelineMessagePortOperationType {
    SerializeData = "serializeData",
    DeserializeData = "deserializeData",
}
