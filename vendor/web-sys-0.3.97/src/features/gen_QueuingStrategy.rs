#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "QueuingStrategy")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub type QueuingStrategy;
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    #[wasm_bindgen(method, getter = "highWaterMark")]
    pub fn get_high_water_mark(this: &QueuingStrategy) -> Option<f64>;
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    #[wasm_bindgen(method, setter = "highWaterMark")]
    pub fn set_high_water_mark(this: &QueuingStrategy, val: f64);
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &QueuingStrategy) -> Option<::js_sys::Function>;
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &QueuingStrategy, val: &::js_sys::Function);
}
impl QueuingStrategy {
    #[doc = "Construct a new `QueuingStrategy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_high_water_mark()` instead."]
    pub fn high_water_mark(&mut self, val: f64) -> &mut Self {
        self.set_high_water_mark(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_size(val);
        self
    }
}
impl Default for QueuingStrategy {
    fn default() -> Self {
        Self::new()
    }
}
