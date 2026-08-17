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
        js_name = "HTMLQuoteElement",
        typescript_type = "HTMLQuoteElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlQuoteElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub type HtmlQuoteElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLQuoteElement", js_name = "cite")]
    #[doc = "Getter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn cite(this: &HtmlQuoteElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLQuoteElement", js_name = "cite")]
    #[doc = "Setter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
    pub fn set_cite(this: &HtmlQuoteElement, value: &str);
}
