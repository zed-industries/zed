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
        js_name = "HTMLDirectoryElement",
        typescript_type = "HTMLDirectoryElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDirectoryElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
    pub type HtmlDirectoryElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLDirectoryElement", js_name = "compact")]
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
    pub fn compact(this: &HtmlDirectoryElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLDirectoryElement", js_name = "compact")]
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
    pub fn set_compact(this: &HtmlDirectoryElement, value: bool);
}
