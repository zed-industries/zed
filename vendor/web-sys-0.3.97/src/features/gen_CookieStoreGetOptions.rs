#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CookieStoreGetOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieStoreGetOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"]
    pub type CookieStoreGetOptions;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &CookieStoreGetOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &CookieStoreGetOptions, val: &str);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &CookieStoreGetOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"]
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &CookieStoreGetOptions, val: &str);
}
impl CookieStoreGetOptions {
    #[doc = "Construct a new `CookieStoreGetOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreGetOptions`*"]
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
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for CookieStoreGetOptions {
    fn default() -> Self {
        Self::new()
    }
}
