#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ClientRectsAndTexts")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClientRectsAndTexts` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    pub type ClientRectsAndTexts;
    #[cfg(feature = "DomRectList")]
    #[doc = "Get the `rectList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    #[wasm_bindgen(method, getter = "rectList")]
    pub fn get_rect_list(this: &ClientRectsAndTexts) -> DomRectList;
    #[cfg(feature = "DomRectList")]
    #[doc = "Change the `rectList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    #[wasm_bindgen(method, setter = "rectList")]
    pub fn set_rect_list(this: &ClientRectsAndTexts, val: &DomRectList);
    #[doc = "Get the `textList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    #[wasm_bindgen(method, getter = "textList")]
    pub fn get_text_list(this: &ClientRectsAndTexts) -> ::js_sys::Array;
    #[doc = "Change the `textList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`*"]
    #[wasm_bindgen(method, setter = "textList")]
    pub fn set_text_list(this: &ClientRectsAndTexts, val: &::wasm_bindgen::JsValue);
}
impl ClientRectsAndTexts {
    #[cfg(feature = "DomRectList")]
    #[doc = "Construct a new `ClientRectsAndTexts`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientRectsAndTexts`, `DomRectList`*"]
    pub fn new(rect_list: &DomRectList, text_list: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_rect_list(rect_list);
        ret.set_text_list(text_list);
        ret
    }
    #[cfg(feature = "DomRectList")]
    #[deprecated = "Use `set_rect_list()` instead."]
    pub fn rect_list(&mut self, val: &DomRectList) -> &mut Self {
        self.set_rect_list(val);
        self
    }
    #[deprecated = "Use `set_text_list()` instead."]
    pub fn text_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_text_list(val);
        self
    }
}
