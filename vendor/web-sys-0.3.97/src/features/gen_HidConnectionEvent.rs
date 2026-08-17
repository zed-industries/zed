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
        js_name = "HIDConnectionEvent",
        typescript_type = "HIDConnectionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidConnectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidConnectionEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidConnectionEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    #[wasm_bindgen(method, getter, js_class = "HIDConnectionEvent", js_name = "device")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidConnectionEvent`, `HidDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(this: &HidConnectionEvent) -> HidDevice;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidConnectionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "HIDConnectionEvent")]
    #[doc = "The `new HidConnectionEvent(..)` constructor, creating a new instance of `HidConnectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent/HIDConnectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidConnectionEvent`, `HidConnectionEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &HidConnectionEventInit,
    ) -> Result<HidConnectionEvent, JsValue>;
}
