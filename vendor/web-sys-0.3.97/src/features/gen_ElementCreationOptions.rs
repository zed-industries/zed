#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ElementCreationOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ElementCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    pub type ElementCreationOptions;
    #[doc = "Get the `is` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    #[wasm_bindgen(method, getter = "is")]
    pub fn get_is(this: &ElementCreationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `is` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    #[wasm_bindgen(method, setter = "is")]
    pub fn set_is(this: &ElementCreationOptions, val: &str);
    #[doc = "Get the `pseudo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    #[wasm_bindgen(method, getter = "pseudo")]
    pub fn get_pseudo(this: &ElementCreationOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `pseudo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    #[wasm_bindgen(method, setter = "pseudo")]
    pub fn set_pseudo(this: &ElementCreationOptions, val: &str);
}
impl ElementCreationOptions {
    #[doc = "Construct a new `ElementCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is()` instead."]
    pub fn is(&mut self, val: &str) -> &mut Self {
        self.set_is(val);
        self
    }
    #[deprecated = "Use `set_pseudo()` instead."]
    pub fn pseudo(&mut self, val: &str) -> &mut Self {
        self.set_pseudo(val);
        self
    }
}
impl Default for ElementCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}
