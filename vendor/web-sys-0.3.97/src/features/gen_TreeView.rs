#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "TreeView" , typescript_type = "TreeView")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeView` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub type TreeView;
    #[wasm_bindgen(method, getter, js_class = "TreeView", js_name = "rowCount")]
    #[doc = "Getter for the `rowCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/rowCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn row_count(this: &TreeView) -> i32;
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "canDrop")]
    #[doc = "The `canDrop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/canDrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
    pub fn can_drop(
        this: &TreeView,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<bool, JsValue>;
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(catch, method, js_class = "TreeView")]
    #[doc = "The `drop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/drop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
    pub fn drop(
        this: &TreeView,
        row: i32,
        orientation: i32,
        data_transfer: Option<&DataTransfer>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "getLevel")]
    #[doc = "The `getLevel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getLevel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn get_level(this: &TreeView, row: i32) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "getParentIndex")]
    #[doc = "The `getParentIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getParentIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn get_parent_index(this: &TreeView, row: i32) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "getRowProperties")]
    #[doc = "The `getRowProperties()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getRowProperties)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn get_row_properties(
        this: &TreeView,
        row: i32,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "hasNextSibling")]
    #[doc = "The `hasNextSibling()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/hasNextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn has_next_sibling(this: &TreeView, row: i32, after_index: i32) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "isContainer")]
    #[doc = "The `isContainer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_container(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "isContainerEmpty")]
    #[doc = "The `isContainerEmpty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerEmpty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_container_empty(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "isContainerOpen")]
    #[doc = "The `isContainerOpen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerOpen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_container_open(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "isSeparator")]
    #[doc = "The `isSeparator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSeparator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_separator(this: &TreeView, row: i32) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_class = "TreeView", js_name = "isSorted")]
    #[doc = "The `isSorted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSorted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn is_sorted(this: &TreeView) -> bool;
    #[wasm_bindgen(method, js_class = "TreeView", js_name = "performAction")]
    #[doc = "The `performAction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn perform_action(this: &TreeView, action: &str);
    #[wasm_bindgen(method, js_class = "TreeView", js_name = "performActionOnRow")]
    #[doc = "The `performActionOnRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performActionOnRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn perform_action_on_row(this: &TreeView, action: &str, row: i32);
    #[wasm_bindgen(method, js_class = "TreeView", js_name = "selectionChanged")]
    #[doc = "The `selectionChanged()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/selectionChanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn selection_changed(this: &TreeView);
    #[cfg(feature = "TreeBoxObject")]
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "setTree")]
    #[doc = "The `setTree()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/setTree)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeView`*"]
    pub fn set_tree(this: &TreeView, tree: Option<&TreeBoxObject>) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "TreeView", js_name = "toggleOpenState")]
    #[doc = "The `toggleOpenState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/toggleOpenState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub fn toggle_open_state(this: &TreeView, row: i32) -> Result<(), JsValue>;
}
impl TreeView {
    #[doc = "The `TreeView.DROP_BEFORE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub const DROP_BEFORE: i16 = -1i64 as i16;
    #[doc = "The `TreeView.DROP_ON` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub const DROP_ON: i16 = 0i64 as i16;
    #[doc = "The `TreeView.DROP_AFTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub const DROP_AFTER: i16 = 1u64 as i16;
}
