#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIdentityProviderDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub type RtcIdentityProviderDetails;
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &RtcIdentityProviderDetails) -> ::alloc::string::String;
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &RtcIdentityProviderDetails, val: &str);
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    #[wasm_bindgen(method, getter = "protocol")]
    pub fn get_protocol(this: &RtcIdentityProviderDetails) -> Option<::alloc::string::String>;
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    #[wasm_bindgen(method, setter = "protocol")]
    pub fn set_protocol(this: &RtcIdentityProviderDetails, val: &str);
}
impl RtcIdentityProviderDetails {
    #[doc = "Construct a new `RtcIdentityProviderDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub fn new(domain: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_domain(domain);
        ret
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol(val);
        self
    }
}
