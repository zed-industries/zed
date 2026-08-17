#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IDBObjectStore",
        typescript_type = "IDBObjectStore"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbObjectStore` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub type IdbObjectStore;
    #[wasm_bindgen(method, getter, js_class = "IDBObjectStore", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn name(this: &IdbObjectStore) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "IDBObjectStore", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn set_name(this: &IdbObjectStore, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "IDBObjectStore",
        js_name = "keyPath"
    )]
    #[doc = "Getter for the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/keyPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn key_path(this: &IdbObjectStore) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "DomStringList")]
    #[wasm_bindgen(method, getter, js_class = "IDBObjectStore", js_name = "indexNames")]
    #[doc = "Getter for the `indexNames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/indexNames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`, `IdbObjectStore`*"]
    pub fn index_names(this: &IdbObjectStore) -> DomStringList;
    #[cfg(feature = "IdbTransaction")]
    #[wasm_bindgen(method, getter, js_class = "IDBObjectStore", js_name = "transaction")]
    #[doc = "Getter for the `transaction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
    pub fn transaction(this: &IdbObjectStore) -> IdbTransaction;
    #[wasm_bindgen(method, getter, js_class = "IDBObjectStore", js_name = "autoIncrement")]
    #[doc = "Getter for the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/autoIncrement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn auto_increment(this: &IdbObjectStore) -> bool;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn add(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn add_with_key(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn clear(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `count()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn count(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "count")]
    #[doc = "The `count()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/count)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn count_with_key(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbIndex")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "createIndex")]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    pub fn create_index_with_str(
        this: &IdbObjectStore,
        name: &str,
        key_path: &str,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(feature = "IdbIndex")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "createIndex")]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    pub fn create_index_with_str_sequence(
        this: &IdbObjectStore,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "createIndex")]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
    pub fn create_index_with_str_and_optional_parameters(
        this: &IdbObjectStore,
        name: &str,
        key_path: &str,
        options: &IdbIndexParameters,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(all(feature = "IdbIndex", feature = "IdbIndexParameters",))]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "createIndex")]
    #[doc = "The `createIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/createIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbIndexParameters`, `IdbObjectStore`*"]
    pub fn create_index_with_str_sequence_and_optional_parameters(
        this: &IdbObjectStore,
        name: &str,
        key_path: &::wasm_bindgen::JsValue,
        options: &IdbIndexParameters,
    ) -> Result<IdbIndex, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn delete(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "deleteIndex")]
    #[doc = "The `deleteIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/deleteIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`*"]
    pub fn delete_index(this: &IdbObjectStore, name: &str) -> Result<(), JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_with_key(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_with_key_and_limit(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
        count: u32,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getAllKeys")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_keys(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getAllKeys")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_keys_with_key(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getAllKeys")]
    #[doc = "The `getAllKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getAllKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_all_keys_with_key_and_limit(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
        count: u32,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "getKey")]
    #[doc = "The `getKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/getKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn get_key(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbIndex")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `index()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndex`, `IdbObjectStore`*"]
    pub fn index(this: &IdbObjectStore, name: &str) -> Result<IdbIndex, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "openCursor")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_cursor(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "openCursor")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_cursor_with_range(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "openCursor")]
    #[doc = "The `openCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_cursor_with_range_and_direction(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "openKeyCursor")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_key_cursor(this: &IdbObjectStore) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "openKeyCursor")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_key_cursor_with_range(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(all(feature = "IdbCursorDirection", feature = "IdbRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "openKeyCursor")]
    #[doc = "The `openKeyCursor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/openKeyCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`, `IdbObjectStore`, `IdbRequest`*"]
    pub fn open_key_cursor_with_range_and_direction(
        this: &IdbObjectStore,
        query: &::wasm_bindgen::JsValue,
        direction: IdbCursorDirection,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore")]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn put(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBObjectStore", js_name = "put")]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBObjectStore/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbRequest`*"]
    pub fn put_with_key(
        this: &IdbObjectStore,
        value: &::wasm_bindgen::JsValue,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, JsValue>;
}
