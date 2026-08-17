#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaCapabilities",
        typescript_type = "MediaCapabilities"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaCapabilities` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`*"]
    pub type MediaCapabilities;
    #[cfg(feature = "MediaDecodingConfiguration")]
    #[wasm_bindgen(method, js_class = "MediaCapabilities", js_name = "decodingInfo")]
    #[doc = "The `decodingInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/decodingInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaDecodingConfiguration`*"]
    pub fn decoding_info(
        this: &MediaCapabilities,
        configuration: &MediaDecodingConfiguration,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "MediaEncodingConfiguration")]
    #[wasm_bindgen(method, js_class = "MediaCapabilities", js_name = "encodingInfo")]
    #[doc = "The `encodingInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/encodingInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaEncodingConfiguration`*"]
    pub fn encoding_info(
        this: &MediaCapabilities,
        configuration: &MediaEncodingConfiguration,
    ) -> ::js_sys::Promise;
}
