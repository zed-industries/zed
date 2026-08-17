#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "XmlHttpRequestEventTarget",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "XMLHttpRequestUpload",
        typescript_type = "XMLHttpRequestUpload"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlHttpRequestUpload` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestUpload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequestUpload`*"]
    pub type XmlHttpRequestUpload;
}
