#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlCollection",
        extends = "::js_sys::Object",
        js_name = "HTMLFormControlsCollection",
        typescript_type = "HTMLFormControlsCollection"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFormControlsCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    pub type HtmlFormControlsCollection;
    #[wasm_bindgen(method, js_class = "HTMLFormControlsCollection", js_name = "namedItem")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    pub fn named_item(this: &HtmlFormControlsCollection, name: &str) -> Option<::js_sys::Object>;
    #[wasm_bindgen(method, js_class = "HTMLFormControlsCollection", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
    pub fn get(this: &HtmlFormControlsCollection, name: &str) -> Option<::js_sys::Object>;
}
