#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CredentialRequestOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
    pub type CredentialRequestOptions;
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "publicKey")]
    pub fn get_public_key(
        this: &CredentialRequestOptions,
    ) -> Option<PublicKeyCredentialRequestOptions>;
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "publicKey")]
    pub fn set_public_key(this: &CredentialRequestOptions, val: &PublicKeyCredentialRequestOptions);
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "signal")]
    pub fn get_signal(this: &CredentialRequestOptions) -> Option<AbortSignal>;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "signal")]
    pub fn set_signal(this: &CredentialRequestOptions, val: &AbortSignal);
}
impl CredentialRequestOptions {
    #[doc = "Construct a new `CredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[deprecated = "Use `set_public_key()` instead."]
    pub fn public_key(&mut self, val: &PublicKeyCredentialRequestOptions) -> &mut Self {
        self.set_public_key(val);
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal(val);
        self
    }
}
impl Default for CredentialRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
