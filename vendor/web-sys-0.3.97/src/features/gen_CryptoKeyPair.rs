#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CryptoKeyPair")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CryptoKeyPair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKeyPair`*"]
    pub type CryptoKeyPair;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `privateKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    #[wasm_bindgen(method, getter = "privateKey")]
    pub fn get_private_key(this: &CryptoKeyPair) -> CryptoKey;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `privateKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    #[wasm_bindgen(method, setter = "privateKey")]
    pub fn set_private_key(this: &CryptoKeyPair, val: &CryptoKey);
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    #[wasm_bindgen(method, getter = "publicKey")]
    pub fn get_public_key(this: &CryptoKeyPair) -> CryptoKey;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    #[wasm_bindgen(method, setter = "publicKey")]
    pub fn set_public_key(this: &CryptoKeyPair, val: &CryptoKey);
}
impl CryptoKeyPair {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `CryptoKeyPair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn new(private_key: &CryptoKey, public_key: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_private_key(private_key);
        ret.set_public_key(public_key);
        ret
    }
    #[cfg(feature = "CryptoKey")]
    #[deprecated = "Use `set_private_key()` instead."]
    pub fn private_key(&mut self, val: &CryptoKey) -> &mut Self {
        self.set_private_key(val);
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[deprecated = "Use `set_public_key()` instead."]
    pub fn public_key(&mut self, val: &CryptoKey) -> &mut Self {
        self.set_public_key(val);
        self
    }
}
