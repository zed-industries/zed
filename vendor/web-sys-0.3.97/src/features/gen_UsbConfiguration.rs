#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBConfiguration",
        typescript_type = "USBConfiguration"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbConfiguration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbConfiguration;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBConfiguration",
        js_name = "configurationValue"
    )]
    #[doc = "Getter for the `configurationValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/configurationValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configuration_value(this: &UsbConfiguration) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBConfiguration",
        js_name = "configurationName"
    )]
    #[doc = "Getter for the `configurationName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/configurationName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configuration_name(this: &UsbConfiguration) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbInterface")]
    #[wasm_bindgen(method, getter, js_class = "USBConfiguration", js_name = "interfaces")]
    #[doc = "Getter for the `interfaces` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/interfaces)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`, `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn interfaces(this: &UsbConfiguration) -> ::js_sys::Array<UsbInterface>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDevice")]
    #[wasm_bindgen(catch, constructor, js_class = "USBConfiguration")]
    #[doc = "The `new UsbConfiguration(..)` constructor, creating a new instance of `UsbConfiguration`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConfiguration/USBConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(device: &UsbDevice, configuration_value: u8) -> Result<UsbConfiguration, JsValue>;
}
