#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SocketOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub type SocketOptions;
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Get the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
    #[wasm_bindgen(method, getter = "binaryType")]
    pub fn get_binary_type(this: &SocketOptions) -> Option<TcpSocketBinaryType>;
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
    #[wasm_bindgen(method, setter = "binaryType")]
    pub fn set_binary_type(this: &SocketOptions, val: TcpSocketBinaryType);
    #[doc = "Get the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    #[wasm_bindgen(method, getter = "useSecureTransport")]
    pub fn get_use_secure_transport(this: &SocketOptions) -> Option<bool>;
    #[doc = "Change the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    #[wasm_bindgen(method, setter = "useSecureTransport")]
    pub fn set_use_secure_transport(this: &SocketOptions, val: bool);
}
impl SocketOptions {
    #[doc = "Construct a new `SocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[deprecated = "Use `set_binary_type()` instead."]
    pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self {
        self.set_binary_type(val);
        self
    }
    #[deprecated = "Use `set_use_secure_transport()` instead."]
    pub fn use_secure_transport(&mut self, val: bool) -> &mut Self {
        self.set_use_secure_transport(val);
        self
    }
}
impl Default for SocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
