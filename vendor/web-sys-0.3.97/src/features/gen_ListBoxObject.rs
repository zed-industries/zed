#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "ListBoxObject" , typescript_type = "ListBoxObject")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ListBoxObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub type ListBoxObject;
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "ensureIndexIsVisible")]
    #[doc = "The `ensureIndexIsVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/ensureIndexIsVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn ensure_index_is_visible(this: &ListBoxObject, row_index: i32);
    #[wasm_bindgen(
        method,
        js_class = "ListBoxObject",
        js_name = "getIndexOfFirstVisibleRow"
    )]
    #[doc = "The `getIndexOfFirstVisibleRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfFirstVisibleRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_index_of_first_visible_row(this: &ListBoxObject) -> i32;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "getIndexOfItem")]
    #[doc = "The `getIndexOfItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
    pub fn get_index_of_item(this: &ListBoxObject, item: &Element) -> i32;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "getItemAtIndex")]
    #[doc = "The `getItemAtIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getItemAtIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
    pub fn get_item_at_index(this: &ListBoxObject, index: i32) -> Option<Element>;
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "getNumberOfVisibleRows")]
    #[doc = "The `getNumberOfVisibleRows()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getNumberOfVisibleRows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_number_of_visible_rows(this: &ListBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "getRowCount")]
    #[doc = "The `getRowCount()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_row_count(this: &ListBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "getRowHeight")]
    #[doc = "The `getRowHeight()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn get_row_height(this: &ListBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "scrollByLines")]
    #[doc = "The `scrollByLines()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollByLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn scroll_by_lines(this: &ListBoxObject, num_lines: i32);
    #[wasm_bindgen(method, js_class = "ListBoxObject", js_name = "scrollToIndex")]
    #[doc = "The `scrollToIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollToIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
    pub fn scroll_to_index(this: &ListBoxObject, row_index: i32);
}
