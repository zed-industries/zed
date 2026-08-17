#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBIsochronousOutTransferResult",
        typescript_type = "USBIsochronousOutTransferResult"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbIsochronousOutTransferResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbIsochronousOutTransferResult;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbIsochronousOutTransferPacket")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBIsochronousOutTransferResult",
        js_name = "packets"
    )]
    #[doc = "Getter for the `packets` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult/packets)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`, `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn packets(
        this: &UsbIsochronousOutTransferResult,
    ) -> ::js_sys::Array<UsbIsochronousOutTransferPacket>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbIsochronousOutTransferPacket")]
    #[wasm_bindgen(catch, constructor, js_class = "USBIsochronousOutTransferResult")]
    #[doc = "The `new UsbIsochronousOutTransferResult(..)` constructor, creating a new instance of `UsbIsochronousOutTransferResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBIsochronousOutTransferResult/USBIsochronousOutTransferResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbIsochronousOutTransferPacket`, `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        packets: &[UsbIsochronousOutTransferPacket],
    ) -> Result<UsbIsochronousOutTransferResult, JsValue>;
}
