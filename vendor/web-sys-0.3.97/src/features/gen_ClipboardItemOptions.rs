#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ClipboardItemOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardItemOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"]
    pub type ClipboardItemOptions;
    #[cfg(feature = "PresentationStyle")]
    #[doc = "Get the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"]
    #[wasm_bindgen(method, getter = "presentationStyle")]
    pub fn get_presentation_style(this: &ClipboardItemOptions) -> Option<PresentationStyle>;
    #[cfg(feature = "PresentationStyle")]
    #[doc = "Change the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`, `PresentationStyle`*"]
    #[wasm_bindgen(method, setter = "presentationStyle")]
    pub fn set_presentation_style(this: &ClipboardItemOptions, val: PresentationStyle);
}
impl ClipboardItemOptions {
    #[doc = "Construct a new `ClipboardItemOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItemOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "PresentationStyle")]
    #[deprecated = "Use `set_presentation_style()` instead."]
    pub fn presentation_style(&mut self, val: PresentationStyle) -> &mut Self {
        self.set_presentation_style(val);
        self
    }
}
impl Default for ClipboardItemOptions {
    fn default() -> Self {
        Self::new()
    }
}
