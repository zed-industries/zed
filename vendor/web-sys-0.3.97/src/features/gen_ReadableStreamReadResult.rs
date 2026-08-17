#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ReadableStreamReadResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamReadResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    pub type ReadableStreamReadResult;
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    #[wasm_bindgen(method, getter = "done")]
    pub fn get_done(this: &ReadableStreamReadResult) -> Option<bool>;
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    #[wasm_bindgen(method, setter = "done")]
    pub fn set_done(this: &ReadableStreamReadResult, val: bool);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &ReadableStreamReadResult) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &ReadableStreamReadResult, val: &::wasm_bindgen::JsValue);
}
impl ReadableStreamReadResult {
    #[doc = "Construct a new `ReadableStreamReadResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_done()` instead."]
    pub fn done(&mut self, val: bool) -> &mut Self {
        self.set_done(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for ReadableStreamReadResult {
    fn default() -> Self {
        Self::new()
    }
}
