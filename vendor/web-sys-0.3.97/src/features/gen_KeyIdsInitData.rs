#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "KeyIdsInitData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyIdsInitData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    pub type KeyIdsInitData;
    #[doc = "Get the `kids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    #[wasm_bindgen(method, getter = "kids")]
    pub fn get_kids(this: &KeyIdsInitData) -> ::js_sys::Array;
    #[doc = "Change the `kids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    #[wasm_bindgen(method, setter = "kids")]
    pub fn set_kids(this: &KeyIdsInitData, val: &::wasm_bindgen::JsValue);
}
impl KeyIdsInitData {
    #[doc = "Construct a new `KeyIdsInitData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
    pub fn new(kids: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_kids(kids);
        ret
    }
    #[deprecated = "Use `set_kids()` instead."]
    pub fn kids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_kids(val);
        self
    }
}
