#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MimeTypeArray",
        typescript_type = "MimeTypeArray"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MimeTypeArray` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeTypeArray`*"]
    pub type MimeTypeArray;
    #[wasm_bindgen(method, getter, js_class = "MimeTypeArray", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeTypeArray`*"]
    pub fn length(this: &MimeTypeArray) -> u32;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "MimeTypeArray")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn item(this: &MimeTypeArray, index: u32) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "MimeTypeArray", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn named_item(this: &MimeTypeArray, name: &str) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "MimeTypeArray", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn get_with_index(this: &MimeTypeArray, index: u32) -> Option<MimeType>;
    #[cfg(feature = "MimeType")]
    #[wasm_bindgen(method, js_class = "MimeTypeArray", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
    pub fn get_with_name(this: &MimeTypeArray, name: &str) -> Option<MimeType>;
}
