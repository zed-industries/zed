#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PromiseRejectionEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseRejectionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub type PromiseRejectionEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &PromiseRejectionEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &PromiseRejectionEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &PromiseRejectionEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &PromiseRejectionEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &PromiseRejectionEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &PromiseRejectionEventInit, val: bool);
    #[doc = "Get the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, getter = "promise")]
    pub fn get_promise(this: &PromiseRejectionEventInit) -> ::js_sys::Promise;
    #[doc = "Change the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, setter = "promise")]
    pub fn set_promise(this: &PromiseRejectionEventInit, val: &::js_sys::Promise);
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &PromiseRejectionEventInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &PromiseRejectionEventInit, val: &::wasm_bindgen::JsValue);
}
impl PromiseRejectionEventInit {
    #[doc = "Construct a new `PromiseRejectionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
    pub fn new(promise: &::js_sys::Promise) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_promise(promise);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_promise()` instead."]
    pub fn promise(&mut self, val: &::js_sys::Promise) -> &mut Self {
        self.set_promise(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_reason(val);
        self
    }
}
