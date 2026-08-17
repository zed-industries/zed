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
        js_name = "HTMLTableCaptionElement",
        typescript_type = "HTMLTableCaptionElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableCaptionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*"]
    pub type HtmlTableCaptionElement;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "HTMLTableCaptionElement",
        js_name = "align"
    )]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*"]
    pub fn align(this: &HtmlTableCaptionElement) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "HTMLTableCaptionElement",
        js_name = "align"
    )]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableCaptionElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`*"]
    pub fn set_align(this: &HtmlTableCaptionElement, value: &str);
}
