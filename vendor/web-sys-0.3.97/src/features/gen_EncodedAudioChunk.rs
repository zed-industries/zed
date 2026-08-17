#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "EncodedAudioChunk",
        typescript_type = "EncodedAudioChunk"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedAudioChunk` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type EncodedAudioChunk;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkType")]
    #[wasm_bindgen(method, getter, js_class = "EncodedAudioChunk", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`, `EncodedAudioChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(this: &EncodedAudioChunk) -> EncodedAudioChunkType;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "EncodedAudioChunk", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(this: &EncodedAudioChunk) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "EncodedAudioChunk", js_name = "duration")]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(this: &EncodedAudioChunk) -> Option<f64>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "EncodedAudioChunk", js_name = "byteLength")]
    #[doc = "Getter for the `byteLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/byteLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn byte_length(this: &EncodedAudioChunk) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkInit")]
    #[wasm_bindgen(catch, constructor, js_class = "EncodedAudioChunk")]
    #[doc = "The `new EncodedAudioChunk(..)` constructor, creating a new instance of `EncodedAudioChunk`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/EncodedAudioChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`, `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(init: &EncodedAudioChunkInit) -> Result<EncodedAudioChunk, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "EncodedAudioChunk", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_buffer_source(
        this: &EncodedAudioChunk,
        destination: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "EncodedAudioChunk", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_u8_slice(
        this: &EncodedAudioChunk,
        destination: &mut [u8],
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, method, js_class = "EncodedAudioChunk", js_name = "copyTo")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn copy_to_with_u8_array(
        this: &EncodedAudioChunk,
        destination: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
}
