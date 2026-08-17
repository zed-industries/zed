#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DOMRectList",
        typescript_type = "DOMRectList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectList`*"]
    pub type DomRectList;
    #[wasm_bindgen(method, getter, js_class = "DOMRectList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectList`*"]
    pub fn length(this: &DomRectList) -> u32;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, js_class = "DOMRectList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `DomRectList`*"]
    pub fn item(this: &DomRectList, index: u32) -> Option<DomRect>;
    #[cfg(feature = "DomRect")]
    #[wasm_bindgen(method, js_class = "DOMRectList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `DomRectList`*"]
    pub fn get(this: &DomRectList, index: u32) -> Option<DomRect>;
}
