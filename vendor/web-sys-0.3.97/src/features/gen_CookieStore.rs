#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "CookieStore",
        typescript_type = "CookieStore"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CookieStore` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub type CookieStore;
    #[wasm_bindgen(method, getter, js_class = "CookieStore", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn onchange(this: &CookieStore) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "CookieStore", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn set_onchange(this: &CookieStore, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "delete")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn delete_with_name(this: &CookieStore, name: &str) -> ::js_sys::Promise;
    #[cfg(feature = "CookieStoreDeleteOptions")]
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "delete")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`, `CookieStoreDeleteOptions`*"]
    pub fn delete_with_options(
        this: &CookieStore,
        options: &CookieStoreDeleteOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "get")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn get_with_name(this: &CookieStore, name: &str) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStore")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn get(this: &CookieStore) -> ::js_sys::Promise;
    #[cfg(feature = "CookieStoreGetOptions")]
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "get")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`, `CookieStoreGetOptions`*"]
    pub fn get_with_cookie_store_get_options(
        this: &CookieStore,
        options: &CookieStoreGetOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn get_all_with_name(this: &CookieStore, name: &str) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn get_all(this: &CookieStore) -> ::js_sys::Promise;
    #[cfg(feature = "CookieStoreGetOptions")]
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`, `CookieStoreGetOptions`*"]
    pub fn get_all_with_cookie_store_get_options(
        this: &CookieStore,
        options: &CookieStoreGetOptions,
    ) -> ::js_sys::Promise;
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "set")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieStore`*"]
    pub fn set_with_name_and_value(
        this: &CookieStore,
        name: &str,
        value: &str,
    ) -> ::js_sys::Promise;
    #[cfg(feature = "CookieInit")]
    #[wasm_bindgen(method, js_class = "CookieStore", js_name = "set")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CookieInit`, `CookieStore`*"]
    pub fn set_with_options(this: &CookieStore, options: &CookieInit) -> ::js_sys::Promise;
}
