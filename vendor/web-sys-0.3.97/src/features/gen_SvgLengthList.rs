#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGLengthList",
        typescript_type = "SVGLengthList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgLengthList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLengthList`*"]
    pub type SvgLengthList;
    #[wasm_bindgen(method, getter, js_class = "SVGLengthList", js_name = "numberOfItems")]
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLengthList`*"]
    pub fn number_of_items(this: &SvgLengthList) -> u32;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList", js_name = "appendItem")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn append_item(this: &SvgLengthList, new_item: &SvgLength) -> Result<SvgLength, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLengthList`*"]
    pub fn clear(this: &SvgLengthList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList", js_name = "getItem")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn get_item(this: &SvgLengthList, index: u32) -> Result<SvgLength, JsValue>;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn initialize(this: &SvgLengthList, new_item: &SvgLength) -> Result<SvgLength, JsValue>;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGLengthList",
        js_name = "insertItemBefore"
    )]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn insert_item_before(
        this: &SvgLengthList,
        new_item: &SvgLength,
        index: u32,
    ) -> Result<SvgLength, JsValue>;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList", js_name = "removeItem")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn remove_item(this: &SvgLengthList, index: u32) -> Result<SvgLength, JsValue>;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList", js_name = "replaceItem")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn replace_item(
        this: &SvgLengthList,
        new_item: &SvgLength,
        index: u32,
    ) -> Result<SvgLength, JsValue>;
    #[cfg(feature = "SvgLength")]
    #[wasm_bindgen(catch, method, js_class = "SVGLengthList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgLengthList`*"]
    pub fn get(this: &SvgLengthList, index: u32) -> Result<SvgLength, JsValue>;
}
