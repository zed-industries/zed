#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "HkdfParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HkdfParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub type HkdfParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &HkdfParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &HkdfParams, val: &str);
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &HkdfParams) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &HkdfParams, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_object(this: &HkdfParams, val: &::js_sys::Object);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_str(this: &HkdfParams, val: &str);
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &HkdfParams) -> ::js_sys::Object;
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info(this: &HkdfParams, val: &::js_sys::Object);
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info_buffer_source(this: &HkdfParams, val: &::js_sys::Object);
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info_u8_slice(this: &HkdfParams, val: &mut [u8]);
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info_u8_array(this: &HkdfParams, val: &::js_sys::Uint8Array);
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, getter = "salt")]
    pub fn get_salt(this: &HkdfParams) -> ::js_sys::Object;
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt(this: &HkdfParams, val: &::js_sys::Object);
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt_buffer_source(this: &HkdfParams, val: &::js_sys::Object);
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt_u8_slice(this: &HkdfParams, val: &mut [u8]);
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt_u8_array(this: &HkdfParams, val: &::js_sys::Uint8Array);
}
impl HkdfParams {
    #[doc = "Construct a new `HkdfParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        info: &::js_sys::Object,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash(hash);
        ret.set_info(info);
        ret.set_salt(salt);
        ret
    }
    #[doc = "Construct a new `HkdfParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn new_with_object(
        name: &str,
        hash: &::js_sys::Object,
        info: &::js_sys::Object,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash_object(hash);
        ret.set_info(info);
        ret.set_salt(salt);
        ret
    }
    #[doc = "Construct a new `HkdfParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn new_with_str(
        name: &str,
        hash: &str,
        info: &::js_sys::Object,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash_str(hash);
        ret.set_info(info);
        ret.set_salt(salt);
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
    #[deprecated = "Use `set_info()` instead."]
    pub fn info(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_info(val);
        self
    }
    #[deprecated = "Use `set_salt()` instead."]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_salt(val);
        self
    }
}
