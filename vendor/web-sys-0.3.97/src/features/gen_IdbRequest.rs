#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "IDBRequest",
        typescript_type = "IDBRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub type IdbRequest;
    #[wasm_bindgen(catch, method, getter, js_class = "IDBRequest", js_name = "result")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub fn result(this: &IdbRequest) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(catch, method, getter, js_class = "IDBRequest", js_name = "error")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `IdbRequest`*"]
    pub fn error(this: &IdbRequest) -> Result<Option<DomException>, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "IDBRequest", js_name = "source")]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub fn source(this: &IdbRequest) -> Option<::js_sys::Object>;
    #[cfg(feature = "IdbTransaction")]
    #[wasm_bindgen(method, getter, js_class = "IDBRequest", js_name = "transaction")]
    #[doc = "Getter for the `transaction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`, `IdbTransaction`*"]
    pub fn transaction(this: &IdbRequest) -> Option<IdbTransaction>;
    #[cfg(feature = "IdbRequestReadyState")]
    #[wasm_bindgen(method, getter, js_class = "IDBRequest", js_name = "readyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`, `IdbRequestReadyState`*"]
    pub fn ready_state(this: &IdbRequest) -> IdbRequestReadyState;
    #[wasm_bindgen(method, getter, js_class = "IDBRequest", js_name = "onsuccess")]
    #[doc = "Getter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub fn onsuccess(this: &IdbRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBRequest", js_name = "onsuccess")]
    #[doc = "Setter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub fn set_onsuccess(this: &IdbRequest, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBRequest", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub fn onerror(this: &IdbRequest) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBRequest", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
    pub fn set_onerror(this: &IdbRequest, value: Option<&::js_sys::Function>);
}
