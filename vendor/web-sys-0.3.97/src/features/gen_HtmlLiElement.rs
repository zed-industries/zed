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
        js_name = "HTMLLIElement",
        typescript_type = "HTMLLIElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLiElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub type HtmlLiElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLLIElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn value(this: &HtmlLiElement) -> i32;
    #[wasm_bindgen(method, setter, js_class = "HTMLLIElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn set_value(this: &HtmlLiElement, value: i32);
    #[wasm_bindgen(method, getter, js_class = "HTMLLIElement", js_name = "type")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn type_(this: &HtmlLiElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLLIElement", js_name = "type")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
    pub fn set_type(this: &HtmlLiElement, value: &str);
}
