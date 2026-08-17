#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBInterface",
        typescript_type = "USBInterface"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbInterface` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbInterface;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBInterface", js_name = "interfaceNumber")]
    #[doc = "Getter for the `interfaceNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/interfaceNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn interface_number(this: &UsbInterface) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbAlternateInterface")]
    #[wasm_bindgen(method, getter, js_class = "USBInterface", js_name = "alternate")]
    #[doc = "Getter for the `alternate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/alternate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alternate(this: &UsbInterface) -> UsbAlternateInterface;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbAlternateInterface")]
    #[wasm_bindgen(method, getter, js_class = "USBInterface", js_name = "alternates")]
    #[doc = "Getter for the `alternates` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/alternates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn alternates(this: &UsbInterface) -> ::js_sys::Array<UsbAlternateInterface>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBInterface", js_name = "claimed")]
    #[doc = "Getter for the `claimed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/claimed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn claimed(this: &UsbInterface) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbConfiguration")]
    #[wasm_bindgen(catch, constructor, js_class = "USBInterface")]
    #[doc = "The `new UsbInterface(..)` constructor, creating a new instance of `UsbInterface`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBInterface/USBInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`, `UsbInterface`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        configuration: &UsbConfiguration,
        interface_number: u8,
    ) -> Result<UsbInterface, JsValue>;
}
