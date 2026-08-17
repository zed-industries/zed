#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PushSubscriptionJSON")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub type PushSubscriptionJson;
    #[doc = "Get the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    #[wasm_bindgen(method, getter = "endpoint")]
    pub fn get_endpoint(this: &PushSubscriptionJson) -> Option<::alloc::string::String>;
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    #[wasm_bindgen(method, setter = "endpoint")]
    pub fn set_endpoint(this: &PushSubscriptionJson, val: &str);
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    #[wasm_bindgen(method, getter = "keys")]
    pub fn get_keys(this: &PushSubscriptionJson) -> Option<PushSubscriptionKeys>;
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    #[wasm_bindgen(method, setter = "keys")]
    pub fn set_keys(this: &PushSubscriptionJson, val: &PushSubscriptionKeys);
}
impl PushSubscriptionJson {
    #[doc = "Construct a new `PushSubscriptionJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_endpoint()` instead."]
    pub fn endpoint(&mut self, val: &str) -> &mut Self {
        self.set_endpoint(val);
        self
    }
    #[cfg(feature = "PushSubscriptionKeys")]
    #[deprecated = "Use `set_keys()` instead."]
    pub fn keys(&mut self, val: &PushSubscriptionKeys) -> &mut Self {
        self.set_keys(val);
        self
    }
}
impl Default for PushSubscriptionJson {
    fn default() -> Self {
        Self::new()
    }
}
