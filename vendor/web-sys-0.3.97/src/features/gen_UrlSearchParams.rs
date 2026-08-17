#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "URLSearchParams",
        typescript_type = "URLSearchParams"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UrlSearchParams` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub type UrlSearchParams;
    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new() -> Result<UrlSearchParams, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new_with_str_sequence_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<UrlSearchParams, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new_with_record_from_str_to_str(
        init: &::js_sys::Object,
    ) -> Result<UrlSearchParams, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "URLSearchParams")]
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn new_with_str(init: &str) -> Result<UrlSearchParams, JsValue>;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn append(this: &UrlSearchParams, name: &str, value: &str);
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn delete(this: &UrlSearchParams, name: &str);
    #[wasm_bindgen(catch, method, js_class = "URLSearchParams", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn for_each(this: &UrlSearchParams, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn get(this: &UrlSearchParams, name: &str) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "URLSearchParams", js_name = "getAll")]
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn get_all(this: &UrlSearchParams, name: &str) -> ::js_sys::Array;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn has(this: &UrlSearchParams, name: &str) -> bool;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn set(this: &UrlSearchParams, name: &str, value: &str);
    #[wasm_bindgen(catch, method, js_class = "URLSearchParams")]
    #[doc = "The `sort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn sort(this: &UrlSearchParams) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn entries(this: &UrlSearchParams) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn keys(this: &UrlSearchParams) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "URLSearchParams")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
    pub fn values(this: &UrlSearchParams) -> ::js_sys::Iterator;
}
