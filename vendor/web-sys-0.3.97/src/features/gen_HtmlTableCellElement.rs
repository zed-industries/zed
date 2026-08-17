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
        js_name = "HTMLTableCellElement",
        typescript_type = "HTMLTableCellElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableCellElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub type HtmlTableCellElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "colSpan")]
    #[doc = "Getter for the `colSpan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn col_span(this: &HtmlTableCellElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "colSpan")]
    #[doc = "Setter for the `colSpan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/colSpan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_col_span(this: &HtmlTableCellElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "rowSpan")]
    #[doc = "Getter for the `rowSpan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn row_span(this: &HtmlTableCellElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "rowSpan")]
    #[doc = "Setter for the `rowSpan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/rowSpan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_row_span(this: &HtmlTableCellElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "headers")]
    #[doc = "Getter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn headers(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "headers")]
    #[doc = "Setter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_headers(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLTableCellElement",
        js_name = "cellIndex"
    )]
    #[doc = "Getter for the `cellIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/cellIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn cell_index(this: &HtmlTableCellElement) -> i32;
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "abbr")]
    #[doc = "Getter for the `abbr` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/abbr)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn abbr(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "abbr")]
    #[doc = "Setter for the `abbr` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/abbr)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_abbr(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "scope")]
    #[doc = "Getter for the `scope` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/scope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn scope(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "scope")]
    #[doc = "Setter for the `scope` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/scope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_scope(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn align(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_align(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "axis")]
    #[doc = "Getter for the `axis` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn axis(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "axis")]
    #[doc = "Setter for the `axis` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/axis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_axis(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "height")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn height(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "height")]
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_height(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn width(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_width(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "ch")]
    #[doc = "Getter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn ch(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "ch")]
    #[doc = "Setter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_ch(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "chOff")]
    #[doc = "Getter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn ch_off(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "chOff")]
    #[doc = "Setter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_ch_off(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "noWrap")]
    #[doc = "Getter for the `noWrap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn no_wrap(this: &HtmlTableCellElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "noWrap")]
    #[doc = "Setter for the `noWrap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/noWrap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_no_wrap(this: &HtmlTableCellElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "vAlign")]
    #[doc = "Getter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn v_align(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "vAlign")]
    #[doc = "Setter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_v_align(this: &HtmlTableCellElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableCellElement", js_name = "bgColor")]
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn bg_color(this: &HtmlTableCellElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableCellElement", js_name = "bgColor")]
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCellElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCellElement`*"]
    pub fn set_bg_color(this: &HtmlTableCellElement, value: &str);
}
