#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ScrollIntoViewOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollIntoViewOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
    pub type ScrollIntoViewOptions;
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
    #[wasm_bindgen(method, getter = "behavior")]
    pub fn get_behavior(this: &ScrollIntoViewOptions) -> Option<ScrollBehavior>;
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
    #[wasm_bindgen(method, setter = "behavior")]
    pub fn set_behavior(this: &ScrollIntoViewOptions, val: ScrollBehavior);
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Get the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    #[wasm_bindgen(method, getter = "block")]
    pub fn get_block(this: &ScrollIntoViewOptions) -> Option<ScrollLogicalPosition>;
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    #[wasm_bindgen(method, setter = "block")]
    pub fn set_block(this: &ScrollIntoViewOptions, val: ScrollLogicalPosition);
    #[cfg(feature = "ScrollIntoViewContainer")]
    #[doc = "Get the `container` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewContainer`, `ScrollIntoViewOptions`*"]
    #[wasm_bindgen(method, getter = "container")]
    pub fn get_container(this: &ScrollIntoViewOptions) -> Option<ScrollIntoViewContainer>;
    #[cfg(feature = "ScrollIntoViewContainer")]
    #[doc = "Change the `container` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewContainer`, `ScrollIntoViewOptions`*"]
    #[wasm_bindgen(method, setter = "container")]
    pub fn set_container(this: &ScrollIntoViewOptions, val: ScrollIntoViewContainer);
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Get the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    #[wasm_bindgen(method, getter = "inline")]
    pub fn get_inline(this: &ScrollIntoViewOptions) -> Option<ScrollLogicalPosition>;
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    #[wasm_bindgen(method, setter = "inline")]
    pub fn set_inline(this: &ScrollIntoViewOptions, val: ScrollLogicalPosition);
}
impl ScrollIntoViewOptions {
    #[doc = "Construct a new `ScrollIntoViewOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
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
    #[cfg(feature = "ScrollLogicalPosition")]
    #[deprecated = "Use `set_block()` instead."]
    pub fn block(&mut self, val: ScrollLogicalPosition) -> &mut Self {
        self.set_block(val);
        self
    }
    #[cfg(feature = "ScrollIntoViewContainer")]
    #[deprecated = "Use `set_container()` instead."]
    pub fn container(&mut self, val: ScrollIntoViewContainer) -> &mut Self {
        self.set_container(val);
        self
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[deprecated = "Use `set_inline()` instead."]
    pub fn inline(&mut self, val: ScrollLogicalPosition) -> &mut Self {
        self.set_inline(val);
        self
    }
}
impl Default for ScrollIntoViewOptions {
    fn default() -> Self {
        Self::new()
    }
}
