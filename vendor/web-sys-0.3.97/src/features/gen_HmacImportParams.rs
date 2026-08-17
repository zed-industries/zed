#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HmacImportParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub type HmacImportParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &HmacImportParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &HmacImportParams, val: &str);
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &HmacImportParams) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &HmacImportParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_object(this: &HmacImportParams, val: &::js_sys::Object);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_str(this: &HmacImportParams, val: &str);
}
impl HmacImportParams {
    #[doc = "Construct a new `HmacImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash(hash);
        ret
    }
    #[doc = "Construct a new `HmacImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub fn new_with_object(name: &str, hash: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash_object(hash);
        ret
    }
    #[doc = "Construct a new `HmacImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacImportParams`*"]
    pub fn new_with_str(name: &str, hash: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash_str(hash);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_hash()` instead."]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_hash(val);
        self
    }
}
