#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DhKeyDeriveParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DhKeyDeriveParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    pub type DhKeyDeriveParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DhKeyDeriveParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DhKeyDeriveParams, val: &str);
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    #[wasm_bindgen(method, getter = "public")]
    pub fn get_public(this: &DhKeyDeriveParams) -> CryptoKey;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    #[wasm_bindgen(method, setter = "public")]
    pub fn set_public(this: &DhKeyDeriveParams, val: &CryptoKey);
}
impl DhKeyDeriveParams {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `DhKeyDeriveParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_public(public);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[deprecated = "Use `set_public()` instead."]
    pub fn public(&mut self, val: &CryptoKey) -> &mut Self {
        self.set_public(val);
        self
    }
}
