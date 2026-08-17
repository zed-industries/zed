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
        js_name = "HIDInputReportEvent",
        typescript_type = "HIDInputReportEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidInputReportEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidInputReportEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    #[wasm_bindgen(method, getter, js_class = "HIDInputReportEvent", js_name = "device")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`, `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(this: &HidInputReportEvent) -> HidDevice;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDInputReportEvent", js_name = "reportId")]
    #[doc = "Getter for the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/reportId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn report_id(this: &HidInputReportEvent) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "HIDInputReportEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data(this: &HidInputReportEvent) -> ::js_sys::DataView;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidInputReportEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "HIDInputReportEvent")]
    #[doc = "The `new HidInputReportEvent(..)` constructor, creating a new instance of `HidInputReportEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/HIDInputReportEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`, `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &HidInputReportEventInit,
    ) -> Result<HidInputReportEvent, JsValue>;
}
