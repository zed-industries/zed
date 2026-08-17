#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "IDBTransaction",
        typescript_type = "IDBTransaction"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbTransaction` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub type IdbTransaction;
    #[cfg(feature = "DomStringList")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "IDBTransaction",
        js_name = "objectStoreNames"
    )]
    #[doc = "Getter for the `objectStoreNames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`, `IdbTransaction`*"]
    pub fn object_store_names(this: &IdbTransaction) -> DomStringList;
    #[cfg(feature = "IdbTransactionMode")]
    #[wasm_bindgen(catch, method, getter, js_class = "IDBTransaction", js_name = "mode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`, `IdbTransactionMode`*"]
    pub fn mode(this: &IdbTransaction) -> Result<IdbTransactionMode, JsValue>;
    #[cfg(feature = "IdbDatabase")]
    #[wasm_bindgen(method, getter, js_class = "IDBTransaction", js_name = "db")]
    #[doc = "Getter for the `db` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    pub fn db(this: &IdbTransaction) -> IdbDatabase;
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(method, getter, js_class = "IDBTransaction", js_name = "error")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `IdbTransaction`*"]
    pub fn error(this: &IdbTransaction) -> Option<DomException>;
    #[wasm_bindgen(method, getter, js_class = "IDBTransaction", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn onabort(this: &IdbTransaction) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBTransaction", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn set_onabort(this: &IdbTransaction, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBTransaction", js_name = "oncomplete")]
    #[doc = "Getter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn oncomplete(this: &IdbTransaction) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBTransaction", js_name = "oncomplete")]
    #[doc = "Setter for the `oncomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn set_oncomplete(this: &IdbTransaction, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBTransaction", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn onerror(this: &IdbTransaction) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBTransaction", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn set_onerror(this: &IdbTransaction, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "IDBTransaction")]
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    pub fn abort(this: &IdbTransaction) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBTransaction")]
    #[doc = "The `commit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/commit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[deprecated]
    pub fn commit(this: &IdbTransaction) -> Result<(), JsValue>;
    #[cfg(feature = "IdbObjectStore")]
    #[wasm_bindgen(catch, method, js_class = "IDBTransaction", js_name = "objectStore")]
    #[doc = "The `objectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
    pub fn object_store(this: &IdbTransaction, name: &str) -> Result<IdbObjectStore, JsValue>;
}
