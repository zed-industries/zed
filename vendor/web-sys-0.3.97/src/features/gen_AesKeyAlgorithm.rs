#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AesKeyAlgorithm")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    pub type AesKeyAlgorithm;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesKeyAlgorithm) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AesKeyAlgorithm, val: &str);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &AesKeyAlgorithm) -> u16;
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &AesKeyAlgorithm, val: u16);
}
impl AesKeyAlgorithm {
    #[doc = "Construct a new `AesKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyAlgorithm`*"]
    pub fn new(name: &str, length: u16) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_length(length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: u16) -> &mut Self {
        self.set_length(val);
        self
    }
}
