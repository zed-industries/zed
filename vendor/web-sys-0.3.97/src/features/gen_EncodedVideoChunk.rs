#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "EncodedVideoChunk",
        typescript_type = "EncodedVideoChunk"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedVideoChunk` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedVideoChunk;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkType")]
    #[wasm_bindgen(method, getter, js_class = "EncodedVideoChunk", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`, `EncodedVideoChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &EncodedVideoChunk) -> EncodedVideoChunkType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "EncodedVideoChunk", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(this: &EncodedVideoChunk) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "EncodedVideoChunk", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(this: &EncodedVideoChunk) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "EncodedVideoChunk", js_name = "byteLength")]
    #[doc = "Getter for the `byteLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/byteLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn byte_length(this: &EncodedVideoChunk) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkInit")]
    #[wasm_bindgen(catch, constructor, js_class = "EncodedVideoChunk")]
    #[doc = "The `new EncodedVideoChunk(..)` constructor, creating a new instance of `EncodedVideoChunk`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/EncodedVideoChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`, `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &EncodedVideoChunkInit) -> Result<EncodedVideoChunk, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "EncodedVideoChunk", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_buffer_source(
        this: &EncodedVideoChunk,
        destination: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "EncodedVideoChunk", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_u8_slice(
        this: &EncodedVideoChunk,
        destination: &mut [u8],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "EncodedVideoChunk", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_u8_array(
        this: &EncodedVideoChunk,
        destination: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
}
