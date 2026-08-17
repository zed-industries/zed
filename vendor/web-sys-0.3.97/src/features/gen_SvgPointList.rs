#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGPointList",
        typescript_type = "SVGPointList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPointList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`*"]
    pub type SvgPointList;
    #[wasm_bindgen(method, getter, js_class = "SVGPointList", js_name = "numberOfItems")]
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`*"]
    pub fn number_of_items(this: &SvgPointList) -> u32;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList", js_name = "appendItem")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn append_item(this: &SvgPointList, new_item: &SvgPoint) -> Result<SvgPoint, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGPointList")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`*"]
    pub fn clear(this: &SvgPointList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList", js_name = "getItem")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn get_item(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn initialize(this: &SvgPointList, new_item: &SvgPoint) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList", js_name = "insertItemBefore")]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn insert_item_before(
        this: &SvgPointList,
        new_item: &SvgPoint,
        index: u32,
    ) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList", js_name = "removeItem")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn remove_item(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList", js_name = "replaceItem")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn replace_item(
        this: &SvgPointList,
        new_item: &SvgPoint,
        index: u32,
    ) -> Result<SvgPoint, JsValue>;
    #[cfg(feature = "SvgPoint")]
    #[wasm_bindgen(catch, method, js_class = "SVGPointList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgPointList`*"]
    pub fn get(this: &SvgPointList, index: u32) -> Result<SvgPoint, JsValue>;
}
