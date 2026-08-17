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
        js_name = "HTMLUListElement",
        typescript_type = "HTMLUListElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlUListElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub type HtmlUListElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLUListElement", js_name = "compact")]
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn compact(this: &HtmlUListElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLUListElement", js_name = "compact")]
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn set_compact(this: &HtmlUListElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLUListElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn type_(this: &HtmlUListElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLUListElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
    pub fn set_type(this: &HtmlUListElement, value: &str);
}
