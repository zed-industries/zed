#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IDBCursor",
        typescript_type = "IDBCursor"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbCursor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub type IdbCursor;
    #[wasm_bindgen(method, getter, js_class = "IDBCursor", js_name = "source")]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn source(this: &IdbCursor) -> ::js_sys::Object;
    #[cfg(feature = "IdbCursorDirection")]
    #[wasm_bindgen(method, getter, js_class = "IDBCursor", js_name = "direction")]
    #[doc = "Getter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbCursorDirection`*"]
    pub fn direction(this: &IdbCursor) -> IdbCursorDirection;
    #[wasm_bindgen(catch, method, getter, js_class = "IDBCursor", js_name = "key")]
    #[doc = "Getter for the `key` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn key(this: &IdbCursor) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "IDBCursor", js_name = "primaryKey")]
    #[doc = "Getter for the `primaryKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn primary_key(this: &IdbCursor) -> Result<::wasm_bindgen::JsValue, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(method, getter, js_class = "IDBCursor", js_name = "request")]
    #[doc = "Getter for the `request` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    #[deprecated]
    pub fn request(this: &IdbCursor) -> IdbRequest;
    #[wasm_bindgen(catch, method, js_class = "IDBCursor")]
    #[doc = "The `advance()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn advance(this: &IdbCursor, count: u32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBCursor", js_name = "continue")]
    #[doc = "The `continue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn continue_(this: &IdbCursor) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBCursor", js_name = "continue")]
    #[doc = "The `continue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn continue_with_key(
        this: &IdbCursor,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "IDBCursor", js_name = "continuePrimaryKey")]
    #[doc = "The `continuePrimaryKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`*"]
    pub fn continue_primary_key(
        this: &IdbCursor,
        key: &::wasm_bindgen::JsValue,
        primary_key: &::wasm_bindgen::JsValue,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBCursor")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    pub fn delete(this: &IdbCursor) -> Result<IdbRequest, JsValue>;
    #[cfg(feature = "IdbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBCursor")]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    pub fn update(this: &IdbCursor, value: &::wasm_bindgen::JsValue)
        -> Result<IdbRequest, JsValue>;
}
