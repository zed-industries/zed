#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "HTMLAllCollection",
        typescript_type = "HTMLAllCollection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAllCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub type HtmlAllCollection;
    #[wasm_bindgen(method, getter, js_class = "HTMLAllCollection", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn length(this: &HtmlAllCollection) -> u32;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, js_class = "HTMLAllCollection", js_name = "item")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
    pub fn item_with_index(this: &HtmlAllCollection, index: u32) -> Option<Node>;
    #[wasm_bindgen(method, js_class = "HTMLAllCollection", js_name = "item")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn item_with_name(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, js_class = "HTMLAllCollection", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn named_item(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;
    #[cfg(feature = "Node")]
    #[wasm_bindgen(method, js_class = "HTMLAllCollection", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
    pub fn get_with_index(this: &HtmlAllCollection, index: u32) -> Option<Node>;
    #[wasm_bindgen(method, js_class = "HTMLAllCollection", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
    pub fn get_with_name(this: &HtmlAllCollection, name: &str) -> Option<::js_sys::Object>;
}
