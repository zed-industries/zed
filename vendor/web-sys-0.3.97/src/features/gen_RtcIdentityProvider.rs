#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIdentityProvider")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProvider` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub type RtcIdentityProvider;
    #[doc = "Get the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    #[wasm_bindgen(method, getter = "generateAssertion")]
    pub fn get_generate_assertion(this: &RtcIdentityProvider) -> ::js_sys::Function;
    #[doc = "Change the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    #[wasm_bindgen(method, setter = "generateAssertion")]
    pub fn set_generate_assertion(this: &RtcIdentityProvider, val: &::js_sys::Function);
    #[doc = "Get the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    #[wasm_bindgen(method, getter = "validateAssertion")]
    pub fn get_validate_assertion(this: &RtcIdentityProvider) -> ::js_sys::Function;
    #[doc = "Change the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    #[wasm_bindgen(method, setter = "validateAssertion")]
    pub fn set_validate_assertion(this: &RtcIdentityProvider, val: &::js_sys::Function);
}
impl RtcIdentityProvider {
    #[doc = "Construct a new `RtcIdentityProvider`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn new(
        generate_assertion: &::js_sys::Function,
        validate_assertion: &::js_sys::Function,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_generate_assertion(generate_assertion);
        ret.set_validate_assertion(validate_assertion);
        ret
    }
    #[deprecated = "Use `set_generate_assertion()` instead."]
    pub fn generate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_generate_assertion(val);
        self
    }
    #[deprecated = "Use `set_validate_assertion()` instead."]
    pub fn validate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_validate_assertion(val);
        self
    }
}
