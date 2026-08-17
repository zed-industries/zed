#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "PaintRequestList",
        typescript_type = "PaintRequestList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintRequestList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequestList`*"]
    pub type PaintRequestList;
    #[wasm_bindgen(method, getter, js_class = "PaintRequestList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequestList`*"]
    pub fn length(this: &PaintRequestList) -> u32;
    #[cfg(feature = "PaintRequest")]
    #[wasm_bindgen(method, js_class = "PaintRequestList")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequestList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequest`, `PaintRequestList`*"]
    pub fn item(this: &PaintRequestList, index: u32) -> Option<PaintRequest>;
    #[cfg(feature = "PaintRequest")]
    #[wasm_bindgen(method, js_class = "PaintRequestList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequest`, `PaintRequestList`*"]
    pub fn get(this: &PaintRequestList, index: u32) -> Option<PaintRequest>;
}
