#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TextDecoderOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecoderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub type TextDecoderOptions;
    #[doc = "Get the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    #[wasm_bindgen(method, getter = "fatal")]
    pub fn get_fatal(this: &TextDecoderOptions) -> Option<bool>;
    #[doc = "Change the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    #[wasm_bindgen(method, setter = "fatal")]
    pub fn set_fatal(this: &TextDecoderOptions, val: bool);
}
impl TextDecoderOptions {
    #[doc = "Construct a new `TextDecoderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_fatal()` instead."]
    pub fn fatal(&mut self, val: bool) -> &mut Self {
        self.set_fatal(val);
        self
    }
}
impl Default for TextDecoderOptions {
    fn default() -> Self {
        Self::new()
    }
}
