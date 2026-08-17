#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "QueuingStrategyInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategyInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    pub type QueuingStrategyInit;
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    #[wasm_bindgen(method, getter = "highWaterMark")]
    pub fn get_high_water_mark(this: &QueuingStrategyInit) -> f64;
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    #[wasm_bindgen(method, setter = "highWaterMark")]
    pub fn set_high_water_mark(this: &QueuingStrategyInit, val: f64);
}
impl QueuingStrategyInit {
    #[doc = "Construct a new `QueuingStrategyInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    pub fn new(high_water_mark: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_high_water_mark(high_water_mark);
        ret
    }
    #[deprecated = "Use `set_high_water_mark()` instead."]
    pub fn high_water_mark(&mut self, val: f64) -> &mut Self {
        self.set_high_water_mark(val);
        self
    }
}
