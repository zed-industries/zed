#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ScrollToOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollToOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    pub type ScrollToOptions;
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollToOptions`*"]
    #[wasm_bindgen(method, getter = "behavior")]
    pub fn get_behavior(this: &ScrollToOptions) -> Option<ScrollBehavior>;
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollToOptions`*"]
    #[wasm_bindgen(method, setter = "behavior")]
    pub fn set_behavior(this: &ScrollToOptions, val: ScrollBehavior);
    #[doc = "Get the `left` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &ScrollToOptions) -> Option<f64>;
    #[doc = "Change the `left` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &ScrollToOptions, val: f64);
    #[doc = "Get the `top` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &ScrollToOptions) -> Option<f64>;
    #[doc = "Change the `top` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &ScrollToOptions, val: f64);
}
impl ScrollToOptions {
    #[doc = "Construct a new `ScrollToOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[deprecated = "Use `set_behavior()` instead."]
    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {
        self.set_behavior(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: f64) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: f64) -> &mut Self {
        self.set_top(val);
        self
    }
}
impl Default for ScrollToOptions {
    fn default() -> Self {
        Self::new()
    }
}
