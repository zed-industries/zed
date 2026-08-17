#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConsoleCounterError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleCounterError` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    pub type ConsoleCounterError;
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &ConsoleCounterError) -> Option<::alloc::string::String>;
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &ConsoleCounterError, val: &str);
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &ConsoleCounterError) -> Option<::alloc::string::String>;
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &ConsoleCounterError, val: &str);
}
impl ConsoleCounterError {
    #[doc = "Construct a new `ConsoleCounterError`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: &str) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
}
impl Default for ConsoleCounterError {
    fn default() -> Self {
        Self::new()
    }
}
