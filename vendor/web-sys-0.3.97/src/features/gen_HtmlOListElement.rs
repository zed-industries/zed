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
        js_name = "HTMLOListElement",
        typescript_type = "HTMLOListElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOListElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub type HtmlOListElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLOListElement", js_name = "reversed")]
    #[doc = "Getter for the `reversed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn reversed(this: &HtmlOListElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLOListElement", js_name = "reversed")]
    #[doc = "Setter for the `reversed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/reversed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn set_reversed(this: &HtmlOListElement, value: bool);
    #[wasm_bindgen(method, getter, js_class = "HTMLOListElement", js_name = "start")]
    #[doc = "Getter for the `start` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn start(this: &HtmlOListElement) -> i32;
    #[wasm_bindgen(method, setter, js_class = "HTMLOListElement", js_name = "start")]
    #[doc = "Setter for the `start` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn set_start(this: &HtmlOListElement, value: i32);
    #[wasm_bindgen(method, getter, js_class = "HTMLOListElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn type_(this: &HtmlOListElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLOListElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn set_type(this: &HtmlOListElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLOListElement", js_name = "compact")]
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn compact(this: &HtmlOListElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLOListElement", js_name = "compact")]
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOListElement`*"]
    pub fn set_compact(this: &HtmlOListElement, value: bool);
}
