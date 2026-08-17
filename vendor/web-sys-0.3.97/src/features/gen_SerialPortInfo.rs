#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "SerialPortInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialPortInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialPortInfo;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usbProductId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usbProductId")]
    pub fn get_usb_product_id(this: &SerialPortInfo) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usbProductId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usbProductId")]
    pub fn set_usb_product_id(this: &SerialPortInfo, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usbVendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "usbVendorId")]
    pub fn get_usb_vendor_id(this: &SerialPortInfo) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usbVendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "usbVendorId")]
    pub fn set_usb_vendor_id(this: &SerialPortInfo, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl SerialPortInfo {
    #[doc = "Construct a new `SerialPortInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialPortInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usb_product_id()` instead."]
    pub fn usb_product_id(&mut self, val: u16) -> &mut Self {
        self.set_usb_product_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_usb_vendor_id()` instead."]
    pub fn usb_vendor_id(&mut self, val: u16) -> &mut Self {
        self.set_usb_vendor_id(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for SerialPortInfo {
    fn default() -> Self {
        Self::new()
    }
}
