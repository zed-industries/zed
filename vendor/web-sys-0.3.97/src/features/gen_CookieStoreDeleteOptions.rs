#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CookieStoreDeleteOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieStoreDeleteOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    pub type CookieStoreDeleteOptions;
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &CookieStoreDeleteOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &CookieStoreDeleteOptions, val: Option<&str>);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &CookieStoreDeleteOptions) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &CookieStoreDeleteOptions, val: &str);
    #[doc = "Get the `partitioned` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, getter = "partitioned")]
    pub fn get_partitioned(this: &CookieStoreDeleteOptions) -> Option<bool>;
    #[doc = "Change the `partitioned` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, setter = "partitioned")]
    pub fn set_partitioned(this: &CookieStoreDeleteOptions, val: bool);
    #[doc = "Get the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &CookieStoreDeleteOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &CookieStoreDeleteOptions, val: &str);
}
impl CookieStoreDeleteOptions {
    #[doc = "Construct a new `CookieStoreDeleteOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStoreDeleteOptions`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: Option<&str>) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_partitioned()` instead."]
    pub fn partitioned(&mut self, val: bool) -> &mut Self {
        self.set_partitioned(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: &str) -> &mut Self {
        self.set_path(val);
        self
    }
}
