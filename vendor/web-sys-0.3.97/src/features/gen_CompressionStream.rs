#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "CompressionStream",
        typescript_type = "CompressionStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CompressionStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompressionStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type CompressionStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, getter, js_class = "CompressionStream", js_name = "readable")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompressionStream`, `ReadableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn readable(this: &CompressionStream) -> ReadableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, getter, js_class = "CompressionStream", js_name = "writable")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompressionStream`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable(this: &CompressionStream) -> WritableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CompressionFormat")]
    #[wasm_bindgen(catch, constructor, js_class = "CompressionStream")]
    #[doc = "The `new CompressionStream(..)` constructor, creating a new instance of `CompressionStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream/CompressionStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompressionFormat`, `CompressionStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: CompressionFormat) -> Result<CompressionStream, JsValue>;
}
