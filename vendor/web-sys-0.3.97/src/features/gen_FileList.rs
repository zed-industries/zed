#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "FileList",
        typescript_type = "FileList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileList`*"]
    pub type FileList;
    #[wasm_bindgen(method, getter, js_class = "FileList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileList`*"]
    pub fn length(this: &FileList) -> u32;
    #[cfg(feature = "File")]
    #[wasm_bindgen(method, js_class = "FileList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FileList`*"]
    pub fn item(this: &FileList, index: u32) -> Option<File>;
    #[cfg(feature = "File")]
    #[wasm_bindgen(method, js_class = "FileList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `File`, `FileList`*"]
    pub fn get(this: &FileList, index: u32) -> Option<File>;
}
