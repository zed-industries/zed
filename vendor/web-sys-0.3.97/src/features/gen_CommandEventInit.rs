#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "CommandEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CommandEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    pub type CommandEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &CommandEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &CommandEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &CommandEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &CommandEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &CommandEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &CommandEventInit, val: bool);
    #[doc = "Get the `command` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, getter = "command")]
    pub fn get_command(this: &CommandEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `command` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
    #[wasm_bindgen(method, setter = "command")]
    pub fn set_command(this: &CommandEventInit, val: &str);
    #[cfg(feature = "Element")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`, `Element`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &CommandEventInit) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`, `Element`*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &CommandEventInit, val: Option<&Element>);
}
impl CommandEventInit {
    #[doc = "Construct a new `CommandEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CommandEventInit`*"]
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
    #[deprecated = "Use `set_command()` instead."]
    pub fn command(&mut self, val: &str) -> &mut Self {
        self.set_command(val);
        self
    }
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for CommandEventInit {
    fn default() -> Self {
        Self::new()
    }
}
