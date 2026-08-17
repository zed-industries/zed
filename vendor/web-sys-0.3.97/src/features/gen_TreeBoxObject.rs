#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "TreeBoxObject" , typescript_type = "TreeBoxObject")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeBoxObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub type TreeBoxObject;
    #[wasm_bindgen(method, getter, js_class = "TreeBoxObject", js_name = "focused")]
    #[doc = "Getter for the `focused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn focused(this: &TreeBoxObject) -> bool;
    #[wasm_bindgen(method, setter, js_class = "TreeBoxObject", js_name = "focused")]
    #[doc = "Setter for the `focused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn set_focused(this: &TreeBoxObject, value: bool);
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, getter, js_class = "TreeBoxObject", js_name = "treeBody")]
    #[doc = "Getter for the `treeBody` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/treeBody)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `TreeBoxObject`*"]
    pub fn tree_body(this: &TreeBoxObject) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "TreeBoxObject", js_name = "rowHeight")]
    #[doc = "Getter for the `rowHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn row_height(this: &TreeBoxObject) -> i32;
    #[wasm_bindgen(method, getter, js_class = "TreeBoxObject", js_name = "rowWidth")]
    #[doc = "Getter for the `rowWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn row_width(this: &TreeBoxObject) -> i32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TreeBoxObject",
        js_name = "horizontalPosition"
    )]
    #[doc = "Getter for the `horizontalPosition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/horizontalPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn horizontal_position(this: &TreeBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "beginUpdateBatch")]
    #[doc = "The `beginUpdateBatch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/beginUpdateBatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn begin_update_batch(this: &TreeBoxObject);
    #[wasm_bindgen(
        method,
        js_class = "TreeBoxObject",
        js_name = "clearStyleAndImageCaches"
    )]
    #[doc = "The `clearStyleAndImageCaches()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/clearStyleAndImageCaches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn clear_style_and_image_caches(this: &TreeBoxObject);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "endUpdateBatch")]
    #[doc = "The `endUpdateBatch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/endUpdateBatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn end_update_batch(this: &TreeBoxObject);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "ensureRowIsVisible")]
    #[doc = "The `ensureRowIsVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/ensureRowIsVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn ensure_row_is_visible(this: &TreeBoxObject, index: i32);
    #[cfg(feature = "TreeCellInfo")]
    #[wasm_bindgen(catch, method, js_class = "TreeBoxObject", js_name = "getCellAt")]
    #[doc = "The `getCellAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeCellInfo`*"]
    pub fn get_cell_at(this: &TreeBoxObject, x: i32, y: i32) -> Result<TreeCellInfo, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeBoxObject", js_name = "getCellAt")]
    #[doc = "The `getCellAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_cell_at_with_row_and_column_and_child_elt(
        this: &TreeBoxObject,
        x: i32,
        y: i32,
        row: &::js_sys::Object,
        column: &::js_sys::Object,
        child_elt: &::js_sys::Object,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "getFirstVisibleRow")]
    #[doc = "The `getFirstVisibleRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getFirstVisibleRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_first_visible_row(this: &TreeBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "getLastVisibleRow")]
    #[doc = "The `getLastVisibleRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getLastVisibleRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_last_visible_row(this: &TreeBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "getPageLength")]
    #[doc = "The `getPageLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getPageLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_page_length(this: &TreeBoxObject) -> i32;
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "getRowAt")]
    #[doc = "The `getRowAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getRowAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn get_row_at(this: &TreeBoxObject, x: i32, y: i32) -> i32;
    #[wasm_bindgen(method, js_class = "TreeBoxObject")]
    #[doc = "The `invalidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn invalidate(this: &TreeBoxObject);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "invalidateRange")]
    #[doc = "The `invalidateRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn invalidate_range(this: &TreeBoxObject, start_index: i32, end_index: i32);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "invalidateRow")]
    #[doc = "The `invalidateRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn invalidate_row(this: &TreeBoxObject, index: i32);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "rowCountChanged")]
    #[doc = "The `rowCountChanged()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowCountChanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn row_count_changed(this: &TreeBoxObject, index: i32, count: i32);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "scrollByLines")]
    #[doc = "The `scrollByLines()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn scroll_by_lines(this: &TreeBoxObject, num_lines: i32);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "scrollByPages")]
    #[doc = "The `scrollByPages()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByPages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn scroll_by_pages(this: &TreeBoxObject, num_pages: i32);
    #[wasm_bindgen(method, js_class = "TreeBoxObject", js_name = "scrollToRow")]
    #[doc = "The `scrollToRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollToRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
    pub fn scroll_to_row(this: &TreeBoxObject, index: i32);
}
