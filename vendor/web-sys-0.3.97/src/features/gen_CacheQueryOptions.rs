#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CacheQueryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheQueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub type CacheQueryOptions;
    #[doc = "Get the `cacheName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, getter = "cacheName")]
    pub fn get_cache_name(this: &CacheQueryOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `cacheName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, setter = "cacheName")]
    pub fn set_cache_name(this: &CacheQueryOptions, val: &str);
    #[doc = "Get the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, getter = "ignoreMethod")]
    pub fn get_ignore_method(this: &CacheQueryOptions) -> Option<bool>;
    #[doc = "Change the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, setter = "ignoreMethod")]
    pub fn set_ignore_method(this: &CacheQueryOptions, val: bool);
    #[doc = "Get the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, getter = "ignoreSearch")]
    pub fn get_ignore_search(this: &CacheQueryOptions) -> Option<bool>;
    #[doc = "Change the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, setter = "ignoreSearch")]
    pub fn set_ignore_search(this: &CacheQueryOptions, val: bool);
    #[doc = "Get the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, getter = "ignoreVary")]
    pub fn get_ignore_vary(this: &CacheQueryOptions) -> Option<bool>;
    #[doc = "Change the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    #[wasm_bindgen(method, setter = "ignoreVary")]
    pub fn set_ignore_vary(this: &CacheQueryOptions, val: bool);
}
impl CacheQueryOptions {
    #[doc = "Construct a new `CacheQueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cache_name()` instead."]
    pub fn cache_name(&mut self, val: &str) -> &mut Self {
        self.set_cache_name(val);
        self
    }
    #[deprecated = "Use `set_ignore_method()` instead."]
    pub fn ignore_method(&mut self, val: bool) -> &mut Self {
        self.set_ignore_method(val);
        self
    }
    #[deprecated = "Use `set_ignore_search()` instead."]
    pub fn ignore_search(&mut self, val: bool) -> &mut Self {
        self.set_ignore_search(val);
        self
    }
    #[deprecated = "Use `set_ignore_vary()` instead."]
    pub fn ignore_vary(&mut self, val: bool) -> &mut Self {
        self.set_ignore_vary(val);
        self
    }
}
impl Default for CacheQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
