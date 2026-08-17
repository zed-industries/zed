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
        js_name = "HTMLTableSectionElement",
        typescript_type = "HTMLTableSectionElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableSectionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub type HtmlTableSectionElement;
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLTableSectionElement", js_name = "rows")]
    #[doc = "Getter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableSectionElement`*"]
    pub fn rows(this: &HtmlTableSectionElement) -> HtmlCollection;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLTableSectionElement",
        js_name = "align"
    )]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn align(this: &HtmlTableSectionElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLTableSectionElement",
        js_name = "align"
    )]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_align(this: &HtmlTableSectionElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableSectionElement", js_name = "ch")]
    #[doc = "Getter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn ch(this: &HtmlTableSectionElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableSectionElement", js_name = "ch")]
    #[doc = "Setter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_ch(this: &HtmlTableSectionElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLTableSectionElement",
        js_name = "chOff"
    )]
    #[doc = "Getter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn ch_off(this: &HtmlTableSectionElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLTableSectionElement",
        js_name = "chOff"
    )]
    #[doc = "Setter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_ch_off(this: &HtmlTableSectionElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLTableSectionElement",
        js_name = "vAlign"
    )]
    #[doc = "Getter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn v_align(this: &HtmlTableSectionElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLTableSectionElement",
        js_name = "vAlign"
    )]
    #[doc = "Setter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn set_v_align(this: &HtmlTableSectionElement, value: &str);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLTableSectionElement",
        js_name = "deleteRow"
    )]
    #[doc = "The `deleteRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn delete_row(this: &HtmlTableSectionElement, index: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLTableSectionElement",
        js_name = "insertRow"
    )]
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn insert_row(this: &HtmlTableSectionElement) -> Result<HtmlElement, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLTableSectionElement",
        js_name = "insertRow"
    )]
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
    pub fn insert_row_with_index(
        this: &HtmlTableSectionElement,
        index: i32,
    ) -> Result<HtmlElement, JsValue>;
}
