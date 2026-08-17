#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIdentityAssertionResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertionResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    pub type RtcIdentityAssertionResult;
    #[doc = "Get the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    #[wasm_bindgen(method, getter = "assertion")]
    pub fn get_assertion(this: &RtcIdentityAssertionResult) -> ::alloc::string::String;
    #[doc = "Change the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    #[wasm_bindgen(method, setter = "assertion")]
    pub fn set_assertion(this: &RtcIdentityAssertionResult, val: &str);
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    #[wasm_bindgen(method, getter = "idp")]
    pub fn get_idp(this: &RtcIdentityAssertionResult) -> RtcIdentityProviderDetails;
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    #[wasm_bindgen(method, setter = "idp")]
    pub fn set_idp(this: &RtcIdentityAssertionResult, val: &RtcIdentityProviderDetails);
}
impl RtcIdentityAssertionResult {
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Construct a new `RtcIdentityAssertionResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    pub fn new(assertion: &str, idp: &RtcIdentityProviderDetails) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_assertion(assertion);
        ret.set_idp(idp);
        ret
    }
    #[deprecated = "Use `set_assertion()` instead."]
    pub fn assertion(&mut self, val: &str) -> &mut Self {
        self.set_assertion(val);
        self
    }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[deprecated = "Use `set_idp()` instead."]
    pub fn idp(&mut self, val: &RtcIdentityProviderDetails) -> &mut Self {
        self.set_idp(val);
        self
    }
}
