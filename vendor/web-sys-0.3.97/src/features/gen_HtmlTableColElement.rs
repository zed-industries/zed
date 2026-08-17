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
        js_name = "HTMLTableColElement",
        typescript_type = "HTMLTableColElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableColElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub type HtmlTableColElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLTableColElement", js_name = "span")]
    #[doc = "Getter for the `span` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn span(this: &HtmlTableColElement) -> u32;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableColElement", js_name = "span")]
    #[doc = "Setter for the `span` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/span)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_span(this: &HtmlTableColElement, value: u32);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableColElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn align(this: &HtmlTableColElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableColElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_align(this: &HtmlTableColElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableColElement", js_name = "ch")]
    #[doc = "Getter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn ch(this: &HtmlTableColElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableColElement", js_name = "ch")]
    #[doc = "Setter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_ch(this: &HtmlTableColElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableColElement", js_name = "chOff")]
    #[doc = "Getter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn ch_off(this: &HtmlTableColElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableColElement", js_name = "chOff")]
    #[doc = "Setter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_ch_off(this: &HtmlTableColElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableColElement", js_name = "vAlign")]
    #[doc = "Getter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn v_align(this: &HtmlTableColElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableColElement", js_name = "vAlign")]
    #[doc = "Setter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_v_align(this: &HtmlTableColElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLTableColElement", js_name = "width")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn width(this: &HtmlTableColElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTableColElement", js_name = "width")]
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableColElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableColElement`*"]
    pub fn set_width(this: &HtmlTableColElement, value: &str);
}
