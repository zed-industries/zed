#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "DOMRequest",
        typescript_type = "DOMRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub type DomRequest;
    #[cfg(feature = "DomRequestReadyState")]
    #[wasm_bindgen(method, getter, js_class = "DOMRequest", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`, `DomRequestReadyState`*"]
    pub fn ready_state(this: &DomRequest) -> DomRequestReadyState;
    #[wasm_bindgen(method, getter, js_class = "DOMRequest", js_name = "result")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn result(this: &DomRequest) -> ::wasm_bindgen::JsValue;
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(method, getter, js_class = "DOMRequest", js_name = "error")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `DomRequest`*"]
    pub fn error(this: &DomRequest) -> Option<DomException>;
    #[wasm_bindgen(method, getter, js_class = "DOMRequest", js_name = "onsuccess")]
    #[doc = "Getter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn onsuccess(this: &DomRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "DOMRequest", js_name = "onsuccess")]
    #[doc = "Setter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn set_onsuccess(this: &DomRequest, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "DOMRequest", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn onerror(this: &DomRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "DOMRequest", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn set_onerror(this: &DomRequest, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "DOMRequest")]
    #[doc = "The `then()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn then(this: &DomRequest) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMRequest", js_name = "then")]
    #[doc = "The `then()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn then_with_fulfill_callback(
        this: &DomRequest,
        fulfill_callback: Option<&::js_sys::Function>,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DOMRequest", js_name = "then")]
    #[doc = "The `then()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
    pub fn then_with_fulfill_callback_and_reject_callback(
        this: &DomRequest,
        fulfill_callback: Option<&::js_sys::Function>,
        reject_callback: Option<&::js_sys::Function>,
    ) -> Result<::wasm_bindgen::JsValue, JsValue>;
}
