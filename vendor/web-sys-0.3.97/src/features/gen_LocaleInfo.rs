#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "LocaleInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LocaleInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub type LocaleInfo;
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &LocaleInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &LocaleInfo, val: &str);
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    #[wasm_bindgen(method, getter = "locale")]
    pub fn get_locale(this: &LocaleInfo) -> Option<::alloc::string::String>;
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    #[wasm_bindgen(method, setter = "locale")]
    pub fn set_locale(this: &LocaleInfo, val: &str);
}
impl LocaleInfo {
    #[doc = "Construct a new `LocaleInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: &str) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_locale()` instead."]
    pub fn locale(&mut self, val: &str) -> &mut Self {
        self.set_locale(val);
        self
    }
}
impl Default for LocaleInfo {
    fn default() -> Self {
        Self::new()
    }
}
