#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TransformStreamDefaultController",
        typescript_type = "TransformStreamDefaultController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransformStreamDefaultController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub type TransformStreamDefaultController;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TransformStreamDefaultController",
        js_name = "desiredSize"
    )]
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub fn desired_size(this: &TransformStreamDefaultController) -> Option<f64>;
    #[wasm_bindgen(catch, method, js_class = "TransformStreamDefaultController")]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub fn enqueue(this: &TransformStreamDefaultController) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "TransformStreamDefaultController",
        js_name = "enqueue"
    )]
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub fn enqueue_with_chunk(
        this: &TransformStreamDefaultController,
        chunk: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "TransformStreamDefaultController")]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub fn error(this: &TransformStreamDefaultController);
    #[wasm_bindgen(
        method,
        js_class = "TransformStreamDefaultController",
        js_name = "error"
    )]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub fn error_with_reason(
        this: &TransformStreamDefaultController,
        reason: &::wasm_bindgen::JsValue,
    );
    #[wasm_bindgen(method, js_class = "TransformStreamDefaultController")]
    #[doc = "The `terminate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/terminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
    pub fn terminate(this: &TransformStreamDefaultController);
}
