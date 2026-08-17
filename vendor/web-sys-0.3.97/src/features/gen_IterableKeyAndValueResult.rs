#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "IterableKeyAndValueResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IterableKeyAndValueResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    pub type IterableKeyAndValueResult;
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    #[wasm_bindgen(method, getter = "done")]
    pub fn get_done(this: &IterableKeyAndValueResult) -> Option<bool>;
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    #[wasm_bindgen(method, setter = "done")]
    pub fn set_done(this: &IterableKeyAndValueResult, val: bool);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &IterableKeyAndValueResult) -> Option<::js_sys::Array>;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &IterableKeyAndValueResult, val: &::wasm_bindgen::JsValue);
}
impl IterableKeyAndValueResult {
    #[doc = "Construct a new `IterableKeyAndValueResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
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
impl Default for IterableKeyAndValueResult {
    fn default() -> Self {
        Self::new()
    }
}
