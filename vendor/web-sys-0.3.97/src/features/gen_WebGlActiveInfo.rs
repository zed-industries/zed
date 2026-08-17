#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebGLActiveInfo",
        typescript_type = "WebGLActiveInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlActiveInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    pub type WebGlActiveInfo;
    #[wasm_bindgen(method, getter, js_class = "WebGLActiveInfo", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    pub fn size(this: &WebGlActiveInfo) -> i32;
    #[wasm_bindgen(method, getter, js_class = "WebGLActiveInfo", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    pub fn type_(this: &WebGlActiveInfo) -> u32;
    #[wasm_bindgen(method, getter, js_class = "WebGLActiveInfo", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`*"]
    pub fn name(this: &WebGlActiveInfo) -> ::alloc::string::String;
}
