#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGNumberList",
        typescript_type = "SVGNumberList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgNumberList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumberList`*"]
    pub type SvgNumberList;
    #[wasm_bindgen(method, getter, js_class = "SVGNumberList", js_name = "numberOfItems")]
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumberList`*"]
    pub fn number_of_items(this: &SvgNumberList) -> u32;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList", js_name = "appendItem")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn append_item(this: &SvgNumberList, new_item: &SvgNumber) -> Result<SvgNumber, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumberList`*"]
    pub fn clear(this: &SvgNumberList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList", js_name = "getItem")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn get_item(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn initialize(this: &SvgNumberList, new_item: &SvgNumber) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGNumberList",
        js_name = "insertItemBefore"
    )]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn insert_item_before(
        this: &SvgNumberList,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList", js_name = "removeItem")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn remove_item(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList", js_name = "replaceItem")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn replace_item(
        this: &SvgNumberList,
        new_item: &SvgNumber,
        index: u32,
    ) -> Result<SvgNumber, JsValue>;
    #[cfg(feature = "SvgNumber")]
    #[wasm_bindgen(catch, method, js_class = "SVGNumberList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgNumberList`*"]
    pub fn get(this: &SvgNumberList, index: u32) -> Result<SvgNumber, JsValue>;
}
