#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "USBConnectionEvent",
        typescript_type = "USBConnectionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbConnectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConnectionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbConnectionEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDevice")]
    #[wasm_bindgen(method, getter, js_class = "USBConnectionEvent", js_name = "device")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConnectionEvent`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(this: &UsbConnectionEvent) -> UsbDevice;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbConnectionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "USBConnectionEvent")]
    #[doc = "The `new UsbConnectionEvent(..)` constructor, creating a new instance of `UsbConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBConnectionEvent/USBConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConnectionEvent`, `UsbConnectionEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &UsbConnectionEventInit,
    ) -> Result<UsbConnectionEvent, JsValue>;
}
