#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CacheBatchOperation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheBatchOperation` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub type CacheBatchOperation;
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Get the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
    #[wasm_bindgen(method, getter = "options")]
    pub fn get_options(this: &CacheBatchOperation) -> Option<CacheQueryOptions>;
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Change the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
    #[wasm_bindgen(method, setter = "options")]
    pub fn set_options(this: &CacheBatchOperation, val: &CacheQueryOptions);
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    #[wasm_bindgen(method, getter = "request")]
    pub fn get_request(this: &CacheBatchOperation) -> Option<Request>;
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    #[wasm_bindgen(method, setter = "request")]
    pub fn set_request(this: &CacheBatchOperation, val: &Request);
    #[cfg(feature = "Response")]
    #[doc = "Get the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    #[wasm_bindgen(method, getter = "response")]
    pub fn get_response(this: &CacheBatchOperation) -> Option<Response>;
    #[cfg(feature = "Response")]
    #[doc = "Change the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    #[wasm_bindgen(method, setter = "response")]
    pub fn set_response(this: &CacheBatchOperation, val: &Response);
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CacheBatchOperation) -> Option<::alloc::string::String>;
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &CacheBatchOperation, val: &str);
}
impl CacheBatchOperation {
    #[doc = "Construct a new `CacheBatchOperation`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CacheQueryOptions")]
    #[deprecated = "Use `set_options()` instead."]
    pub fn options(&mut self, val: &CacheQueryOptions) -> &mut Self {
        self.set_options(val);
        self
    }
    #[cfg(feature = "Request")]
    #[deprecated = "Use `set_request()` instead."]
    pub fn request(&mut self, val: &Request) -> &mut Self {
        self.set_request(val);
        self
    }
    #[cfg(feature = "Response")]
    #[deprecated = "Use `set_response()` instead."]
    pub fn response(&mut self, val: &Response) -> &mut Self {
        self.set_response(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for CacheBatchOperation {
    fn default() -> Self {
        Self::new()
    }
}
