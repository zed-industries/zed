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
        js_name = "RTCTransformEvent",
        typescript_type = "RTCTransformEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTransformEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTransformEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransformEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RtcTransformEvent;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcRtpScriptTransformer")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "RTCTransformEvent",
        js_name = "transformer"
    )]
    #[doc = "Getter for the `transformer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTransformEvent/transformer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`, `RtcTransformEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transformer(this: &RtcTransformEvent) -> RtcRtpScriptTransformer;
}
