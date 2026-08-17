#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "AbortSignal",
        typescript_type = "AbortSignal"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AbortSignal` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub type AbortSignal;
    #[wasm_bindgen(method, getter, js_class = "AbortSignal", js_name = "aborted")]
    #[doc = "Getter for the `aborted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/aborted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn aborted(this: &AbortSignal) -> bool;
    #[wasm_bindgen(method, getter, js_class = "AbortSignal", js_name = "reason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn reason(this: &AbortSignal) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, getter, js_class = "AbortSignal", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn onabort(this: &AbortSignal) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "AbortSignal", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn set_onabort(this: &AbortSignal, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(static_method_of = "AbortSignal", js_class = "AbortSignal")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/abort_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn abort() -> AbortSignal;
    #[wasm_bindgen(
        static_method_of = "AbortSignal",
        js_class = "AbortSignal",
        js_name = "abort"
    )]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/abort_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn abort_with_reason(reason: &::wasm_bindgen::JsValue) -> AbortSignal;
    #[wasm_bindgen(static_method_of = "AbortSignal", js_class = "AbortSignal")]
    #[doc = "The `any()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/any_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn any(signals: &::wasm_bindgen::JsValue) -> AbortSignal;
    #[wasm_bindgen(method, js_class = "AbortSignal", js_name = "throwIfAborted")]
    #[doc = "The `throwIfAborted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/throwIfAborted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn throw_if_aborted(this: &AbortSignal);
    #[wasm_bindgen(
        static_method_of = "AbortSignal",
        js_class = "AbortSignal",
        js_name = "timeout"
    )]
    #[doc = "The `timeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn timeout_with_u32(milliseconds: u32) -> AbortSignal;
    #[wasm_bindgen(
        static_method_of = "AbortSignal",
        js_class = "AbortSignal",
        js_name = "timeout"
    )]
    #[doc = "The `timeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`*"]
    pub fn timeout_with_f64(milliseconds: f64) -> AbortSignal;
}
