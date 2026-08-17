#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `IdbTransactionMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbTransactionMode`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdbTransactionMode {
    Readonly = "readonly",
    Readwrite = "readwrite",
    Versionchange = "versionchange",
    Readwriteflush = "readwriteflush",
    Cleanup = "cleanup",
}
