#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TextDecodeOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub type TextDecodeOptions;
    #[doc = "Get the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    #[wasm_bindgen(method, getter = "stream")]
    pub fn get_stream(this: &TextDecodeOptions) -> Option<bool>;
    #[doc = "Change the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    #[wasm_bindgen(method, setter = "stream")]
    pub fn set_stream(this: &TextDecodeOptions, val: bool);
}
impl TextDecodeOptions {
    #[doc = "Construct a new `TextDecodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_stream()` instead."]
    pub fn stream(&mut self, val: bool) -> &mut Self {
        self.set_stream(val);
        self
    }
}
impl Default for TextDecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
