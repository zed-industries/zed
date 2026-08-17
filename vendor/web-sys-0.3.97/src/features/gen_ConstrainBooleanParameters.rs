#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConstrainBooleanParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainBooleanParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    pub type ConstrainBooleanParameters;
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    #[wasm_bindgen(method, getter = "exact")]
    pub fn get_exact(this: &ConstrainBooleanParameters) -> Option<bool>;
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    #[wasm_bindgen(method, setter = "exact")]
    pub fn set_exact(this: &ConstrainBooleanParameters, val: bool);
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    #[wasm_bindgen(method, getter = "ideal")]
    pub fn get_ideal(this: &ConstrainBooleanParameters) -> Option<bool>;
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    #[wasm_bindgen(method, setter = "ideal")]
    pub fn set_ideal(this: &ConstrainBooleanParameters, val: bool);
}
impl ConstrainBooleanParameters {
    #[doc = "Construct a new `ConstrainBooleanParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_exact()` instead."]
    pub fn exact(&mut self, val: bool) -> &mut Self {
        self.set_exact(val);
        self
    }
    #[deprecated = "Use `set_ideal()` instead."]
    pub fn ideal(&mut self, val: bool) -> &mut Self {
        self.set_ideal(val);
        self
    }
}
impl Default for ConstrainBooleanParameters {
    fn default() -> Self {
        Self::new()
    }
}
