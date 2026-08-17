#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "UDPMessageEventInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub type UdpMessageEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &UdpMessageEventInit) -> Option<bool>;
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, setter = "bubbles")]
    pub fn set_bubbles(this: &UdpMessageEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &UdpMessageEventInit) -> Option<bool>;
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, setter = "cancelable")]
    pub fn set_cancelable(this: &UdpMessageEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &UdpMessageEventInit) -> Option<bool>;
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, setter = "composed")]
    pub fn set_composed(this: &UdpMessageEventInit, val: bool);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &UdpMessageEventInit) -> ::wasm_bindgen::JsValue;
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &UdpMessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "remoteAddress")]
    pub fn get_remote_address(this: &UdpMessageEventInit) -> Option<::alloc::string::String>;
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, setter = "remoteAddress")]
    pub fn set_remote_address(this: &UdpMessageEventInit, val: &str);
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "remotePort")]
    pub fn get_remote_port(this: &UdpMessageEventInit) -> Option<u16>;
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, setter = "remotePort")]
    pub fn set_remote_port(this: &UdpMessageEventInit, val: u16);
}
impl UdpMessageEventInit {
    #[doc = "Construct a new `UdpMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
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
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_remote_address()` instead."]
    pub fn remote_address(&mut self, val: &str) -> &mut Self {
        self.set_remote_address(val);
        self
    }
    #[deprecated = "Use `set_remote_port()` instead."]
    pub fn remote_port(&mut self, val: u16) -> &mut Self {
        self.set_remote_port(val);
        self
    }
}
impl Default for UdpMessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
