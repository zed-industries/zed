#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "BasicCardRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardRequest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub type BasicCardRequest;
    #[doc = "Get the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    #[wasm_bindgen(method, getter = "supportedNetworks")]
    pub fn get_supported_networks(this: &BasicCardRequest) -> Option<::js_sys::Array>;
    #[doc = "Change the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    #[wasm_bindgen(method, setter = "supportedNetworks")]
    pub fn set_supported_networks(this: &BasicCardRequest, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    #[wasm_bindgen(method, getter = "supportedTypes")]
    pub fn get_supported_types(this: &BasicCardRequest) -> Option<::js_sys::Array>;
    #[doc = "Change the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    #[wasm_bindgen(method, setter = "supportedTypes")]
    pub fn set_supported_types(this: &BasicCardRequest, val: &::wasm_bindgen::JsValue);
}
impl BasicCardRequest {
    #[doc = "Construct a new `BasicCardRequest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_supported_networks()` instead."]
    pub fn supported_networks(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_supported_networks(val);
        self
    }
    #[deprecated = "Use `set_supported_types()` instead."]
    pub fn supported_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_supported_types(val);
        self
    }
}
impl Default for BasicCardRequest {
    fn default() -> Self {
        Self::new()
    }
}
