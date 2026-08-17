#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "BoxQuadOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BoxQuadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub type BoxQuadOptions;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
    #[wasm_bindgen(method, getter = "box")]
    pub fn get_box(this: &BoxQuadOptions) -> Option<CssBoxType>;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `box` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `CssBoxType`*"]
    #[wasm_bindgen(method, setter = "box")]
    pub fn set_box(this: &BoxQuadOptions, val: CssBoxType);
    #[doc = "Get the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    #[wasm_bindgen(method, getter = "relativeTo")]
    pub fn get_relative_to(this: &BoxQuadOptions) -> Option<::js_sys::Object>;
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    #[wasm_bindgen(method, setter = "relativeTo")]
    pub fn set_relative_to(this: &BoxQuadOptions, val: &::js_sys::Object);
    #[cfg(feature = "Text")]
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Text`*"]
    #[wasm_bindgen(method, setter = "relativeTo")]
    pub fn set_relative_to_text(this: &BoxQuadOptions, val: &Text);
    #[cfg(feature = "Element")]
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Element`*"]
    #[wasm_bindgen(method, setter = "relativeTo")]
    pub fn set_relative_to_element(this: &BoxQuadOptions, val: &Element);
    #[cfg(feature = "Document")]
    #[doc = "Change the `relativeTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`, `Document`*"]
    #[wasm_bindgen(method, setter = "relativeTo")]
    pub fn set_relative_to_document(this: &BoxQuadOptions, val: &Document);
}
impl BoxQuadOptions {
    #[doc = "Construct a new `BoxQuadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BoxQuadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[deprecated = "Use `set_box()` instead."]
    pub fn box_(&mut self, val: CssBoxType) -> &mut Self {
        self.set_box(val);
        self
    }
    #[deprecated = "Use `set_relative_to()` instead."]
    pub fn relative_to(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_relative_to(val);
        self
    }
}
impl Default for BoxQuadOptions {
    fn default() -> Self {
        Self::new()
    }
}
