#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "Pbkdf2Params")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Pbkdf2Params` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub type Pbkdf2Params;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Pbkdf2Params) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Pbkdf2Params, val: &str);
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &Pbkdf2Params) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &Pbkdf2Params, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_object(this: &Pbkdf2Params, val: &::js_sys::Object);
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash_str(this: &Pbkdf2Params, val: &str);
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, getter = "iterations")]
    pub fn get_iterations(this: &Pbkdf2Params) -> u32;
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "iterations")]
    pub fn set_iterations(this: &Pbkdf2Params, val: u32);
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, getter = "salt")]
    pub fn get_salt(this: &Pbkdf2Params) -> ::js_sys::Object;
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt(this: &Pbkdf2Params, val: &::js_sys::Object);
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt_buffer_source(this: &Pbkdf2Params, val: &::js_sys::Object);
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt_u8_slice(this: &Pbkdf2Params, val: &mut [u8]);
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    #[wasm_bindgen(method, setter = "salt")]
    pub fn set_salt_u8_array(this: &Pbkdf2Params, val: &::js_sys::Uint8Array);
}
impl Pbkdf2Params {
    #[doc = "Construct a new `Pbkdf2Params`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        iterations: u32,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash(hash);
        ret.set_iterations(iterations);
        ret.set_salt(salt);
        ret
    }
    #[doc = "Construct a new `Pbkdf2Params`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn new_with_object(
        name: &str,
        hash: &::js_sys::Object,
        iterations: u32,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash_object(hash);
        ret.set_iterations(iterations);
        ret.set_salt(salt);
        ret
    }
    #[doc = "Construct a new `Pbkdf2Params`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
    pub fn new_with_str(name: &str, hash: &str, iterations: u32, salt: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_name(name);
        ret.set_hash_str(hash);
        ret.set_iterations(iterations);
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
    #[deprecated = "Use `set_iterations()` instead."]
    pub fn iterations(&mut self, val: u32) -> &mut Self {
        self.set_iterations(val);
        self
    }
    #[deprecated = "Use `set_salt()` instead."]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_salt(val);
        self
    }
}
