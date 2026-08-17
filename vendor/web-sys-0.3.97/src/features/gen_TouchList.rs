#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "TouchList",
        typescript_type = "TouchList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchList`*"]
    pub type TouchList;
    #[wasm_bindgen(method, getter, js_class = "TouchList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchList`*"]
    pub fn length(this: &TouchList) -> u32;
    #[cfg(feature = "Touch")]
    #[wasm_bindgen(method, js_class = "TouchList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TouchList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`, `TouchList`*"]
    pub fn item(this: &TouchList, index: u32) -> Option<Touch>;
    #[cfg(feature = "Touch")]
    #[wasm_bindgen(method, js_class = "TouchList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`, `TouchList`*"]
    pub fn get(this: &TouchList, index: u32) -> Option<Touch>;
}
