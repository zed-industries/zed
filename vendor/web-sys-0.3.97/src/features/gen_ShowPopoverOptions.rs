#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ShowPopoverOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShowPopoverOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShowPopoverOptions`*"]
    pub type ShowPopoverOptions;
    #[cfg(feature = "HtmlElement")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `ShowPopoverOptions`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &ShowPopoverOptions) -> Option<HtmlElement>;
    #[cfg(feature = "HtmlElement")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlElement`, `ShowPopoverOptions`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &ShowPopoverOptions, val: &HtmlElement);
}
impl ShowPopoverOptions {
    #[doc = "Construct a new `ShowPopoverOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShowPopoverOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "HtmlElement")]
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: &HtmlElement) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for ShowPopoverOptions {
    fn default() -> Self {
        Self::new()
    }
}
