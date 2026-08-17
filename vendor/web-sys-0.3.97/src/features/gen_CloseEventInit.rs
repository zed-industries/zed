#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CloseEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    pub type CloseEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &CloseEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &CloseEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &CloseEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &CloseEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &CloseEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &CloseEventInit, val: bool);
    #[doc = "Get the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &CloseEventInit) -> Option<u16>;
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &CloseEventInit, val: u16);
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &CloseEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &CloseEventInit, val: &str);
    #[doc = "Get the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, getter = "wasClean")]
    pub fn get_was_clean(this: &CloseEventInit) -> Option<bool>;
    #[doc = "Change the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
    #[wasm_bindgen(method, setter = "wasClean")]
    pub fn set_was_clean(this: &CloseEventInit, val: bool);
}
impl CloseEventInit {
    #[doc = "Construct a new `CloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
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
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: u16) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.set_reason(val);
        self
    }
    #[deprecated = "Use `set_was_clean()` instead."]
    pub fn was_clean(&mut self, val: bool) -> &mut Self {
        self.set_was_clean(val);
        self
    }
}
impl Default for CloseEventInit {
    fn default() -> Self {
        Self::new()
    }
}
