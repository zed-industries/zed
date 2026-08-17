#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "IDBDatabase",
        typescript_type = "IDBDatabase"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbDatabase` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub type IdbDatabase;
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn name(this: &IdbDatabase) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "version")]
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn version(this: &IdbDatabase) -> f64;
    #[cfg(feature = "DomStringList")]
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "objectStoreNames")]
    #[doc = "Getter for the `objectStoreNames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`, `IdbDatabase`*"]
    pub fn object_store_names(this: &IdbDatabase) -> DomStringList;
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "onabort")]
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onabort(this: &IdbDatabase) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBDatabase", js_name = "onabort")]
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onabort(this: &IdbDatabase, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onclose(this: &IdbDatabase) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBDatabase", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onclose(this: &IdbDatabase, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onerror(this: &IdbDatabase) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBDatabase", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onerror(this: &IdbDatabase, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "onversionchange")]
    #[doc = "Getter for the `onversionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn onversionchange(this: &IdbDatabase) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "IDBDatabase", js_name = "onversionchange")]
    #[doc = "Setter for the `onversionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn set_onversionchange(this: &IdbDatabase, value: Option<&::js_sys::Function>);
    #[cfg(feature = "StorageType")]
    #[wasm_bindgen(method, getter, js_class = "IDBDatabase", js_name = "storage")]
    #[doc = "Getter for the `storage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/storage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `StorageType`*"]
    #[deprecated]
    pub fn storage(this: &IdbDatabase) -> StorageType;
    #[wasm_bindgen(method, js_class = "IDBDatabase")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn close(this: &IdbDatabase);
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "createMutableFile")]
    #[doc = "The `createMutableFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    #[deprecated]
    pub fn create_mutable_file(this: &IdbDatabase, name: &str) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "createMutableFile")]
    #[doc = "The `createMutableFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    #[deprecated]
    pub fn create_mutable_file_with_type(
        this: &IdbDatabase,
        name: &str,
        type_: &str,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbObjectStore")]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "createObjectStore")]
    #[doc = "The `createObjectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`*"]
    pub fn create_object_store(this: &IdbDatabase, name: &str) -> Result<IdbObjectStore, JsValue>;
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbObjectStoreParameters",))]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "createObjectStore")]
    #[doc = "The `createObjectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`, `IdbObjectStoreParameters`*"]
    pub fn create_object_store_with_optional_parameters(
        this: &IdbDatabase,
        name: &str,
        options: &IdbObjectStoreParameters,
    ) -> Result<IdbObjectStore, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "deleteObjectStore")]
    #[doc = "The `deleteObjectStore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`*"]
    pub fn delete_object_store(this: &IdbDatabase, name: &str) -> Result<(), JsValue>;
    #[cfg(feature = "IdbTransaction")]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "transaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    pub fn transaction_with_str(
        this: &IdbDatabase,
        store_names: &str,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(feature = "IdbTransaction")]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "transaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    pub fn transaction_with_str_sequence(
        this: &IdbDatabase,
        store_names: &::wasm_bindgen::JsValue,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "transaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
    pub fn transaction_with_str_and_mode(
        this: &IdbDatabase,
        store_names: &str,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "transaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
    pub fn transaction_with_str_sequence_and_mode(
        this: &IdbDatabase,
        store_names: &::wasm_bindgen::JsValue,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "IdbTransaction",
        feature = "IdbTransactionMode",
        feature = "IdbTransactionOptions",
    ))]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "transaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transaction_with_str_and_mode_and_options(
        this: &IdbDatabase,
        store_names: &str,
        mode: IdbTransactionMode,
        options: &IdbTransactionOptions,
    ) -> Result<IdbTransaction, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "IdbTransaction",
        feature = "IdbTransactionMode",
        feature = "IdbTransactionOptions",
    ))]
    #[wasm_bindgen(catch, method, js_class = "IDBDatabase", js_name = "transaction")]
    #[doc = "The `transaction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transaction_with_str_sequence_and_mode_and_options(
        this: &IdbDatabase,
        store_names: &[::js_sys::JsString],
        mode: IdbTransactionMode,
        options: &IdbTransactionOptions,
    ) -> Result<IdbTransaction, JsValue>;
}
