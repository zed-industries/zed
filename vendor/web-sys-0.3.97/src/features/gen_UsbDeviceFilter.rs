#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "USBDeviceFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbDeviceFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbDeviceFilter;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `classCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "classCode")]
    pub fn get_class_code(this: &UsbDeviceFilter) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `classCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "classCode")]
    pub fn set_class_code(this: &UsbDeviceFilter, val: u8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &UsbDeviceFilter) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &UsbDeviceFilter, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `protocolCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "protocolCode")]
    pub fn get_protocol_code(this: &UsbDeviceFilter) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `protocolCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "protocolCode")]
    pub fn set_protocol_code(this: &UsbDeviceFilter, val: u8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "serialNumber")]
    pub fn get_serial_number(this: &UsbDeviceFilter) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "serialNumber")]
    pub fn set_serial_number(this: &UsbDeviceFilter, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `subclassCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "subclassCode")]
    pub fn get_subclass_code(this: &UsbDeviceFilter) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `subclassCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "subclassCode")]
    pub fn set_subclass_code(this: &UsbDeviceFilter, val: u8);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &UsbDeviceFilter) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &UsbDeviceFilter, val: u16);
}
#[cfg(web_sys_unstable_apis)]
impl UsbDeviceFilter {
    #[doc = "Construct a new `UsbDeviceFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_class_code()` instead."]
    pub fn class_code(&mut self, val: u8) -> &mut Self {
        self.set_class_code(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: u16) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_protocol_code()` instead."]
    pub fn protocol_code(&mut self, val: u8) -> &mut Self {
        self.set_protocol_code(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_serial_number()` instead."]
    pub fn serial_number(&mut self, val: &str) -> &mut Self {
        self.set_serial_number(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_subclass_code()` instead."]
    pub fn subclass_code(&mut self, val: u8) -> &mut Self {
        self.set_subclass_code(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: u16) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for UsbDeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}
