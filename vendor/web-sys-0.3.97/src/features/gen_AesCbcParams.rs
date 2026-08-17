#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AesCbcParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesCbcParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub type AesCbcParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesCbcParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AesCbcParams, val: &str);
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, getter = "iv")]
    pub fn get_iv(this: &AesCbcParams) -> ::js_sys::Object;
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv(this: &AesCbcParams, val: &::js_sys::Object);
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv_buffer_source(this: &AesCbcParams, val: &::js_sys::Object);
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv_u8_slice(this: &AesCbcParams, val: &mut [u8]);
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv_u8_array(this: &AesCbcParams, val: &::js_sys::Uint8Array);
}
impl AesCbcParams {
    #[doc = "Construct a new `AesCbcParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub fn new(name: &str, iv: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_iv(iv);
        ret
    }
    #[doc = "Construct a new `AesCbcParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub fn new_with_u8_slice(name: &str, iv: &mut [u8]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_iv_u8_slice(iv);
        ret
    }
    #[doc = "Construct a new `AesCbcParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
    pub fn new_with_u8_array(name: &str, iv: &::js_sys::Uint8Array) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_iv_u8_array(iv);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_iv()` instead."]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_iv(val);
        self
    }
}
