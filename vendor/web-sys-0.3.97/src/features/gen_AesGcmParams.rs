#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AesGcmParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesGcmParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub type AesGcmParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AesGcmParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AesGcmParams, val: &str);
    #[doc = "Get the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "additionalData")]
    pub fn get_additional_data(this: &AesGcmParams) -> Option<::js_sys::Object>;
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "additionalData")]
    pub fn set_additional_data(this: &AesGcmParams, val: &::js_sys::Object);
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "additionalData")]
    pub fn set_additional_data_buffer_source(this: &AesGcmParams, val: &::js_sys::Object);
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "additionalData")]
    pub fn set_additional_data_u8_slice(this: &AesGcmParams, val: &mut [u8]);
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "additionalData")]
    pub fn set_additional_data_u8_array(this: &AesGcmParams, val: &::js_sys::Uint8Array);
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "iv")]
    pub fn get_iv(this: &AesGcmParams) -> ::js_sys::Object;
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv(this: &AesGcmParams, val: &::js_sys::Object);
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv_buffer_source(this: &AesGcmParams, val: &::js_sys::Object);
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv_u8_slice(this: &AesGcmParams, val: &mut [u8]);
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "iv")]
    pub fn set_iv_u8_array(this: &AesGcmParams, val: &::js_sys::Uint8Array);
    #[doc = "Get the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, getter = "tagLength")]
    pub fn get_tag_length(this: &AesGcmParams) -> Option<u8>;
    #[doc = "Change the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    #[wasm_bindgen(method, setter = "tagLength")]
    pub fn set_tag_length(this: &AesGcmParams, val: u8);
}
impl AesGcmParams {
    #[doc = "Construct a new `AesGcmParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn new(name: &str, iv: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_iv(iv);
        ret
    }
    #[doc = "Construct a new `AesGcmParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn new_with_u8_slice(name: &str, iv: &mut [u8]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_iv_u8_slice(iv);
        ret
    }
    #[doc = "Construct a new `AesGcmParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
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
    #[deprecated = "Use `set_additional_data()` instead."]
    pub fn additional_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_additional_data(val);
        self
    }
    #[deprecated = "Use `set_iv()` instead."]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_iv(val);
        self
    }
    #[deprecated = "Use `set_tag_length()` instead."]
    pub fn tag_length(&mut self, val: u8) -> &mut Self {
        self.set_tag_length(val);
        self
    }
}
