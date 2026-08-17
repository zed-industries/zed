#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "SVGStringList",
        typescript_type = "SVGStringList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStringList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub type SvgStringList;
    #[wasm_bindgen(method, getter, js_class = "SVGStringList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn length(this: &SvgStringList) -> u32;
    #[wasm_bindgen(method, getter, js_class = "SVGStringList", js_name = "numberOfItems")]
    #[doc = "Getter for the `numberOfItems` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/numberOfItems)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn number_of_items(this: &SvgStringList) -> u32;
    #[wasm_bindgen(catch, method, js_class = "SVGStringList", js_name = "appendItem")]
    #[doc = "The `appendItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/appendItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn append_item(
        this: &SvgStringList,
        new_item: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGStringList")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn clear(this: &SvgStringList);
    #[wasm_bindgen(catch, method, js_class = "SVGStringList", js_name = "getItem")]
    #[doc = "The `getItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/getItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn get_item(this: &SvgStringList, index: u32) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGStringList")]
    #[doc = "The `initialize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/initialize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn initialize(
        this: &SvgStringList,
        new_item: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "SVGStringList",
        js_name = "insertItemBefore"
    )]
    #[doc = "The `insertItemBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/insertItemBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn insert_item_before(
        this: &SvgStringList,
        new_item: &str,
        index: u32,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGStringList", js_name = "removeItem")]
    #[doc = "The `removeItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/removeItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn remove_item(
        this: &SvgStringList,
        index: u32,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "SVGStringList", js_name = "replaceItem")]
    #[doc = "The `replaceItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/replaceItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn replace_item(
        this: &SvgStringList,
        new_item: &str,
        index: u32,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "SVGStringList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`*"]
    pub fn get(this: &SvgStringList, index: u32) -> Option<::alloc::string::String>;
}
