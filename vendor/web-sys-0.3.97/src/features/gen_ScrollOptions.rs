#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ScrollOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
    pub type ScrollOptions;
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
    #[wasm_bindgen(method, getter = "behavior")]
    pub fn get_behavior(this: &ScrollOptions) -> Option<ScrollBehavior>;
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
    #[wasm_bindgen(method, setter = "behavior")]
    pub fn set_behavior(this: &ScrollOptions, val: ScrollBehavior);
}
impl ScrollOptions {
    #[doc = "Construct a new `ScrollOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
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
}
impl Default for ScrollOptions {
    fn default() -> Self {
        Self::new()
    }
}
