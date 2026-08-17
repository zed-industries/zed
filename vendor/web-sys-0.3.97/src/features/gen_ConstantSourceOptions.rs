#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConstantSourceOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstantSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    pub type ConstantSourceOptions;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &ConstantSourceOptions) -> Option<f32>;
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &ConstantSourceOptions, val: f32);
}
impl ConstantSourceOptions {
    #[doc = "Construct a new `ConstantSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: f32) -> &mut Self {
        self.set_offset(val);
        self
    }
}
impl Default for ConstantSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}
