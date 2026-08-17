#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "PositionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PositionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub type PositionOptions;
    #[doc = "Get the `enableHighAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    #[wasm_bindgen(method, getter = "enableHighAccuracy")]
    pub fn get_enable_high_accuracy(this: &PositionOptions) -> Option<bool>;
    #[doc = "Change the `enableHighAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    #[wasm_bindgen(method, setter = "enableHighAccuracy")]
    pub fn set_enable_high_accuracy(this: &PositionOptions, val: bool);
    #[doc = "Get the `maximumAge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    #[wasm_bindgen(method, getter = "maximumAge")]
    pub fn get_maximum_age(this: &PositionOptions) -> Option<u32>;
    #[doc = "Change the `maximumAge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    #[wasm_bindgen(method, setter = "maximumAge")]
    pub fn set_maximum_age(this: &PositionOptions, val: u32);
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &PositionOptions) -> Option<u32>;
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &PositionOptions, val: u32);
}
impl PositionOptions {
    #[doc = "Construct a new `PositionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_enable_high_accuracy()` instead."]
    pub fn enable_high_accuracy(&mut self, val: bool) -> &mut Self {
        self.set_enable_high_accuracy(val);
        self
    }
    #[deprecated = "Use `set_maximum_age()` instead."]
    pub fn maximum_age(&mut self, val: u32) -> &mut Self {
        self.set_maximum_age(val);
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout(val);
        self
    }
}
impl Default for PositionOptions {
    fn default() -> Self {
        Self::new()
    }
}
