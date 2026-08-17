#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "IDBFactory",
        typescript_type = "IDBFactory"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFactory` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`*"]
    pub type IdbFactory;
    #[wasm_bindgen(catch, method, js_class = "IDBFactory")]
    #[doc = "The `cmp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/cmp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`*"]
    pub fn cmp(
        this: &IdbFactory,
        first: &::wasm_bindgen::JsValue,
        second: &::wasm_bindgen::JsValue,
    ) -> Result<i16, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFactory", js_name = "deleteDatabase")]
    #[doc = "The `deleteDatabase()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn delete_database(this: &IdbFactory, name: &str) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(all(feature = "IdbOpenDbOptions", feature = "IdbOpenDbRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBFactory", js_name = "deleteDatabase")]
    #[doc = "The `deleteDatabase()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/deleteDatabase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    #[deprecated]
    pub fn delete_database_with_options(
        this: &IdbFactory,
        name: &str,
        options: &IdbOpenDbOptions,
    ) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFactory")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn open(this: &IdbFactory, name: &str) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFactory", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn open_with_u32(
        this: &IdbFactory,
        name: &str,
        version: u32,
    ) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(feature = "IdbOpenDbRequest")]
    #[wasm_bindgen(catch, method, js_class = "IDBFactory", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbRequest`*"]
    pub fn open_with_f64(
        this: &IdbFactory,
        name: &str,
        version: f64,
    ) -> Result<IdbOpenDbRequest, JsValue>;
    #[cfg(all(feature = "IdbOpenDbOptions", feature = "IdbOpenDbRequest",))]
    #[wasm_bindgen(catch, method, js_class = "IDBFactory", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFactory/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `IdbOpenDbOptions`, `IdbOpenDbRequest`*"]
    #[deprecated]
    pub fn open_with_idb_open_db_options(
        this: &IdbFactory,
        name: &str,
        options: &IdbOpenDbOptions,
    ) -> Result<IdbOpenDbRequest, JsValue>;
}
