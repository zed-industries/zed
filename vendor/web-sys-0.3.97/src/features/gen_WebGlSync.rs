#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebGLSync",
        typescript_type = "WebGLSync"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlSync` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlSync`*"]
    pub type WebGlSync;
}
