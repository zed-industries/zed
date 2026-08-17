#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableStream",
        typescript_type = "ReadableStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub type ReadableStream;
    #[wasm_bindgen(method, getter, js_class = "ReadableStream", js_name = "locked")]
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn locked(this: &ReadableStream) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "ReadableStream")]
    #[doc = "The `new ReadableStream(..)` constructor, creating a new instance of `ReadableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn new() -> Result<ReadableStream, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "ReadableStream")]
    #[doc = "The `new ReadableStream(..)` constructor, creating a new instance of `ReadableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn new_with_underlying_source(
        underlying_source: &::js_sys::Object,
    ) -> Result<ReadableStream, JsValue>;
    #[cfg(feature = "QueuingStrategy")]
    #[wasm_bindgen(catch, constructor, js_class = "ReadableStream")]
    #[doc = "The `new ReadableStream(..)` constructor, creating a new instance of `ReadableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `ReadableStream`*"]
    pub fn new_with_underlying_source_and_strategy(
        underlying_source: &::js_sys::Object,
        strategy: &QueuingStrategy,
    ) -> Result<ReadableStream, JsValue>;
    #[wasm_bindgen(method, js_class = "ReadableStream")]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn cancel(this: &ReadableStream) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "cancel")]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn cancel_with_reason(
        this: &ReadableStream,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "getReader")]
    #[doc = "The `getReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn get_reader(this: &ReadableStream) -> ::js_sys::Object;
    #[cfg(feature = "ReadableStreamGetReaderOptions")]
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "getReader")]
    #[doc = "The `getReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamGetReaderOptions`*"]
    pub fn get_reader_with_options(
        this: &ReadableStream,
        options: &ReadableStreamGetReaderOptions,
    ) -> ::js_sys::Object;
    #[cfg(feature = "ReadableWritablePair")]
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "pipeThrough")]
    #[doc = "The `pipeThrough()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    pub fn pipe_through(this: &ReadableStream, transform: &ReadableWritablePair) -> ReadableStream;
    #[cfg(all(feature = "ReadableWritablePair", feature = "StreamPipeOptions",))]
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "pipeThrough")]
    #[doc = "The `pipeThrough()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `StreamPipeOptions`*"]
    pub fn pipe_through_with_options(
        this: &ReadableStream,
        transform: &ReadableWritablePair,
        options: &StreamPipeOptions,
    ) -> ReadableStream;
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "pipeTo")]
    #[doc = "The `pipeTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `WritableStream`*"]
    pub fn pipe_to(this: &ReadableStream, destination: &WritableStream) -> ::js_sys::Promise;
    #[cfg(all(feature = "StreamPipeOptions", feature = "WritableStream",))]
    #[wasm_bindgen(method, js_class = "ReadableStream", js_name = "pipeTo")]
    #[doc = "The `pipeTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `StreamPipeOptions`, `WritableStream`*"]
    pub fn pipe_to_with_options(
        this: &ReadableStream,
        destination: &WritableStream,
        options: &StreamPipeOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "ReadableStream")]
    #[doc = "The `tee()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/tee)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn tee(this: &ReadableStream) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "ReadableStream")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn entries(this: &ReadableStream) -> ::js_sys::AsyncIterator;
    #[wasm_bindgen(method, js_class = "ReadableStream")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn keys(this: &ReadableStream) -> ::js_sys::AsyncIterator;
    #[wasm_bindgen(method, js_class = "ReadableStream")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
    pub fn values(this: &ReadableStream) -> ::js_sys::AsyncIterator;
}
