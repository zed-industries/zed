#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConstrainDOMStringParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainDomStringParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub type ConstrainDomStringParameters;
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, getter = "exact")]
    pub fn get_exact(this: &ConstrainDomStringParameters) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, setter = "exact")]
    pub fn set_exact(this: &ConstrainDomStringParameters, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, setter = "exact")]
    pub fn set_exact_str(this: &ConstrainDomStringParameters, val: &str);
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, setter = "exact")]
    pub fn set_exact_str_sequence(
        this: &ConstrainDomStringParameters,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, getter = "ideal")]
    pub fn get_ideal(this: &ConstrainDomStringParameters) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, setter = "ideal")]
    pub fn set_ideal(this: &ConstrainDomStringParameters, val: &::wasm_bindgen::JsValue);
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, setter = "ideal")]
    pub fn set_ideal_str(this: &ConstrainDomStringParameters, val: &str);
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    #[wasm_bindgen(method, setter = "ideal")]
    pub fn set_ideal_str_sequence(
        this: &ConstrainDomStringParameters,
        val: &::wasm_bindgen::JsValue,
    );
}
impl ConstrainDomStringParameters {
    #[doc = "Construct a new `ConstrainDomStringParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_exact()` instead."]
    pub fn exact(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_exact(val);
        self
    }
    #[deprecated = "Use `set_ideal()` instead."]
    pub fn ideal(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ideal(val);
        self
    }
}
impl Default for ConstrainDomStringParameters {
    fn default() -> Self {
        Self::new()
    }
}
