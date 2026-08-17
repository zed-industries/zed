#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PushSubscriptionKeys")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionKeys` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub type PushSubscriptionKeys;
    #[doc = "Get the `auth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    #[wasm_bindgen(method, getter = "auth")]
    pub fn get_auth(this: &PushSubscriptionKeys) -> Option<::alloc::string::String>;
    #[doc = "Change the `auth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    #[wasm_bindgen(method, setter = "auth")]
    pub fn set_auth(this: &PushSubscriptionKeys, val: &str);
    #[doc = "Get the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    #[wasm_bindgen(method, getter = "p256dh")]
    pub fn get_p256dh(this: &PushSubscriptionKeys) -> Option<::alloc::string::String>;
    #[doc = "Change the `p256dh` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    #[wasm_bindgen(method, setter = "p256dh")]
    pub fn set_p256dh(this: &PushSubscriptionKeys, val: &str);
}
impl PushSubscriptionKeys {
    #[doc = "Construct a new `PushSubscriptionKeys`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionKeys`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auth()` instead."]
    pub fn auth(&mut self, val: &str) -> &mut Self {
        self.set_auth(val);
        self
    }
    #[deprecated = "Use `set_p256dh()` instead."]
    pub fn p256dh(&mut self, val: &str) -> &mut Self {
        self.set_p256dh(val);
        self
    }
}
impl Default for PushSubscriptionKeys {
    fn default() -> Self {
        Self::new()
    }
}
