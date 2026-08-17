#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebGLVertexArrayObject",
        typescript_type = "WebGLVertexArrayObject"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlVertexArrayObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLVertexArrayObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlVertexArrayObject`*"]
    pub type WebGlVertexArrayObject;
}
