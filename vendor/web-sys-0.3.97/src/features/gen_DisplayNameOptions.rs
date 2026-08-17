#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DisplayNameOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub type DisplayNameOptions;
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    #[wasm_bindgen(method, getter = "keys")]
    pub fn get_keys(this: &DisplayNameOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    #[wasm_bindgen(method, setter = "keys")]
    pub fn set_keys(this: &DisplayNameOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    #[wasm_bindgen(method, getter = "style")]
    pub fn get_style(this: &DisplayNameOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    #[wasm_bindgen(method, setter = "style")]
    pub fn set_style(this: &DisplayNameOptions, val: &str);
}
impl DisplayNameOptions {
    #[doc = "Construct a new `DisplayNameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_keys()` instead."]
    pub fn keys(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_keys(val);
        self
    }
    #[deprecated = "Use `set_style()` instead."]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.set_style(val);
        self
    }
}
impl Default for DisplayNameOptions {
    fn default() -> Self {
        Self::new()
    }
}
