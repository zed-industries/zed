#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableByteStreamController",
        typescript_type = "ReadableByteStreamController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableByteStreamController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub type ReadableByteStreamController;
    #[cfg(feature = "ReadableStreamByobRequest")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ReadableByteStreamController",
        js_name = "byobRequest"
    )]
    #[doc = "Getter for the `byobRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/byobRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`, `ReadableStreamByobRequest`*"]
    pub fn byob_request(this: &ReadableByteStreamController) -> Option<ReadableStreamByobRequest>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ReadableByteStreamController",
        js_name = "desiredSize"
    )]
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn desired_size(this: &ReadableByteStreamController) -> Option<f64>;
    #[wasm_bindgen(catch, method, js_class = "ReadableByteStreamController")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn close(this: &ReadableByteStreamController) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableByteStreamController",
        js_name = "enqueue"
    )]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn enqueue_with_array_buffer_view(
        this: &ReadableByteStreamController,
        chunk: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableByteStreamController",
        js_name = "enqueue"
    )]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn enqueue_with_u8_array(
        this: &ReadableByteStreamController,
        chunk: &mut [u8],
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableByteStreamController",
        js_name = "enqueue"
    )]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn enqueue_with_js_u8_array(
        this: &ReadableByteStreamController,
        chunk: &::js_sys::Uint8Array,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "ReadableByteStreamController")]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn error(this: &ReadableByteStreamController);
    #[wasm_bindgen(method, js_class = "ReadableByteStreamController", js_name = "error")]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
    pub fn error_with_e(this: &ReadableByteStreamController, e: &::wasm_bindgen::JsValue);
}
