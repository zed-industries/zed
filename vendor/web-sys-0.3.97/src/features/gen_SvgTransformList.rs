#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGTransformList",
        typescript_type = "SVGTransformList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTransformList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransformList`*"]
    pub type SvgTransformList;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "SVGTransformList",
        js_name = "numberOfItems"
    )]
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransformList`*"]
    pub fn number_of_items(this: &SvgTransformList) -> u32;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList", js_name = "appendItem")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn append_item(
        this: &SvgTransformList,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransformList`*"]
    pub fn clear(this: &SvgTransformList) -> Result<(), JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList")]
    #[doc = "The `consolidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/consolidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn consolidate(this: &SvgTransformList) -> Result<Option<SvgTransform>, JsValue>;
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    #[wasm_bindgen(
        method,
        js_class = "SVGTransformList",
        js_name = "createSVGTransformFromMatrix"
    )]
    #[doc = "The `createSVGTransformFromMatrix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgTransformList`*"]
    pub fn create_svg_transform_from_matrix(
        this: &SvgTransformList,
        matrix: &SvgMatrix,
    ) -> SvgTransform;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList", js_name = "getItem")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn get_item(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn initialize(
        this: &SvgTransformList,
        new_item: &SvgTransform,
    ) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGTransformList",
        js_name = "insertItemBefore"
    )]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn insert_item_before(
        this: &SvgTransformList,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList", js_name = "removeItem")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn remove_item(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList", js_name = "replaceItem")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn replace_item(
        this: &SvgTransformList,
        new_item: &SvgTransform,
        index: u32,
    ) -> Result<SvgTransform, JsValue>;
    #[cfg(feature = "SvgTransform")]
    #[wasm_bindgen(catch, method, js_class = "SVGTransformList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgTransformList`*"]
    pub fn get(this: &SvgTransformList, index: u32) -> Result<SvgTransform, JsValue>;
}
