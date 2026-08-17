#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ClipboardEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    pub type ClipboardEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &ClipboardEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &ClipboardEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &ClipboardEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &ClipboardEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &ClipboardEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &ClipboardEventInit, val: bool);
    #[cfg(feature = "DataTransfer")]
    #[doc = "Get the `clipboardData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`, `DataTransfer`*"]
    #[wasm_bindgen(method, getter = "clipboardData")]
    pub fn get_clipboard_data(this: &ClipboardEventInit) -> Option<DataTransfer>;
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `clipboardData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`, `DataTransfer`*"]
    #[wasm_bindgen(method, setter = "clipboardData")]
    pub fn set_clipboard_data(this: &ClipboardEventInit, val: Option<&DataTransfer>);
}
impl ClipboardEventInit {
    #[doc = "Construct a new `ClipboardEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardEventInit`*"]
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
    #[cfg(feature = "DataTransfer")]
    #[deprecated = "Use `set_clipboard_data()` instead."]
    pub fn clipboard_data(&mut self, val: Option<&DataTransfer>) -> &mut Self {
        self.set_clipboard_data(val);
        self
    }
}
impl Default for ClipboardEventInit {
    fn default() -> Self {
        Self::new()
    }
}
