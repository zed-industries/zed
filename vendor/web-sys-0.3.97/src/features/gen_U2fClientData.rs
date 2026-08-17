#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "U2FClientData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `U2fClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub type U2fClientData;
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &U2fClientData) -> Option<::alloc::string::String>;
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge(this: &U2fClientData, val: &str);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &U2fClientData) -> Option<::alloc::string::String>;
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &U2fClientData, val: &str);
    #[doc = "Get the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    #[wasm_bindgen(method, getter = "typ")]
    pub fn get_typ(this: &U2fClientData) -> Option<::alloc::string::String>;
    #[doc = "Change the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    #[wasm_bindgen(method, setter = "typ")]
    pub fn set_typ(this: &U2fClientData, val: &str);
}
impl U2fClientData {
    #[doc = "Construct a new `U2fClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_challenge()` instead."]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[deprecated = "Use `set_typ()` instead."]
    pub fn typ(&mut self, val: &str) -> &mut Self {
        self.set_typ(val);
        self
    }
}
impl Default for U2fClientData {
    fn default() -> Self {
        Self::new()
    }
}
