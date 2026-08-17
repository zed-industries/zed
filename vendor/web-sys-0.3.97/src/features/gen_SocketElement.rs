#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SocketElement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub type SocketElement;
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &SocketElement) -> Option<bool>;
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &SocketElement, val: bool);
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "host")]
    pub fn get_host(this: &SocketElement) -> Option<::alloc::string::String>;
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, setter = "host")]
    pub fn set_host(this: &SocketElement, val: &str);
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &SocketElement) -> Option<u32>;
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, setter = "port")]
    pub fn set_port(this: &SocketElement, val: u32);
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "received")]
    pub fn get_received(this: &SocketElement) -> Option<f64>;
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, setter = "received")]
    pub fn set_received(this: &SocketElement, val: f64);
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "sent")]
    pub fn get_sent(this: &SocketElement) -> Option<f64>;
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, setter = "sent")]
    pub fn set_sent(this: &SocketElement, val: f64);
    #[doc = "Get the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "tcp")]
    pub fn get_tcp(this: &SocketElement) -> Option<bool>;
    #[doc = "Change the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, setter = "tcp")]
    pub fn set_tcp(this: &SocketElement, val: bool);
}
impl SocketElement {
    #[doc = "Construct a new `SocketElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.set_port(val);
        self
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
    #[deprecated = "Use `set_tcp()` instead."]
    pub fn tcp(&mut self, val: bool) -> &mut Self {
        self.set_tcp(val);
        self
    }
}
impl Default for SocketElement {
    fn default() -> Self {
        Self::new()
    }
}
