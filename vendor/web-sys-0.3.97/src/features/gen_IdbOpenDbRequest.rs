#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "IdbRequest",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "IDBOpenDBRequest",
        typescript_type = "IDBOpenDBRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbOpenDbRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub type IdbOpenDbRequest;
    #[wasm_bindgen(method, getter, js_class = "IDBOpenDBRequest", js_name = "onblocked")]
    #[doc = "Getter for the `onblocked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn onblocked(this: &IdbOpenDbRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBOpenDBRequest", js_name = "onblocked")]
    #[doc = "Setter for the `onblocked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn set_onblocked(this: &IdbOpenDbRequest, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IDBOpenDBRequest",
        js_name = "onupgradeneeded"
    )]
    #[doc = "Getter for the `onupgradeneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn onupgradeneeded(this: &IdbOpenDbRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "IDBOpenDBRequest",
        js_name = "onupgradeneeded"
    )]
    #[doc = "Setter for the `onupgradeneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbRequest`*"]
    pub fn set_onupgradeneeded(this: &IdbOpenDbRequest, value: Option<&::js_sys::Function>);
}
