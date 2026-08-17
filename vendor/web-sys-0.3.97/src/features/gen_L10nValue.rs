#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "L10nValue")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    pub type L10nValue;
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    #[wasm_bindgen(method, getter = "attributes")]
    pub fn get_attributes(this: &L10nValue) -> Option<::js_sys::Array>;
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    #[wasm_bindgen(method, setter = "attributes")]
    pub fn set_attributes(this: &L10nValue, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &L10nValue) -> Option<::alloc::string::String>;
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &L10nValue, val: Option<&str>);
}
impl L10nValue {
    #[doc = "Construct a new `L10nValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_attributes()` instead."]
    pub fn attributes(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        self.set_attributes(val.unwrap_or(&::wasm_bindgen::JsValue::NULL));
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: Option<&str>) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for L10nValue {
    fn default() -> Self {
        Self::new()
    }
}
