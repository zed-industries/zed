#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "IdbCursor",
        extends = "::js_sys::Object",
        js_name = "IDBCursorWithValue",
        typescript_type = "IDBCursorWithValue"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbCursorWithValue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
    pub type IdbCursorWithValue;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "IDBCursorWithValue",
        js_name = "value"
    )]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursorWithValue/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbCursorWithValue`*"]
    pub fn value(this: &IdbCursorWithValue) -> Result<::wasm_bindgen::JsValue, JsValue>;
}
