#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "WebTransportErrorOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportErrorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportErrorOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`, `WebTransportErrorSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &WebTransportErrorOptions) -> Option<WebTransportErrorSource>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`, `WebTransportErrorSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &WebTransportErrorOptions, val: WebTransportErrorSource);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "streamErrorCode")]
    pub fn get_stream_error_code(this: &WebTransportErrorOptions) -> Option<u8>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "streamErrorCode")]
    pub fn set_stream_error_code(this: &WebTransportErrorOptions, val: Option<u8>);
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportErrorOptions {
    #[doc = "Construct a new `WebTransportErrorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: WebTransportErrorSource) -> &mut Self {
        self.set_source(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_stream_error_code()` instead."]
    pub fn stream_error_code(&mut self, val: Option<u8>) -> &mut Self {
        self.set_stream_error_code(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportErrorOptions {
    fn default() -> Self {
        Self::new()
    }
}
