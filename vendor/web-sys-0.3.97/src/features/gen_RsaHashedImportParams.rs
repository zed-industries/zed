#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "RsaHashedImportParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaHashedImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub type RsaHashedImportParams;
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &RsaHashedImportParams) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &RsaHashedImportParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_object(this: &RsaHashedImportParams, val: &::js_sys::Object);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_str(this: &RsaHashedImportParams, val: &str);
}
impl RsaHashedImportParams {
    #[doc = "Construct a new `RsaHashedImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn new(hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_hash(hash);
        ret
    }
    #[doc = "Construct a new `RsaHashedImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn new_with_object(hash: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_hash_object(hash);
        ret
    }
    #[doc = "Construct a new `RsaHashedImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn new_with_str(hash: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_hash_str(hash);
        ret
    }
    #[deprecated = "Use `set_hash()` instead."]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash(val);
        self
    }
}
