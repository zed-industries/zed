#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "DataTransferItemList",
        typescript_type = "DataTransferItemList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DataTransferItemList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    pub type DataTransferItemList;
    #[wasm_bindgen(method, getter, js_class = "DataTransferItemList", js_name = "length")]
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    pub fn length(this: &DataTransferItemList) -> u32;
    #[cfg(feature = "DataTransferItem")]
    #[wasm_bindgen(catch, method, js_class = "DataTransferItemList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*"]
    pub fn add_with_str_and_type(
        this: &DataTransferItemList,
        data: &str,
        type_: &str,
    ) -> Result<Option<DataTransferItem>, JsValue>;
    #[cfg(all(feature = "DataTransferItem", feature = "File",))]
    #[wasm_bindgen(catch, method, js_class = "DataTransferItemList", js_name = "add")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`, `File`*"]
    pub fn add_with_file(
        this: &DataTransferItemList,
        data: &File,
    ) -> Result<Option<DataTransferItem>, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransferItemList")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    pub fn clear(this: &DataTransferItemList) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "DataTransferItemList")]
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItemList`*"]
    pub fn remove(this: &DataTransferItemList, index: u32) -> Result<(), JsValue>;
    #[cfg(feature = "DataTransferItem")]
    #[wasm_bindgen(method, js_class = "DataTransferItemList", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransferItem`, `DataTransferItemList`*"]
    pub fn get(this: &DataTransferItemList, index: u32) -> Option<DataTransferItem>;
}
