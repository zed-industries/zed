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
        js_name = "HTMLBRElement",
        typescript_type = "HTMLBRElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBrElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBrElement`*"]
    pub type HtmlBrElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLBRElement", js_name = "clear")]
    #[doc = "Getter for the `clear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBrElement`*"]
    pub fn clear(this: &HtmlBrElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBRElement", js_name = "clear")]
    #[doc = "Setter for the `clear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBrElement`*"]
    pub fn set_clear(this: &HtmlBrElement, value: &str);
}
