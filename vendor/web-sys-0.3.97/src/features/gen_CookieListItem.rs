#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CookieListItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieListItem` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieListItem`*"]
    pub type CookieListItem;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieListItem`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &CookieListItem) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieListItem`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &CookieListItem, val: &str);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieListItem`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &CookieListItem) -> Option<::alloc::string::String>;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieListItem`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &CookieListItem, val: &str);
}
impl CookieListItem {
    #[doc = "Construct a new `CookieListItem`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieListItem`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &str) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for CookieListItem {
    fn default() -> Self {
        Self::new()
    }
}
