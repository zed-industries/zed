#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DecompressionStream",
        typescript_type = "DecompressionStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DecompressionStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecompressionStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type DecompressionStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, getter, js_class = "DecompressionStream", js_name = "readable")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecompressionStream`, `ReadableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn readable(this: &DecompressionStream) -> ReadableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, getter, js_class = "DecompressionStream", js_name = "writable")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecompressionStream`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn writable(this: &DecompressionStream) -> WritableStream;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CompressionFormat")]
    #[wasm_bindgen(catch, constructor, js_class = "DecompressionStream")]
    #[doc = "The `new DecompressionStream(..)` constructor, creating a new instance of `DecompressionStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/DecompressionStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompressionFormat`, `DecompressionStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: CompressionFormat) -> Result<DecompressionStream, JsValue>;
}
