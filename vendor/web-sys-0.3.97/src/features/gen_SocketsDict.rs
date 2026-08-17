#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SocketsDict")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketsDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub type SocketsDict;
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    #[wasm_bindgen(method, getter = "received")]
    pub fn get_received(this: &SocketsDict) -> Option<f64>;
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    #[wasm_bindgen(method, setter = "received")]
    pub fn set_received(this: &SocketsDict, val: f64);
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    #[wasm_bindgen(method, getter = "sent")]
    pub fn get_sent(this: &SocketsDict) -> Option<f64>;
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    #[wasm_bindgen(method, setter = "sent")]
    pub fn set_sent(this: &SocketsDict, val: f64);
    #[doc = "Get the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    #[wasm_bindgen(method, getter = "sockets")]
    pub fn get_sockets(this: &SocketsDict) -> Option<::js_sys::Array>;
    #[doc = "Change the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    #[wasm_bindgen(method, setter = "sockets")]
    pub fn set_sockets(this: &SocketsDict, val: &::wasm_bindgen::JsValue);
}
impl SocketsDict {
    #[doc = "Construct a new `SocketsDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_received()` instead."]
    pub fn received(&mut self, val: f64) -> &mut Self {
        self.set_received(val);
        self
    }
    #[deprecated = "Use `set_sent()` instead."]
    pub fn sent(&mut self, val: f64) -> &mut Self {
        self.set_sent(val);
        self
    }
    #[deprecated = "Use `set_sockets()` instead."]
    pub fn sockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_sockets(val);
        self
    }
}
impl Default for SocketsDict {
    fn default() -> Self {
        Self::new()
    }
}
