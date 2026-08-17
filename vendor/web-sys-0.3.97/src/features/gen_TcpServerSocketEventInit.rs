#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "TCPServerSocketEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocketEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    pub type TcpServerSocketEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &TcpServerSocketEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &TcpServerSocketEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &TcpServerSocketEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &TcpServerSocketEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &TcpServerSocketEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &TcpServerSocketEventInit, val: bool);
    #[cfg(feature = "TcpSocket")]
    #[doc = "Get the `socket` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`, `TcpSocket`*"]
    #[wasm_bindgen(method, getter = "socket")]
    pub fn get_socket(this: &TcpServerSocketEventInit) -> Option<TcpSocket>;
    #[cfg(feature = "TcpSocket")]
    #[doc = "Change the `socket` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`, `TcpSocket`*"]
    #[wasm_bindgen(method, setter = "socket")]
    pub fn set_socket(this: &TcpServerSocketEventInit, val: Option<&TcpSocket>);
}
impl TcpServerSocketEventInit {
    #[doc = "Construct a new `TcpServerSocketEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
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
    #[cfg(feature = "TcpSocket")]
    #[deprecated = "Use `set_socket()` instead."]
    pub fn socket(&mut self, val: Option<&TcpSocket>) -> &mut Self {
        self.set_socket(val);
        self
    }
}
impl Default for TcpServerSocketEventInit {
    fn default() -> Self {
        Self::new()
    }
}
