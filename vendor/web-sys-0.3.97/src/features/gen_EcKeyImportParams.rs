#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "EcKeyImportParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub type EcKeyImportParams;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &EcKeyImportParams) -> ::alloc::string::String;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &EcKeyImportParams, val: &str);
    #[doc = "Get the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    #[wasm_bindgen(method, getter = "namedCurve")]
    pub fn get_named_curve(this: &EcKeyImportParams) -> Option<::alloc::string::String>;
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    #[wasm_bindgen(method, setter = "namedCurve")]
    pub fn set_named_curve(this: &EcKeyImportParams, val: &str);
}
impl EcKeyImportParams {
    #[doc = "Construct a new `EcKeyImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
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
    #[deprecated = "Use `set_named_curve()` instead."]
    pub fn named_curve(&mut self, val: &str) -> &mut Self {
        self.set_named_curve(val);
        self
    }
}
