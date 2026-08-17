#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "ReadableStreamDefaultController",
        typescript_type = "ReadableStreamDefaultController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub type ReadableStreamDefaultController;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "ReadableStreamDefaultController",
        js_name = "desiredSize"
    )]
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub fn desired_size(this: &ReadableStreamDefaultController) -> Option<f64>;
    #[wasm_bindgen(catch, method, js_class = "ReadableStreamDefaultController")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub fn close(this: &ReadableStreamDefaultController) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ReadableStreamDefaultController")]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub fn enqueue(this: &ReadableStreamDefaultController) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "ReadableStreamDefaultController",
        js_name = "enqueue"
    )]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub fn enqueue_with_chunk(
        this: &ReadableStreamDefaultController,
        chunk: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "ReadableStreamDefaultController")]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub fn error(this: &ReadableStreamDefaultController);
    #[wasm_bindgen(
        method,
        js_class = "ReadableStreamDefaultController",
        js_name = "error"
    )]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
    pub fn error_with_e(this: &ReadableStreamDefaultController, e: &::wasm_bindgen::JsValue);
}
