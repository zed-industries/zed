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
        js_name = "HTMLDListElement",
        typescript_type = "HTMLDListElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDListElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDListElement`*"]
    pub type HtmlDListElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLDListElement", js_name = "compact")]
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDListElement`*"]
    pub fn compact(this: &HtmlDListElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLDListElement", js_name = "compact")]
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDListElement`*"]
    pub fn set_compact(this: &HtmlDListElement, value: bool);
}
