#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableStreamBYOBReader",
        typescript_type = "ReadableStreamBYOBReader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamByobReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
    pub type ReadableStreamByobReader;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ReadableStreamBYOBReader",
        js_name = "closed"
    )]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
    pub fn closed(this: &ReadableStreamByobReader) -> ::js_sys::Promise;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor, js_class = "ReadableStreamBYOBReader")]
    #[doc = "The `new ReadableStreamByobReader(..)` constructor, creating a new instance of `ReadableStreamByobReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/ReadableStreamBYOBReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamByobReader`*"]
    pub fn new(stream: &ReadableStream) -> Result<ReadableStreamByobReader, JsValue>;
    #[wasm_bindgen(method, js_class = "ReadableStreamBYOBReader", js_name = "read")]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
    pub fn read_with_array_buffer_view(
        this: &ReadableStreamByobReader,
        view: &::js_sys::Object,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "ReadableStreamBYOBReader", js_name = "releaseLock")]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
    pub fn release_lock(this: &ReadableStreamByobReader);
    #[wasm_bindgen(method, js_class = "ReadableStreamBYOBReader")]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
    pub fn cancel(this: &ReadableStreamByobReader) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "ReadableStreamBYOBReader", js_name = "cancel")]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
    pub fn cancel_with_reason(
        this: &ReadableStreamByobReader,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
}
