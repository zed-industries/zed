//! Raw bindings to JavaScript objects used
//! by a [`ReadableStream`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream).
//! These are re-exported from [web-sys](https://docs.rs/web-sys/0.3.70/web_sys/struct.ReadableStream.html).
use js_sys::{Array, Error, Object};
use wasm_bindgen::prelude::*;
pub use web_sys::ReadableByteStreamController;
// Re-export from web-sys
pub use web_sys::ReadableStream;
pub use web_sys::ReadableStreamByobReader as ReadableStreamBYOBReader;
pub use web_sys::ReadableStreamByobRequest as ReadableStreamBYOBRequest;
pub use web_sys::ReadableStreamDefaultController;
pub use web_sys::ReadableStreamDefaultReader;
pub use web_sys::ReadableStreamGetReaderOptions;
pub use web_sys::ReadableStreamReadResult;
pub use web_sys::ReadableStreamReaderMode;
pub use web_sys::ReadableStreamType;
pub use web_sys::StreamPipeOptions as PipeOptions;

use crate::queuing_strategy::sys::QueuingStrategy;
use crate::readable::into_underlying_byte_source::IntoUnderlyingByteSource;
use crate::readable::into_underlying_source::IntoUnderlyingSource;

#[wasm_bindgen]
extern "C" {
    /// Additional methods for [`ReadableStream`](web_sys::ReadableStream).
    #[wasm_bindgen(js_name = ReadableStream, typescript_type = "ReadableStream")]
    pub(crate) type ReadableStreamExt;

    #[wasm_bindgen(constructor, js_class = ReadableStream)]
    pub(crate) fn new_with_into_underlying_source(
        source: IntoUnderlyingSource,
        strategy: QueuingStrategy,
    ) -> ReadableStreamExt;

    #[wasm_bindgen(constructor, catch, js_class = ReadableStream)]
    pub(crate) fn new_with_into_underlying_byte_source(
        source: IntoUnderlyingByteSource,
    ) -> Result<ReadableStreamExt, Error>;

    #[wasm_bindgen(method, catch, js_class = ReadableStream, js_name = getReader)]
    pub(crate) fn try_get_reader(this: &ReadableStreamExt) -> Result<Object, Error>;

    #[wasm_bindgen(method, catch, js_class = ReadableStream, js_name = getReader)]
    pub(crate) fn try_get_reader_with_options(
        this: &ReadableStreamExt,
        options: &ReadableStreamGetReaderOptions,
    ) -> Result<Object, Error>;

    #[wasm_bindgen(method, catch, js_class = ReadableStream, js_name = tee)]
    pub(crate) fn try_tee(this: &ReadableStreamExt) -> Result<Array, Error>;

    #[wasm_bindgen(catch, static_method_of = ReadableStreamExt, js_class = ReadableStream, js_name = from)]
    pub(crate) fn from_async_iterable(async_iterable: &Object) -> Result<ReadableStreamExt, Error>;
}

#[wasm_bindgen]
extern "C" {
    /// Additional methods for [`ReadableStreamDefaultReader`](web_sys::ReadableStreamDefaultReader)
    /// and [`ReadableStreamByobReader`](web_sys::ReadableStreamByobReader).
    pub(crate) type ReadableStreamReaderExt;

    #[wasm_bindgen(method, catch, js_name = releaseLock)]
    pub(crate) fn try_release_lock(this: &ReadableStreamReaderExt) -> Result<(), Error>;
}
