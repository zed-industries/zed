#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ExtendableCookieChangeEventInit"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableCookieChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    pub type ExtendableCookieChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ExtendableCookieChangeEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &ExtendableCookieChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ExtendableCookieChangeEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &ExtendableCookieChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ExtendableCookieChangeEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &ExtendableCookieChangeEventInit, val: bool);
    #[doc = "Get the `changed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "changed")]
    pub fn get_changed(this: &ExtendableCookieChangeEventInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `changed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "changed")]
    pub fn set_changed(this: &ExtendableCookieChangeEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `deleted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "deleted")]
    pub fn get_deleted(this: &ExtendableCookieChangeEventInit) -> Option<::js_sys::Array>;
    #[doc = "Change the `deleted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "deleted")]
    pub fn set_deleted(this: &ExtendableCookieChangeEventInit, val: &::wasm_bindgen::JsValue);
}
impl ExtendableCookieChangeEventInit {
    #[doc = "Construct a new `ExtendableCookieChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableCookieChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
    #[deprecated = "Use `set_changed()` instead."]
    pub fn changed(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_changed(val);
        self
    }
    #[deprecated = "Use `set_deleted()` instead."]
    pub fn deleted(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_deleted(val);
        self
    }
}
impl Default for ExtendableCookieChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
