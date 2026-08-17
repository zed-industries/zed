#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RTCIdentityValidationResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityValidationResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub type RtcIdentityValidationResult;
    #[doc = "Get the `contents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    #[wasm_bindgen(method, getter = "contents")]
    pub fn get_contents(this: &RtcIdentityValidationResult) -> ::alloc::string::String;
    #[doc = "Change the `contents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    #[wasm_bindgen(method, setter = "contents")]
    pub fn set_contents(this: &RtcIdentityValidationResult, val: &str);
    #[doc = "Get the `identity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    #[wasm_bindgen(method, getter = "identity")]
    pub fn get_identity(this: &RtcIdentityValidationResult) -> ::alloc::string::String;
    #[doc = "Change the `identity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    #[wasm_bindgen(method, setter = "identity")]
    pub fn set_identity(this: &RtcIdentityValidationResult, val: &str);
}
impl RtcIdentityValidationResult {
    #[doc = "Construct a new `RtcIdentityValidationResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
    pub fn new(contents: &str, identity: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_contents(contents);
        ret.set_identity(identity);
        ret
    }
    #[deprecated = "Use `set_contents()` instead."]
    pub fn contents(&mut self, val: &str) -> &mut Self {
        self.set_contents(val);
        self
    }
    #[deprecated = "Use `set_identity()` instead."]
    pub fn identity(&mut self, val: &str) -> &mut Self {
        self.set_identity(val);
        self
    }
}
