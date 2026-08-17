#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "RTCRtpScriptTransform",
        typescript_type = "RTCRtpScriptTransform"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpScriptTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type RtcRtpScriptTransform;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Worker")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCRtpScriptTransform")]
    #[doc = "The `new RtcRtpScriptTransform(..)` constructor, creating a new instance of `RtcRtpScriptTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `Worker`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(worker: &Worker) -> Result<RtcRtpScriptTransform, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Worker")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCRtpScriptTransform")]
    #[doc = "The `new RtcRtpScriptTransform(..)` constructor, creating a new instance of `RtcRtpScriptTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `Worker`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_options(
        worker: &Worker,
        options: &::wasm_bindgen::JsValue,
    ) -> Result<RtcRtpScriptTransform, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Worker")]
    #[wasm_bindgen(catch, constructor, js_class = "RTCRtpScriptTransform")]
    #[doc = "The `new RtcRtpScriptTransform(..)` constructor, creating a new instance of `RtcRtpScriptTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `Worker`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_options_and_transfer(
        worker: &Worker,
        options: &::wasm_bindgen::JsValue,
        transfer: &[::js_sys::Object],
    ) -> Result<RtcRtpScriptTransform, JsValue>;
}
