#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WritableStream",
        typescript_type = "WritableStream"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub type WritableStream;
    #[wasm_bindgen(method, getter, js_class = "WritableStream", js_name = "locked")]
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn locked(this: &WritableStream) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "WritableStream")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn new() -> Result<WritableStream, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WritableStream")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn new_with_underlying_sink(
        underlying_sink: &::js_sys::Object,
    ) -> Result<WritableStream, JsValue>;
    #[cfg(feature = "QueuingStrategy")]
    #[wasm_bindgen(catch, constructor, js_class = "WritableStream")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `WritableStream`*"]
    pub fn new_with_underlying_sink_and_strategy(
        underlying_sink: &::js_sys::Object,
        strategy: &QueuingStrategy,
    ) -> Result<WritableStream, JsValue>;
    #[wasm_bindgen(method, js_class = "WritableStream")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn abort(this: &WritableStream) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "WritableStream", js_name = "abort")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn abort_with_reason(
        this: &WritableStream,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "WritableStream")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
    pub fn close(this: &WritableStream) -> ::js_sys::Promise;
    #[cfg(feature = "WritableStreamDefaultWriter")]
    #[wasm_bindgen(catch, method, js_class = "WritableStream", js_name = "getWriter")]
    #[doc = "The `getWriter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/getWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`, `WritableStreamDefaultWriter`*"]
    pub fn get_writer(this: &WritableStream) -> Result<WritableStreamDefaultWriter, JsValue>;
}
