#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGPathSegList",
        typescript_type = "SVGPathSegList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegList`*"]
    pub type SvgPathSegList;
    #[wasm_bindgen(method, getter, js_class = "SVGPathSegList", js_name = "numberOfItems")]
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegList`*"]
    pub fn number_of_items(this: &SvgPathSegList) -> u32;
    #[cfg(feature = "SvgPathSeg")]
    #[wasm_bindgen(catch, method, js_class = "SVGPathSegList", js_name = "getItem")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*"]
    pub fn get_item(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue>;
    #[cfg(feature = "SvgPathSeg")]
    #[wasm_bindgen(catch, method, js_class = "SVGPathSegList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSeg`, `SvgPathSegList`*"]
    pub fn get(this: &SvgPathSegList, index: u32) -> Result<SvgPathSeg, JsValue>;
}
