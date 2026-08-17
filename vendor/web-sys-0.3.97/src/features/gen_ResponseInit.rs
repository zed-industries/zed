#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ResponseInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResponseInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub type ResponseInit;
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, getter = "headers")]
    pub fn get_headers(this: &ResponseInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, setter = "headers")]
    pub fn set_headers(this: &ResponseInit, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "Headers")]
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`, `ResponseInit`*"]
    #[wasm_bindgen(method, setter = "headers")]
    pub fn set_headers_headers(this: &ResponseInit, val: &Headers);
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, setter = "headers")]
    pub fn set_headers_str_sequence_sequence(this: &ResponseInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, setter = "headers")]
    pub fn set_headers_record_from_str_to_str(this: &ResponseInit, val: &::js_sys::Object);
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &ResponseInit) -> Option<u16>;
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &ResponseInit, val: u16);
    #[doc = "Get the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, getter = "statusText")]
    pub fn get_status_text(this: &ResponseInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, setter = "statusText")]
    pub fn set_status_text(this: &ResponseInit, val: &str);
}
impl ResponseInit {
    #[doc = "Construct a new `ResponseInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_headers()` instead."]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_headers(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: u16) -> &mut Self {
        self.set_status(val);
        self
    }
    #[deprecated = "Use `set_status_text()` instead."]
    pub fn status_text(&mut self, val: &str) -> &mut Self {
        self.set_status_text(val);
        self
    }
}
impl Default for ResponseInit {
    fn default() -> Self {
        Self::new()
    }
}
