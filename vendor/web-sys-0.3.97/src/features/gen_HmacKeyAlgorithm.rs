#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HmacKeyAlgorithm")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    pub type HmacKeyAlgorithm;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &HmacKeyAlgorithm) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &HmacKeyAlgorithm, val: &str);
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &HmacKeyAlgorithm) -> KeyAlgorithm;
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &HmacKeyAlgorithm, val: &KeyAlgorithm);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &HmacKeyAlgorithm) -> u32;
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &HmacKeyAlgorithm, val: u32);
}
impl HmacKeyAlgorithm {
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Construct a new `HmacKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn new(name: &str, hash: &KeyAlgorithm, length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash(hash);
        ret.set_length(length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[cfg(feature = "KeyAlgorithm")]
    #[deprecated = "Use `set_hash()` instead."]
    pub fn hash(&mut self, val: &KeyAlgorithm) -> &mut Self {
        self.set_hash(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length(val);
        self
    }
}
