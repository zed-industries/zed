#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DNSLookupDict")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsLookupDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub type DnsLookupDict;
    #[doc = "Get the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &DnsLookupDict) -> Option<::js_sys::Array>;
    #[doc = "Change the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &DnsLookupDict, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    #[wasm_bindgen(method, getter = "answer")]
    pub fn get_answer(this: &DnsLookupDict) -> Option<bool>;
    #[doc = "Change the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    #[wasm_bindgen(method, setter = "answer")]
    pub fn set_answer(this: &DnsLookupDict, val: bool);
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &DnsLookupDict) -> Option<::alloc::string::String>;
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &DnsLookupDict, val: &str);
}
impl DnsLookupDict {
    #[doc = "Construct a new `DnsLookupDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_answer()` instead."]
    pub fn answer(&mut self, val: bool) -> &mut Self {
        self.set_answer(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: &str) -> &mut Self {
        self.set_error(val);
        self
    }
}
impl Default for DnsLookupDict {
    fn default() -> Self {
        Self::new()
    }
}
