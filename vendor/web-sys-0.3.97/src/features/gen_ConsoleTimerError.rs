#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ConsoleTimerError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerError` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    pub type ConsoleTimerError;
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &ConsoleTimerError) -> Option<::alloc::string::String>;
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &ConsoleTimerError, val: &str);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ConsoleTimerError) -> Option<::alloc::string::String>;
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ConsoleTimerError, val: &str);
}
impl ConsoleTimerError {
    #[doc = "Construct a new `ConsoleTimerError`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for ConsoleTimerError {
    fn default() -> Self {
        Self::new()
    }
}
