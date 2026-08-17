#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PeriodicWaveConstraints")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWaveConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub type PeriodicWaveConstraints;
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    #[wasm_bindgen(method, getter = "disableNormalization")]
    pub fn get_disable_normalization(this: &PeriodicWaveConstraints) -> Option<bool>;
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    #[wasm_bindgen(method, setter = "disableNormalization")]
    pub fn set_disable_normalization(this: &PeriodicWaveConstraints, val: bool);
}
impl PeriodicWaveConstraints {
    #[doc = "Construct a new `PeriodicWaveConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_disable_normalization()` instead."]
    pub fn disable_normalization(&mut self, val: bool) -> &mut Self {
        self.set_disable_normalization(val);
        self
    }
}
impl Default for PeriodicWaveConstraints {
    fn default() -> Self {
        Self::new()
    }
}
