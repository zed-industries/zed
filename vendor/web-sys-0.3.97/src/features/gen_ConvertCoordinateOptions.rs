#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConvertCoordinateOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvertCoordinateOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
    pub type ConvertCoordinateOptions;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    #[wasm_bindgen(method, getter = "fromBox")]
    pub fn get_from_box(this: &ConvertCoordinateOptions) -> Option<CssBoxType>;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    #[wasm_bindgen(method, setter = "fromBox")]
    pub fn set_from_box(this: &ConvertCoordinateOptions, val: CssBoxType);
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    #[wasm_bindgen(method, getter = "toBox")]
    pub fn get_to_box(this: &ConvertCoordinateOptions) -> Option<CssBoxType>;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    #[wasm_bindgen(method, setter = "toBox")]
    pub fn set_to_box(this: &ConvertCoordinateOptions, val: CssBoxType);
}
impl ConvertCoordinateOptions {
    #[doc = "Construct a new `ConvertCoordinateOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[deprecated = "Use `set_from_box()` instead."]
    pub fn from_box(&mut self, val: CssBoxType) -> &mut Self {
        self.set_from_box(val);
        self
    }
    #[cfg(feature = "CssBoxType")]
    #[deprecated = "Use `set_to_box()` instead."]
    pub fn to_box(&mut self, val: CssBoxType) -> &mut Self {
        self.set_to_box(val);
        self
    }
}
impl Default for ConvertCoordinateOptions {
    fn default() -> Self {
        Self::new()
    }
}
