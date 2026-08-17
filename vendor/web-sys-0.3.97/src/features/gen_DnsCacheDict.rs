#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DNSCacheDict")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub type DnsCacheDict;
    #[doc = "Get the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    #[wasm_bindgen(method, getter = "entries")]
    pub fn get_entries(this: &DnsCacheDict) -> Option<::js_sys::Array>;
    #[doc = "Change the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    #[wasm_bindgen(method, setter = "entries")]
    pub fn set_entries(this: &DnsCacheDict, val: &::wasm_bindgen::JsValue);
}
impl DnsCacheDict {
    #[doc = "Construct a new `DnsCacheDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entries()` instead."]
    pub fn entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_entries(val);
        self
    }
}
impl Default for DnsCacheDict {
    fn default() -> Self {
        Self::new()
    }
}
