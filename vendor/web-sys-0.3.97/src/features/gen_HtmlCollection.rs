#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "HTMLCollection",
        typescript_type = "HTMLCollection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`*"]
    pub type HtmlCollection;
    #[wasm_bindgen(method, getter, js_class = "HTMLCollection", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`*"]
    pub fn length(this: &HtmlCollection) -> u32;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "HTMLCollection")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn item(this: &HtmlCollection, index: u32) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "HTMLCollection", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn named_item(this: &HtmlCollection, name: &str) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "HTMLCollection", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_with_index(this: &HtmlCollection, index: u32) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "HTMLCollection", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
    pub fn get_with_name(this: &HtmlCollection, name: &str) -> Option<Element>;
}
