#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "UDPOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub type UdpOptions;
    #[doc = "Get the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, getter = "addressReuse")]
    pub fn get_address_reuse(this: &UdpOptions) -> Option<bool>;
    #[doc = "Change the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, setter = "addressReuse")]
    pub fn set_address_reuse(this: &UdpOptions, val: bool);
    #[doc = "Get the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, getter = "localAddress")]
    pub fn get_local_address(this: &UdpOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, setter = "localAddress")]
    pub fn set_local_address(this: &UdpOptions, val: &str);
    #[doc = "Get the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, getter = "localPort")]
    pub fn get_local_port(this: &UdpOptions) -> Option<u16>;
    #[doc = "Change the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, setter = "localPort")]
    pub fn set_local_port(this: &UdpOptions, val: u16);
    #[doc = "Get the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, getter = "loopback")]
    pub fn get_loopback(this: &UdpOptions) -> Option<bool>;
    #[doc = "Change the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, setter = "loopback")]
    pub fn set_loopback(this: &UdpOptions, val: bool);
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, getter = "remoteAddress")]
    pub fn get_remote_address(this: &UdpOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, setter = "remoteAddress")]
    pub fn set_remote_address(this: &UdpOptions, val: &str);
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, getter = "remotePort")]
    pub fn get_remote_port(this: &UdpOptions) -> Option<u16>;
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    #[wasm_bindgen(method, setter = "remotePort")]
    pub fn set_remote_port(this: &UdpOptions, val: u16);
}
impl UdpOptions {
    #[doc = "Construct a new `UdpOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_address_reuse()` instead."]
    pub fn address_reuse(&mut self, val: bool) -> &mut Self {
        self.set_address_reuse(val);
        self
    }
    #[deprecated = "Use `set_local_address()` instead."]
    pub fn local_address(&mut self, val: &str) -> &mut Self {
        self.set_local_address(val);
        self
    }
    #[deprecated = "Use `set_local_port()` instead."]
    pub fn local_port(&mut self, val: u16) -> &mut Self {
        self.set_local_port(val);
        self
    }
    #[deprecated = "Use `set_loopback()` instead."]
    pub fn loopback(&mut self, val: bool) -> &mut Self {
        self.set_loopback(val);
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
impl Default for UdpOptions {
    fn default() -> Self {
        Self::new()
    }
}
