#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DOMRectInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub type DomRectInit;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &DomRectInit) -> Option<f64>;
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &DomRectInit, val: f64);
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &DomRectInit) -> Option<f64>;
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &DomRectInit, val: f64);
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, getter = "x")]
    pub fn get_x(this: &DomRectInit) -> Option<f64>;
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, setter = "x")]
    pub fn set_x(this: &DomRectInit, val: f64);
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, getter = "y")]
    pub fn get_y(this: &DomRectInit) -> Option<f64>;
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    #[wasm_bindgen(method, setter = "y")]
    pub fn set_y(this: &DomRectInit, val: f64);
}
impl DomRectInit {
    #[doc = "Construct a new `DomRectInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: f64) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: f64) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_x()` instead."]
    pub fn x(&mut self, val: f64) -> &mut Self {
        self.set_x(val);
        self
    }
    #[deprecated = "Use `set_y()` instead."]
    pub fn y(&mut self, val: f64) -> &mut Self {
        self.set_y(val);
        self
    }
}
impl Default for DomRectInit {
    fn default() -> Self {
        Self::new()
    }
}
