#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBEndpoint",
        typescript_type = "USBEndpoint"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbEndpoint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbEndpoint;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBEndpoint", js_name = "endpointNumber")]
    #[doc = "Getter for the `endpointNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/endpointNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn endpoint_number(this: &UsbEndpoint) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDirection")]
    #[wasm_bindgen(method, getter, js_class = "USBEndpoint", js_name = "direction")]
    #[doc = "Getter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDirection`, `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn direction(this: &UsbEndpoint) -> UsbDirection;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbEndpointType")]
    #[wasm_bindgen(method, getter, js_class = "USBEndpoint", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`, `UsbEndpointType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &UsbEndpoint) -> UsbEndpointType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBEndpoint", js_name = "packetSize")]
    #[doc = "Getter for the `packetSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/packetSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn packet_size(this: &UsbEndpoint) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "UsbAlternateInterface", feature = "UsbDirection",))]
    #[wasm_bindgen(catch, constructor, js_class = "USBEndpoint")]
    #[doc = "The `new UsbEndpoint(..)` constructor, creating a new instance of `UsbEndpoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBEndpoint/USBEndpoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbAlternateInterface`, `UsbDirection`, `UsbEndpoint`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        alternate: &UsbAlternateInterface,
        endpoint_number: u8,
        direction: UsbDirection,
    ) -> Result<UsbEndpoint, JsValue>;
}
