#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "StyleSheetChangeEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub type StyleSheetChangeEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &StyleSheetChangeEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &StyleSheetChangeEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &StyleSheetChangeEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &StyleSheetChangeEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &StyleSheetChangeEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &StyleSheetChangeEventInit, val: bool);
    #[doc = "Get the `documentSheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "documentSheet")]
    pub fn get_document_sheet(this: &StyleSheetChangeEventInit) -> Option<bool>;
    #[doc = "Change the `documentSheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "documentSheet")]
    pub fn set_document_sheet(this: &StyleSheetChangeEventInit, val: bool);
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, getter = "stylesheet")]
    pub fn get_stylesheet(this: &StyleSheetChangeEventInit) -> Option<CssStyleSheet>;
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleSheetChangeEventInit`*"]
    #[wasm_bindgen(method, setter = "stylesheet")]
    pub fn set_stylesheet(this: &StyleSheetChangeEventInit, val: Option<&CssStyleSheet>);
}
impl StyleSheetChangeEventInit {
    #[doc = "Construct a new `StyleSheetChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_document_sheet()` instead."]
    pub fn document_sheet(&mut self, val: bool) -> &mut Self {
        self.set_document_sheet(val);
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[deprecated = "Use `set_stylesheet()` instead."]
    pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self {
        self.set_stylesheet(val);
        self
    }
}
impl Default for StyleSheetChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
