#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Blob",
        typescript_type = "Blob"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Blob` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub type Blob;
    #[wasm_bindgen(method, getter, js_class = "Blob", js_name = "size")]
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn size(this: &Blob) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Blob", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn type_(this: &Blob) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn new() -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn new_with_buffer_source_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn new_with_u8_slice_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn new_with_u8_array_sequence(
        blob_parts: &::wasm_bindgen::JsValue,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn new_with_blob_sequence(blob_parts: &::wasm_bindgen::JsValue) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn new_with_str_sequence(blob_parts: &::wasm_bindgen::JsValue) -> Result<Blob, JsValue>;
    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    pub fn new_with_buffer_source_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;
    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    pub fn new_with_u8_slice_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;
    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    pub fn new_with_u8_array_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;
    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    pub fn new_with_blob_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;
    #[cfg(feature = "BlobPropertyBag")]
    #[wasm_bindgen(catch, constructor, js_class = "Blob")]
    #[doc = "The `new Blob(..)` constructor, creating a new instance of `Blob`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/Blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `BlobPropertyBag`*"]
    pub fn new_with_str_sequence_and_options(
        blob_parts: &::wasm_bindgen::JsValue,
        options: &BlobPropertyBag,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(method, js_class = "Blob", js_name = "arrayBuffer")]
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn array_buffer(this: &Blob) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "Blob")]
    #[doc = "The `bytes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/bytes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn bytes(this: &Blob) -> ::js_sys::Promise;
    #[wasm_bindgen(catch, method, js_class = "Blob")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice(this: &Blob) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_i32(this: &Blob, start: i32) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_f64(this: &Blob, start: f64) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_i32_and_i32(this: &Blob, start: i32, end: i32) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_f64_and_i32(this: &Blob, start: f64, end: i32) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_i32_and_f64(this: &Blob, start: i32, end: f64) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_f64_and_f64(this: &Blob, start: f64, end: f64) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_i32_and_i32_and_content_type(
        this: &Blob,
        start: i32,
        end: i32,
        content_type: &str,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_f64_and_i32_and_content_type(
        this: &Blob,
        start: f64,
        end: i32,
        content_type: &str,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_i32_and_f64_and_content_type(
        this: &Blob,
        start: i32,
        end: f64,
        content_type: &str,
    ) -> Result<Blob, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Blob", js_name = "slice")]
    #[doc = "The `slice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn slice_with_f64_and_f64_and_content_type(
        this: &Blob,
        start: f64,
        end: f64,
        content_type: &str,
    ) -> Result<Blob, JsValue>;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(method, js_class = "Blob")]
    #[doc = "The `stream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ReadableStream`*"]
    pub fn stream(this: &Blob) -> ReadableStream;
    #[wasm_bindgen(method, js_class = "Blob")]
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`*"]
    pub fn text(this: &Blob) -> ::js_sys::Promise;
}
