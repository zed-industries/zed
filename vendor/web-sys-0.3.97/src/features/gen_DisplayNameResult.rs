#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "DisplayNameResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub type DisplayNameResult;
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    #[wasm_bindgen(method, getter = "locale")]
    pub fn get_locale(this: &DisplayNameResult) -> Option<::alloc::string::String>;
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    #[wasm_bindgen(method, setter = "locale")]
    pub fn set_locale(this: &DisplayNameResult, val: &str);
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    #[wasm_bindgen(method, getter = "style")]
    pub fn get_style(this: &DisplayNameResult) -> Option<::alloc::string::String>;
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    #[wasm_bindgen(method, setter = "style")]
    pub fn set_style(this: &DisplayNameResult, val: &str);
    #[doc = "Get the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    #[wasm_bindgen(method, getter = "values")]
    pub fn get_values(this: &DisplayNameResult) -> Option<::js_sys::Object>;
    #[doc = "Change the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    #[wasm_bindgen(method, setter = "values")]
    pub fn set_values(this: &DisplayNameResult, val: &::js_sys::Object);
}
impl DisplayNameResult {
    #[doc = "Construct a new `DisplayNameResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_locale()` instead."]
    pub fn locale(&mut self, val: &str) -> &mut Self {
        self.set_locale(val);
        self
    }
    #[deprecated = "Use `set_style()` instead."]
    pub fn style(&mut self, val: &str) -> &mut Self {
        self.set_style(val);
        self
    }
    #[deprecated = "Use `set_values()` instead."]
    pub fn values(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_values(val);
        self
    }
}
impl Default for DisplayNameResult {
    fn default() -> Self {
        Self::new()
    }
}
