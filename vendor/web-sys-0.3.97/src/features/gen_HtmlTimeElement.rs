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
        js_name = "HTMLTimeElement",
        typescript_type = "HTMLTimeElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTimeElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
    pub type HtmlTimeElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLTimeElement", js_name = "dateTime")]
    #[doc = "Getter for the `dateTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
    pub fn date_time(this: &HtmlTimeElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLTimeElement", js_name = "dateTime")]
    #[doc = "Setter for the `dateTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
    pub fn set_date_time(this: &HtmlTimeElement, value: &str);
}
