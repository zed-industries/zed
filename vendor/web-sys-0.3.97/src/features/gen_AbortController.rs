#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "AbortController",
        typescript_type = "AbortController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AbortController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
    pub type AbortController;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter, js_class = "AbortController", js_name = "signal")]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`, `AbortSignal`*"]
    pub fn signal(this: &AbortController) -> AbortSignal;
    #[wasm_bindgen(catch, constructor, js_class = "AbortController")]
    #[doc = "The `new AbortController(..)` constructor, creating a new instance of `AbortController`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/AbortController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
    pub fn new() -> Result<AbortController, JsValue>;
    #[wasm_bindgen(method, js_class = "AbortController")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
    pub fn abort(this: &AbortController);
    #[wasm_bindgen(method, js_class = "AbortController", js_name = "abort")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
    pub fn abort_with_reason(this: &AbortController, reason: &::wasm_bindgen::JsValue);
}
