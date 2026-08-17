#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WritableStreamDefaultWriter",
        typescript_type = "WritableStreamDefaultWriter"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStreamDefaultWriter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub type WritableStreamDefaultWriter;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WritableStreamDefaultWriter",
        js_name = "closed"
    )]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn closed(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "WritableStreamDefaultWriter",
        js_name = "desiredSize"
    )]
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn desired_size(this: &WritableStreamDefaultWriter) -> Result<Option<f64>, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WritableStreamDefaultWriter",
        js_name = "ready"
    )]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn ready(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    #[cfg(feature = "WritableStream")]
    #[wasm_bindgen(catch, constructor, js_class = "WritableStreamDefaultWriter")]
    #[doc = "The `new WritableStreamDefaultWriter(..)` constructor, creating a new instance of `WritableStreamDefaultWriter`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/WritableStreamDefaultWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`, `WritableStreamDefaultWriter`*"]
    pub fn new(stream: &WritableStream) -> Result<WritableStreamDefaultWriter, JsValue>;
    #[wasm_bindgen(method, js_class = "WritableStreamDefaultWriter")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn abort(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "WritableStreamDefaultWriter", js_name = "abort")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn abort_with_reason(
        this: &WritableStreamDefaultWriter,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "WritableStreamDefaultWriter")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn close(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    #[wasm_bindgen(
        method,
        js_class = "WritableStreamDefaultWriter",
        js_name = "releaseLock"
    )]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn release_lock(this: &WritableStreamDefaultWriter);
    #[wasm_bindgen(method, js_class = "WritableStreamDefaultWriter")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn write(this: &WritableStreamDefaultWriter) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "WritableStreamDefaultWriter", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
    pub fn write_with_chunk(
        this: &WritableStreamDefaultWriter,
        chunk: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
}
