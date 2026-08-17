#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HttpConnectionElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnectionElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub type HttpConnectionElement;
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &HttpConnectionElement) -> Option<::js_sys::Array>;
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "halfOpens")]
    pub fn get_half_opens(this: &HttpConnectionElement) -> Option<::js_sys::Array>;
    #[doc = "Change the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "halfOpens")]
    pub fn set_half_opens(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "host")]
    pub fn get_host(this: &HttpConnectionElement) -> Option<::alloc::string::String>;
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "host")]
    pub fn set_host(this: &HttpConnectionElement, val: &str);
    #[doc = "Get the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "idle")]
    pub fn get_idle(this: &HttpConnectionElement) -> Option<::js_sys::Array>;
    #[doc = "Change the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "idle")]
    pub fn set_idle(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &HttpConnectionElement) -> Option<u32>;
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "port")]
    pub fn set_port(this: &HttpConnectionElement, val: u32);
    #[doc = "Get the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "spdy")]
    pub fn get_spdy(this: &HttpConnectionElement) -> Option<bool>;
    #[doc = "Change the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "spdy")]
    pub fn set_spdy(this: &HttpConnectionElement, val: bool);
    #[doc = "Get the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, getter = "ssl")]
    pub fn get_ssl(this: &HttpConnectionElement) -> Option<bool>;
    #[doc = "Change the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    #[wasm_bindgen(method, setter = "ssl")]
    pub fn set_ssl(this: &HttpConnectionElement, val: bool);
}
impl HttpConnectionElement {
    #[doc = "Construct a new `HttpConnectionElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_half_opens()` instead."]
    pub fn half_opens(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_half_opens(val);
        self
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_idle()` instead."]
    pub fn idle(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_idle(val);
        self
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.set_port(val);
        self
    }
    #[deprecated = "Use `set_spdy()` instead."]
    pub fn spdy(&mut self, val: bool) -> &mut Self {
        self.set_spdy(val);
        self
    }
    #[deprecated = "Use `set_ssl()` instead."]
    pub fn ssl(&mut self, val: bool) -> &mut Self {
        self.set_ssl(val);
        self
    }
}
impl Default for HttpConnectionElement {
    fn default() -> Self {
        Self::new()
    }
}
