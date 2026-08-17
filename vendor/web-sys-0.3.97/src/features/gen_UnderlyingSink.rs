#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "UnderlyingSink")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSink` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub type UnderlyingSink;
    #[doc = "Get the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, getter = "abort")]
    pub fn get_abort(this: &UnderlyingSink) -> Option<::js_sys::Function>;
    #[doc = "Change the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, setter = "abort")]
    pub fn set_abort(this: &UnderlyingSink, val: &::js_sys::Function);
    #[doc = "Get the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, getter = "close")]
    pub fn get_close(this: &UnderlyingSink) -> Option<::js_sys::Function>;
    #[doc = "Change the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, setter = "close")]
    pub fn set_close(this: &UnderlyingSink, val: &::js_sys::Function);
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, getter = "start")]
    pub fn get_start(this: &UnderlyingSink) -> Option<::js_sys::Function>;
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, setter = "start")]
    pub fn set_start(this: &UnderlyingSink, val: &::js_sys::Function);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &UnderlyingSink) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &UnderlyingSink, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, getter = "write")]
    pub fn get_write(this: &UnderlyingSink) -> Option<::js_sys::Function>;
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    #[wasm_bindgen(method, setter = "write")]
    pub fn set_write(this: &UnderlyingSink, val: &::js_sys::Function);
}
impl UnderlyingSink {
    #[doc = "Construct a new `UnderlyingSink`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_abort()` instead."]
    pub fn abort(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_abort(val);
        self
    }
    #[deprecated = "Use `set_close()` instead."]
    pub fn close(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_close(val);
        self
    }
    #[deprecated = "Use `set_start()` instead."]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_write()` instead."]
    pub fn write(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_write(val);
        self
    }
}
impl Default for UnderlyingSink {
    fn default() -> Self {
        Self::new()
    }
}
