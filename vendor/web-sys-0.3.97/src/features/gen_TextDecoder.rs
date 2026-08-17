#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TextDecoder",
        typescript_type = "TextDecoder"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub type TextDecoder;
    #[wasm_bindgen(method, getter, js_class = "TextDecoder", js_name = "encoding")]
    #[doc = "Getter for the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/encoding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn encoding(this: &TextDecoder) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "TextDecoder", js_name = "fatal")]
    #[doc = "Getter for the `fatal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/fatal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn fatal(this: &TextDecoder) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "TextDecoder")]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn new() -> Result<TextDecoder, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "TextDecoder")]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn new_with_label(label: &str) -> Result<TextDecoder, JsValue>;
    #[cfg(feature = "TextDecoderOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "TextDecoder")]
    #[doc = "The `new TextDecoder(..)` constructor, creating a new instance of `TextDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/TextDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`, `TextDecoderOptions`*"]
    pub fn new_with_label_and_options(
        label: &str,
        options: &TextDecoderOptions,
    ) -> Result<TextDecoder, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TextDecoder")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn decode(this: &TextDecoder) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TextDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn decode_with_buffer_source(
        this: &TextDecoder,
        input: &::js_sys::Object,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TextDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn decode_with_u8_array(
        this: &TextDecoder,
        input: &[u8],
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TextDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecoder`*"]
    pub fn decode_with_js_u8_array(
        this: &TextDecoder,
        input: &::js_sys::Uint8Array,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "TextDecodeOptions")]
    #[wasm_bindgen(catch, method, js_class = "TextDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
    pub fn decode_with_buffer_source_and_options(
        this: &TextDecoder,
        input: &::js_sys::Object,
        options: &TextDecodeOptions,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "TextDecodeOptions")]
    #[wasm_bindgen(catch, method, js_class = "TextDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
    pub fn decode_with_u8_array_and_options(
        this: &TextDecoder,
        input: &[u8],
        options: &TextDecodeOptions,
    ) -> Result<::alloc::string::String, JsValue>;
    #[cfg(feature = "TextDecodeOptions")]
    #[wasm_bindgen(catch, method, js_class = "TextDecoder", js_name = "decode")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`, `TextDecoder`*"]
    pub fn decode_with_js_u8_array_and_options(
        this: &TextDecoder,
        input: &::js_sys::Uint8Array,
        options: &TextDecodeOptions,
    ) -> Result<::alloc::string::String, JsValue>;
}
