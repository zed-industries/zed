#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "StorageManager",
        typescript_type = "StorageManager"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    pub type StorageManager;
    #[wasm_bindgen(catch, method, js_class = "StorageManager")]
    #[doc = "The `estimate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn estimate(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(method, js_class = "StorageManager", js_name = "getDirectory")]
    #[doc = "The `getDirectory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/getDirectory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn get_directory(this: &StorageManager) -> ::js_sys::Promise;
    #[wasm_bindgen(catch, method, js_class = "StorageManager")]
    #[doc = "The `persist()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn persist(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "StorageManager")]
    #[doc = "The `persisted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageManager`*"]
    pub fn persisted(this: &StorageManager) -> Result<::js_sys::Promise, JsValue>;
}
