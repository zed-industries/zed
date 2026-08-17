#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PerformanceObserverInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    pub type PerformanceObserverInit;
    #[doc = "Get the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    #[wasm_bindgen(method, getter = "buffered")]
    pub fn get_buffered(this: &PerformanceObserverInit) -> Option<bool>;
    #[doc = "Change the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    #[wasm_bindgen(method, setter = "buffered")]
    pub fn set_buffered(this: &PerformanceObserverInit, val: bool);
    #[doc = "Get the `entryTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    #[wasm_bindgen(method, getter = "entryTypes")]
    pub fn get_entry_types(this: &PerformanceObserverInit) -> ::js_sys::Array;
    #[doc = "Change the `entryTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    #[wasm_bindgen(method, setter = "entryTypes")]
    pub fn set_entry_types(this: &PerformanceObserverInit, val: &::wasm_bindgen::JsValue);
}
impl PerformanceObserverInit {
    #[doc = "Construct a new `PerformanceObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
    pub fn new(entry_types: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_entry_types(entry_types);
        ret
    }
    #[deprecated = "Use `set_buffered()` instead."]
    pub fn buffered(&mut self, val: bool) -> &mut Self {
        self.set_buffered(val);
        self
    }
    #[deprecated = "Use `set_entry_types()` instead."]
    pub fn entry_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_entry_types(val);
        self
    }
}
