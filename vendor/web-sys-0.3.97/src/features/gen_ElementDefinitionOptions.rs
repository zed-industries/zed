#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ElementDefinitionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ElementDefinitionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    pub type ElementDefinitionOptions;
    #[doc = "Get the `extends` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    #[wasm_bindgen(method, getter = "extends")]
    pub fn get_extends(this: &ElementDefinitionOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `extends` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    #[wasm_bindgen(method, setter = "extends")]
    pub fn set_extends(this: &ElementDefinitionOptions, val: &str);
}
impl ElementDefinitionOptions {
    #[doc = "Construct a new `ElementDefinitionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extends()` instead."]
    pub fn extends(&mut self, val: &str) -> &mut Self {
        self.set_extends(val);
        self
    }
}
impl Default for ElementDefinitionOptions {
    fn default() -> Self {
        Self::new()
    }
}
