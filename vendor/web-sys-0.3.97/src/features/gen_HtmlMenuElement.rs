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
        js_name = "HTMLMenuElement",
        typescript_type = "HTMLMenuElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMenuElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub type HtmlMenuElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub fn type_(this: &HtmlMenuElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub fn set_type(this: &HtmlMenuElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuElement", js_name = "label")]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub fn label(this: &HtmlMenuElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuElement", js_name = "label")]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub fn set_label(this: &HtmlMenuElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLMenuElement", js_name = "compact")]
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub fn compact(this: &HtmlMenuElement) -> bool;
    #[wasm_bindgen(method, setter, js_class = "HTMLMenuElement", js_name = "compact")]
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMenuElement`*"]
    pub fn set_compact(this: &HtmlMenuElement, value: bool);
}
