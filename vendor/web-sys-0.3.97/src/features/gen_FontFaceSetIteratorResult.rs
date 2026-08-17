#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FontFaceSetIteratorResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetIteratorResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub type FontFaceSetIteratorResult;
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    #[wasm_bindgen(method, getter = "done")]
    pub fn get_done(this: &FontFaceSetIteratorResult) -> bool;
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    #[wasm_bindgen(method, setter = "done")]
    pub fn set_done(this: &FontFaceSetIteratorResult, val: bool);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &FontFaceSetIteratorResult) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &FontFaceSetIteratorResult, val: &::wasm_bindgen::JsValue);
}
impl FontFaceSetIteratorResult {
    #[doc = "Construct a new `FontFaceSetIteratorResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn new(done: bool, value: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_done(done);
        ret.set_value(value);
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
