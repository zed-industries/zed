#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `XmlHttpRequestResponseType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `XmlHttpRequestResponseType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlHttpRequestResponseType {
    None = "",
    Arraybuffer = "arraybuffer",
    Blob = "blob",
    Document = "document",
    Json = "json",
    Text = "text",
}
