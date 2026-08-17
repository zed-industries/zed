#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBAlternateInterface",
        typescript_type = "USBAlternateInterface"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbAlternateInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbAlternateInterface;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBAlternateInterface",
        js_name = "alternateSetting"
    )]
    #[doc = "Getter for the `alternateSetting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/alternateSetting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alternate_setting(this: &UsbAlternateInterface) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBAlternateInterface",
        js_name = "interfaceClass"
    )]
    #[doc = "Getter for the `interfaceClass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceClass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn interface_class(this: &UsbAlternateInterface) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBAlternateInterface",
        js_name = "interfaceSubclass"
    )]
    #[doc = "Getter for the `interfaceSubclass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceSubclass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn interface_subclass(this: &UsbAlternateInterface) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBAlternateInterface",
        js_name = "interfaceProtocol"
    )]
    #[doc = "Getter for the `interfaceProtocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceProtocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn interface_protocol(this: &UsbAlternateInterface) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBAlternateInterface",
        js_name = "interfaceName"
    )]
    #[doc = "Getter for the `interfaceName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/interfaceName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn interface_name(this: &UsbAlternateInterface) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbEndpoint")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBAlternateInterface",
        js_name = "endpoints"
    )]
    #[doc = "Getter for the `endpoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/endpoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn endpoints(this: &UsbAlternateInterface) -> ::js_sys::Array<UsbEndpoint>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbInterface")]
    #[wasm_bindgen(catch, constructor, js_class = "USBAlternateInterface")]
    #[doc = "The `new UsbAlternateInterface(..)` constructor, creating a new instance of `UsbAlternateInterface`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBAlternateInterface/USBAlternateInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        device_interface: &UsbInterface,
        alternate_setting: u8,
    ) -> Result<UsbAlternateInterface, JsValue>;
}
