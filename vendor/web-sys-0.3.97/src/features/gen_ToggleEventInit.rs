#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ToggleEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ToggleEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    pub type ToggleEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ToggleEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &ToggleEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ToggleEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &ToggleEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ToggleEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &ToggleEventInit, val: bool);
    #[doc = "Get the `newState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, getter = "newState")]
    pub fn get_new_state(this: &ToggleEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `newState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, setter = "newState")]
    pub fn set_new_state(this: &ToggleEventInit, val: &str);
    #[doc = "Get the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, getter = "oldState")]
    pub fn get_old_state(this: &ToggleEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
    #[wasm_bindgen(method, setter = "oldState")]
    pub fn set_old_state(this: &ToggleEventInit, val: &str);
}
impl ToggleEventInit {
    #[doc = "Construct a new `ToggleEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
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
    #[deprecated = "Use `set_new_state()` instead."]
    pub fn new_state(&mut self, val: &str) -> &mut Self {
        self.set_new_state(val);
        self
    }
    #[deprecated = "Use `set_old_state()` instead."]
    pub fn old_state(&mut self, val: &str) -> &mut Self {
        self.set_old_state(val);
        self
    }
}
impl Default for ToggleEventInit {
    fn default() -> Self {
        Self::new()
    }
}
