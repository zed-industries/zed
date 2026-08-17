#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConsoleCounter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleCounter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub type ConsoleCounter;
    #[doc = "Get the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    #[wasm_bindgen(method, getter = "count")]
    pub fn get_count(this: &ConsoleCounter) -> Option<u32>;
    #[doc = "Change the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    #[wasm_bindgen(method, setter = "count")]
    pub fn set_count(this: &ConsoleCounter, val: u32);
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &ConsoleCounter) -> Option<::alloc::string::String>;
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &ConsoleCounter, val: &str);
}
impl ConsoleCounter {
    #[doc = "Construct a new `ConsoleCounter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_count()` instead."]
    pub fn count(&mut self, val: u32) -> &mut Self {
        self.set_count(val);
        self
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
}
impl Default for ConsoleCounter {
    fn default() -> Self {
        Self::new()
    }
}
