#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WebTransportDatagramDuplexStream",
        typescript_type = "WebTransportDatagramDuplexStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportDatagramDuplexStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportDatagramDuplexStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "readable"
    )]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn readable(this: &WebTransportDatagramDuplexStream) -> ReadableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "writable"
    )]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable(this: &WebTransportDatagramDuplexStream) -> WritableStream;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "maxDatagramSize"
    )]
    #[doc = "Getter for the `maxDatagramSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/maxDatagramSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_datagram_size(this: &WebTransportDatagramDuplexStream) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "incomingMaxAge"
    )]
    #[doc = "Getter for the `incomingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn incoming_max_age(this: &WebTransportDatagramDuplexStream) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "incomingMaxAge"
    )]
    #[doc = "Setter for the `incomingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_incoming_max_age(this: &WebTransportDatagramDuplexStream, value: f64);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "outgoingMaxAge"
    )]
    #[doc = "Getter for the `outgoingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn outgoing_max_age(this: &WebTransportDatagramDuplexStream) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "outgoingMaxAge"
    )]
    #[doc = "Setter for the `outgoingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_outgoing_max_age(this: &WebTransportDatagramDuplexStream, value: f64);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "incomingHighWaterMark"
    )]
    #[doc = "Getter for the `incomingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn incoming_high_water_mark(this: &WebTransportDatagramDuplexStream) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "incomingHighWaterMark"
    )]
    #[doc = "Setter for the `incomingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_incoming_high_water_mark(this: &WebTransportDatagramDuplexStream, value: f64);
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "outgoingHighWaterMark"
    )]
    #[doc = "Getter for the `outgoingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn outgoing_high_water_mark(this: &WebTransportDatagramDuplexStream) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        setter,
        js_class = "WebTransportDatagramDuplexStream",
        js_name = "outgoingHighWaterMark"
    )]
    #[doc = "Setter for the `outgoingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn set_outgoing_high_water_mark(this: &WebTransportDatagramDuplexStream, value: f64);
}
