#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AesDerivedKeyParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesDerivedKeyParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    pub type AesDerivedKeyParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesDerivedKeyParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AesDerivedKeyParams, val: &str);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &AesDerivedKeyParams) -> u32;
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &AesDerivedKeyParams, val: u32);
}
impl AesDerivedKeyParams {
    #[doc = "Construct a new `AesDerivedKeyParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesDerivedKeyParams`*"]
    pub fn new(name: &str, length: u32) -> Self {
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
    pub fn length(&mut self, val: u32) -> &mut Self {
        self.set_length(val);
        self
    }
}
