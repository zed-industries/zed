#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "StyleSheetList",
        typescript_type = "StyleSheetList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetList`*"]
    pub type StyleSheetList;
    #[wasm_bindgen(method, getter, js_class = "StyleSheetList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetList`*"]
    pub fn length(this: &StyleSheetList) -> u32;
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, js_class = "StyleSheetList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
    pub fn item(this: &StyleSheetList, index: u32) -> Option<StyleSheet>;
    #[cfg(feature = "StyleSheet")]
    #[wasm_bindgen(method, js_class = "StyleSheetList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
    pub fn get(this: &StyleSheetList, index: u32) -> Option<StyleSheet>;
}
