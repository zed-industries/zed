#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "PermissionStatus",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "USBPermissionResult",
        typescript_type = "USBPermissionResult"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbPermissionResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBPermissionResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbPermissionResult;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDevice")]
    #[wasm_bindgen(method, getter, js_class = "USBPermissionResult", js_name = "devices")]
    #[doc = "Getter for the `devices` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBPermissionResult/devices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbPermissionResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn devices(this: &UsbPermissionResult) -> ::js_sys::Array<UsbDevice>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDevice")]
    #[wasm_bindgen(method, setter, js_class = "USBPermissionResult", js_name = "devices")]
    #[doc = "Setter for the `devices` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBPermissionResult/devices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbPermissionResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_devices(this: &UsbPermissionResult, value: &[UsbDevice]);
}
