#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AesCtrParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesCtrParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub type AesCtrParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesCtrParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AesCtrParams, val: &str);
    #[doc = "Get the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, getter = "counter")]
    pub fn get_counter(this: &AesCtrParams) -> ::js_sys::Object;
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, setter = "counter")]
    pub fn set_counter(this: &AesCtrParams, val: &::js_sys::Object);
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, setter = "counter")]
    pub fn set_counter_buffer_source(this: &AesCtrParams, val: &::js_sys::Object);
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, setter = "counter")]
    pub fn set_counter_u8_slice(this: &AesCtrParams, val: &mut [u8]);
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, setter = "counter")]
    pub fn set_counter_u8_array(this: &AesCtrParams, val: &::js_sys::Uint8Array);
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &AesCtrParams) -> u8;
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &AesCtrParams, val: u8);
}
impl AesCtrParams {
    #[doc = "Construct a new `AesCtrParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn new(name: &str, counter: &::js_sys::Object, length: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_counter(counter);
        ret.set_length(length);
        ret
    }
    #[doc = "Construct a new `AesCtrParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn new_with_u8_slice(name: &str, counter: &mut [u8], length: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_counter_u8_slice(counter);
        ret.set_length(length);
        ret
    }
    #[doc = "Construct a new `AesCtrParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
    pub fn new_with_u8_array(name: &str, counter: &::js_sys::Uint8Array, length: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_counter_u8_array(counter);
        ret.set_length(length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_counter()` instead."]
    pub fn counter(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_counter(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: u8) -> &mut Self {
        self.set_length(val);
        self
    }
}
