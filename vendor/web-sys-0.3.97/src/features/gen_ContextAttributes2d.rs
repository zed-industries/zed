#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ContextAttributes2D")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ContextAttributes2d` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub type ContextAttributes2d;
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    #[wasm_bindgen(method, getter = "alpha")]
    pub fn get_alpha(this: &ContextAttributes2d) -> Option<bool>;
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    #[wasm_bindgen(method, setter = "alpha")]
    pub fn set_alpha(this: &ContextAttributes2d, val: bool);
    #[doc = "Get the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    #[wasm_bindgen(method, getter = "willReadFrequently")]
    pub fn get_will_read_frequently(this: &ContextAttributes2d) -> Option<bool>;
    #[doc = "Change the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    #[wasm_bindgen(method, setter = "willReadFrequently")]
    pub fn set_will_read_frequently(this: &ContextAttributes2d, val: bool);
}
impl ContextAttributes2d {
    #[doc = "Construct a new `ContextAttributes2d`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_alpha()` instead."]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        self.set_alpha(val);
        self
    }
    #[deprecated = "Use `set_will_read_frequently()` instead."]
    pub fn will_read_frequently(&mut self, val: bool) -> &mut Self {
        self.set_will_read_frequently(val);
        self
    }
}
impl Default for ContextAttributes2d {
    fn default() -> Self {
        Self::new()
    }
}
