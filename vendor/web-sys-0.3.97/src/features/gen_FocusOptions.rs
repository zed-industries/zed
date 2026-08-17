#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "FocusOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FocusOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    pub type FocusOptions;
    #[doc = "Get the `focusVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    #[wasm_bindgen(method, getter = "focusVisible")]
    pub fn get_focus_visible(this: &FocusOptions) -> Option<bool>;
    #[doc = "Change the `focusVisible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    #[wasm_bindgen(method, setter = "focusVisible")]
    pub fn set_focus_visible(this: &FocusOptions, val: bool);
    #[doc = "Get the `preventScroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    #[wasm_bindgen(method, getter = "preventScroll")]
    pub fn get_prevent_scroll(this: &FocusOptions) -> Option<bool>;
    #[doc = "Change the `preventScroll` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    #[wasm_bindgen(method, setter = "preventScroll")]
    pub fn set_prevent_scroll(this: &FocusOptions, val: bool);
}
impl FocusOptions {
    #[doc = "Construct a new `FocusOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_focus_visible()` instead."]
    pub fn focus_visible(&mut self, val: bool) -> &mut Self {
        self.set_focus_visible(val);
        self
    }
    #[deprecated = "Use `set_prevent_scroll()` instead."]
    pub fn prevent_scroll(&mut self, val: bool) -> &mut Self {
        self.set_prevent_scroll(val);
        self
    }
}
impl Default for FocusOptions {
    fn default() -> Self {
        Self::new()
    }
}
