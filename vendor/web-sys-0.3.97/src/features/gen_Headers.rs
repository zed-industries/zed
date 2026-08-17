#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "Headers",
        typescript_type = "Headers"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Headers` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub type Headers;
    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn new() -> Result<Headers, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn new_with_headers(init: &Headers) -> Result<Headers, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn new_with_str_sequence_sequence(
        init: &::wasm_bindgen::JsValue,
    ) -> Result<Headers, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "Headers")]
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn new_with_record_from_str_to_str(init: &::js_sys::Object) -> Result<Headers, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Headers")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn append(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Headers")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn delete(this: &Headers, name: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Headers", js_name = "forEach")]
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn for_each(this: &Headers, callback: &::js_sys::Function) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Headers")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn get(this: &Headers, name: &str) -> Result<Option<::alloc::string::String>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Headers")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn has(this: &Headers, name: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "Headers")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn set(this: &Headers, name: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "Headers")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn entries(this: &Headers) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "Headers")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn keys(this: &Headers) -> ::js_sys::Iterator;
    #[wasm_bindgen(method, js_class = "Headers")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
    pub fn values(this: &Headers) -> ::js_sys::Iterator;
}
