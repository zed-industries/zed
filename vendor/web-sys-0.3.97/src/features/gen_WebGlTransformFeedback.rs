#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebGLTransformFeedback",
        typescript_type = "WebGLTransformFeedback"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlTransformFeedback` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLTransformFeedback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlTransformFeedback`*"]
    pub type WebGlTransformFeedback;
}
