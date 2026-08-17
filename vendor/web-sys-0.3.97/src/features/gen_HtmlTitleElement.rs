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
        js_name = "HTMLTitleElement",
        typescript_type = "HTMLTitleElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTitleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
    pub type HtmlTitleElement;
    #[wasm_bindgen(catch, method, getter, js_class = "HTMLTitleElement", js_name = "text")]
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
    pub fn text(this: &HtmlTitleElement) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "HTMLTitleElement", js_name = "text")]
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
    pub fn set_text(this: &HtmlTitleElement, value: &str) -> Result<(), JsValue>;
}
