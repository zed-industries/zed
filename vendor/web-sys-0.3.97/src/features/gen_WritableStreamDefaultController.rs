#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "WritableStreamDefaultController",
        typescript_type = "WritableStreamDefaultController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStreamDefaultController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultController`*"]
    pub type WritableStreamDefaultController;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "WritableStreamDefaultController",
        js_name = "signal"
    )]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `WritableStreamDefaultController`*"]
    pub fn signal(this: &WritableStreamDefaultController) -> AbortSignal;
    #[wasm_bindgen(method, js_class = "WritableStreamDefaultController")]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultController`*"]
    pub fn error(this: &WritableStreamDefaultController);
    #[wasm_bindgen(
        method,
        js_class = "WritableStreamDefaultController",
        js_name = "error"
    )]
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultController`*"]
    pub fn error_with_e(this: &WritableStreamDefaultController, e: &::wasm_bindgen::JsValue);
}
