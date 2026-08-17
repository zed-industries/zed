#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "DomException",
        extends = "::js_sys::Object",
        js_name = "WebTransportError",
        typescript_type = "WebTransportError"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportError;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[wasm_bindgen(method, getter, js_class = "WebTransportError", js_name = "source")]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`, `WebTransportErrorSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn source(this: &WebTransportError) -> WebTransportErrorSource;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WebTransportError",
        js_name = "streamErrorCode"
    )]
    #[doc = "Getter for the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/streamErrorCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stream_error_code(this: &WebTransportError) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Result<WebTransportError, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_message(message: &str) -> Result<WebTransportError, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`, `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_message_and_options(
        message: &str,
        options: &WebTransportErrorOptions,
    ) -> Result<WebTransportError, JsValue>;
}
