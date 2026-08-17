#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RsaOaepParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOaepParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub type RsaOaepParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &RsaOaepParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &RsaOaepParams, val: &str);
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &RsaOaepParams) -> Option<::js_sys::Object>;
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &RsaOaepParams, val: &::js_sys::Object);
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label_buffer_source(this: &RsaOaepParams, val: &::js_sys::Object);
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label_u8_slice(this: &RsaOaepParams, val: &mut [u8]);
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label_u8_array(this: &RsaOaepParams, val: &::js_sys::Uint8Array);
}
impl RsaOaepParams {
    #[doc = "Construct a new `RsaOaepParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_label(val);
        self
    }
}
