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
        js_name = "HTMLBaseElement",
        typescript_type = "HTMLBaseElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBaseElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub type HtmlBaseElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLBaseElement", js_name = "href")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn href(this: &HtmlBaseElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBaseElement", js_name = "href")]
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn set_href(this: &HtmlBaseElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLBaseElement", js_name = "target")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn target(this: &HtmlBaseElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLBaseElement", js_name = "target")]
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    pub fn set_target(this: &HtmlBaseElement, value: &str);
}
