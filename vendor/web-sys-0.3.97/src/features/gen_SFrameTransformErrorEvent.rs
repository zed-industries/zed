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
        js_name = "SFrameTransformErrorEvent",
        typescript_type = "SFrameTransformErrorEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransformErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SFrameTransformErrorEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventType")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SFrameTransformErrorEvent",
        js_name = "errorType"
    )]
    #[doc = "Getter for the `errorType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/errorType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`, `SFrameTransformErrorEventType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn error_type(this: &SFrameTransformErrorEvent) -> SFrameTransformErrorEventType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SFrameTransformErrorEvent",
        js_name = "keyID"
    )]
    #[doc = "Getter for the `keyID` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/keyID)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn key_id(this: &SFrameTransformErrorEvent) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SFrameTransformErrorEvent",
        js_name = "frame"
    )]
    #[doc = "Getter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn frame(this: &SFrameTransformErrorEvent) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "SFrameTransformErrorEvent")]
    #[doc = "The `new SFrameTransformErrorEvent(..)` constructor, creating a new instance of `SFrameTransformErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransformErrorEvent/SFrameTransformErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransformErrorEvent`, `SFrameTransformErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        type_: &str,
        event_init_dict: &SFrameTransformErrorEventInit,
    ) -> Result<SFrameTransformErrorEvent, JsValue>;
}
