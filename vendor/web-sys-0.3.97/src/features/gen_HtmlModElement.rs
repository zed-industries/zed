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
        js_name = "HTMLModElement",
        typescript_type = "HTMLModElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlModElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub type HtmlModElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLModElement", js_name = "cite")]
    #[doc = "Getter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn cite(this: &HtmlModElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLModElement", js_name = "cite")]
    #[doc = "Setter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn set_cite(this: &HtmlModElement, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLModElement", js_name = "dateTime")]
    #[doc = "Getter for the `dateTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn date_time(this: &HtmlModElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLModElement", js_name = "dateTime")]
    #[doc = "Setter for the `dateTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLModElement/dateTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlModElement`*"]
    pub fn set_date_time(this: &HtmlModElement, value: &str);
}
