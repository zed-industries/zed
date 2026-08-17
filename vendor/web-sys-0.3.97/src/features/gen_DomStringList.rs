#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMStringList",
        typescript_type = "DOMStringList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomStringList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
    pub type DomStringList;
    #[wasm_bindgen(method, getter, js_class = "DOMStringList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn length(this: &DomStringList) -> u32;
    #[wasm_bindgen(method, js_class = "DOMStringList")]
    #[doc = "The `contains()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/contains)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn contains(this: &DomStringList, string: &str) -> bool;
    #[wasm_bindgen(method, js_class = "DOMStringList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn item(this: &DomStringList, index: u32) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, js_class = "DOMStringList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
    pub fn get(this: &DomStringList, index: u32) -> Option<::alloc::string::String>;
}
