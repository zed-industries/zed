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
        js_name = "HTMLDivElement",
        typescript_type = "HTMLDivElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDivElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDivElement`*"]
    pub type HtmlDivElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLDivElement", js_name = "align")]
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDivElement`*"]
    pub fn align(this: &HtmlDivElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDivElement", js_name = "align")]
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDivElement`*"]
    pub fn set_align(this: &HtmlDivElement, value: &str);
}
