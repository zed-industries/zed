#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMStringMap",
        typescript_type = "DOMStringMap"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomStringMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
    pub type DomStringMap;
    #[wasm_bindgen(method, js_class = "DOMStringMap", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
    pub fn get(this: &DomStringMap, name: &str) -> Option<::alloc::string::String>;
    #[wasm_bindgen(catch, method, js_class = "DOMStringMap", indexing_setter)]
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
    pub fn set(this: &DomStringMap, name: &str, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "DOMStringMap", indexing_deleter)]
    #[doc = "Indexing deleter. As in the literal Javascript `delete this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
    pub fn delete(this: &DomStringMap, name: &str);
}
