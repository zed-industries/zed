#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WebSocketDict")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocketDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub type WebSocketDict;
    #[doc = "Get the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    #[wasm_bindgen(method, getter = "websockets")]
    pub fn get_websockets(this: &WebSocketDict) -> Option<::js_sys::Array>;
    #[doc = "Change the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    #[wasm_bindgen(method, setter = "websockets")]
    pub fn set_websockets(this: &WebSocketDict, val: &::wasm_bindgen::JsValue);
}
impl WebSocketDict {
    #[doc = "Construct a new `WebSocketDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_websockets()` instead."]
    pub fn websockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_websockets(val);
        self
    }
}
impl Default for WebSocketDict {
    fn default() -> Self {
        Self::new()
    }
}
