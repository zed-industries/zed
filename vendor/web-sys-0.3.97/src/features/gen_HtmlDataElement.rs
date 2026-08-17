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
        js_name = "HTMLDataElement",
        typescript_type = "HTMLDataElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDataElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataElement`*"]
    pub type HtmlDataElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLDataElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataElement`*"]
    pub fn value(this: &HtmlDataElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDataElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataElement`*"]
    pub fn set_value(this: &HtmlDataElement, value: &str);
}
