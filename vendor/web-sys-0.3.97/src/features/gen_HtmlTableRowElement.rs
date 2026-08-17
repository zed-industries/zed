#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "HtmlElement",
        extends = "Element",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLTableRowElement",
        typescript_type = "HTMLTableRowElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableRowElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub type HtmlTableRowElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "rowIndex")]
    #[doc = "Getter for the `rowIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn row_index(this: &HtmlTableRowElement) -> i32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLTableRowElement",
        js_name = "sectionRowIndex"
    )]
    #[doc = "Getter for the `sectionRowIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn section_row_index(this: &HtmlTableRowElement) -> i32;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "cells")]
    #[doc = "Getter for the `cells` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableRowElement`*"]
    pub fn cells(this: &HtmlTableRowElement) -> HtmlCollection;
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn align(this: &HtmlTableRowElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableRowElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_align(this: &HtmlTableRowElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "ch")]
    #[doc = "Getter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn ch(this: &HtmlTableRowElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableRowElement", js_name = "ch")]
    #[doc = "Setter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_ch(this: &HtmlTableRowElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "chOff")]
    #[doc = "Getter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn ch_off(this: &HtmlTableRowElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableRowElement", js_name = "chOff")]
    #[doc = "Setter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_ch_off(this: &HtmlTableRowElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "vAlign")]
    #[doc = "Getter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn v_align(this: &HtmlTableRowElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableRowElement", js_name = "vAlign")]
    #[doc = "Setter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_v_align(this: &HtmlTableRowElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableRowElement", js_name = "bgColor")]
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn bg_color(this: &HtmlTableRowElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableRowElement", js_name = "bgColor")]
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn set_bg_color(this: &HtmlTableRowElement, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLTableRowElement",
        js_name = "deleteCell"
    )]
    #[doc = "The `deleteCell()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn delete_cell(this: &HtmlTableRowElement, index: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLTableRowElement",
        js_name = "insertCell"
    )]
    #[doc = "The `insertCell()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn insert_cell(this: &HtmlTableRowElement) -> Result<HtmlElement, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLTableRowElement",
        js_name = "insertCell"
    )]
    #[doc = "The `insertCell()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
    pub fn insert_cell_with_index(
        this: &HtmlTableRowElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;
}
