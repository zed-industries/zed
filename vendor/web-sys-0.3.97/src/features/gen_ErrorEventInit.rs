#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ErrorEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub type ErrorEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ErrorEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &ErrorEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ErrorEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &ErrorEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ErrorEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &ErrorEventInit, val: bool);
    #[doc = "Get the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "colno")]
    pub fn get_colno(this: &ErrorEventInit) -> Option<u32>;
    #[doc = "Change the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "colno")]
    pub fn set_colno(this: &ErrorEventInit, val: u32);
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &ErrorEventInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &ErrorEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &ErrorEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &ErrorEventInit, val: &str);
    #[doc = "Get the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "lineno")]
    pub fn get_lineno(this: &ErrorEventInit) -> Option<u32>;
    #[doc = "Change the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "lineno")]
    pub fn set_lineno(this: &ErrorEventInit, val: u32);
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &ErrorEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &ErrorEventInit, val: &str);
}
impl ErrorEventInit {
    #[doc = "Construct a new `ErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[deprecated = "Use `set_colno()` instead."]
    pub fn colno(&mut self, val: u32) -> &mut Self {
        self.set_colno(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_lineno()` instead."]
    pub fn lineno(&mut self, val: u32) -> &mut Self {
        self.set_lineno(val);
        self
    }
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message(val);
        self
    }
}
impl Default for ErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
