#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebTransportBidirectionalStream",
        typescript_type = "WebTransportBidirectionalStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportBidirectionalStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportBidirectionalStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportReceiveStream")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportBidirectionalStream",
        js_name = "readable"
    )]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`, `WebTransportReceiveStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn readable(this: &WebTransportBidirectionalStream) -> WebTransportReceiveStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportSendStream")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportBidirectionalStream",
        js_name = "writable"
    )]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`, `WebTransportSendStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable(this: &WebTransportBidirectionalStream) -> WebTransportSendStream;
}
