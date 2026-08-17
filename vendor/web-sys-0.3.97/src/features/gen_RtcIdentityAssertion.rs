#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIdentityAssertion")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertion` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub type RtcIdentityAssertion;
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    #[wasm_bindgen(method, getter = "idp")]
    pub fn get_idp(this: &RtcIdentityAssertion) -> Option<::alloc::string::String>;
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    #[wasm_bindgen(method, setter = "idp")]
    pub fn set_idp(this: &RtcIdentityAssertion, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &RtcIdentityAssertion) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &RtcIdentityAssertion, val: &str);
}
impl RtcIdentityAssertion {
    #[doc = "Construct a new `RtcIdentityAssertion`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_idp()` instead."]
    pub fn idp(&mut self, val: &str) -> &mut Self {
        self.set_idp(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for RtcIdentityAssertion {
    fn default() -> Self {
        Self::new()
    }
}
