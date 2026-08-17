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
        js_name = "HTMLHtmlElement",
        typescript_type = "HTMLHtmlElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlHtmlElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
    pub type HtmlHtmlElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLHtmlElement", js_name = "version")]
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
    pub fn version(this: &HtmlHtmlElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLHtmlElement", js_name = "version")]
    #[doc = "Setter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
    pub fn set_version(this: &HtmlHtmlElement, value: &str);
}
