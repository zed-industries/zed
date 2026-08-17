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
        js_name = "HTMLLabelElement",
        typescript_type = "HTMLLabelElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLabelElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub type HtmlLabelElement;
    #[cfg(feature = "HtmlFormElement")]
    #[wasm_bindgen(method, getter, js_class = "HTMLLabelElement", js_name = "form")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlLabelElement`*"]
    pub fn form(this: &HtmlLabelElement) -> Option<HtmlFormElement>;
    #[wasm_bindgen(method, getter, js_class = "HTMLLabelElement", js_name = "htmlFor")]
    #[doc = "Getter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub fn html_for(this: &HtmlLabelElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLLabelElement", js_name = "htmlFor")]
    #[doc = "Setter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub fn set_html_for(this: &HtmlLabelElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLLabelElement", js_name = "control")]
    #[doc = "Getter for the `control` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    pub fn control(this: &HtmlLabelElement) -> Option<HtmlElement>;
}
