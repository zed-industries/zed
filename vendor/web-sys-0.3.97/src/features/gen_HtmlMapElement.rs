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
        js_name = "HTMLMapElement",
        typescript_type = "HTMLMapElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMapElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMapElement`*"]
    pub type HtmlMapElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLMapElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMapElement`*"]
    pub fn name(this: &HtmlMapElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLMapElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMapElement`*"]
    pub fn set_name(this: &HtmlMapElement, value: &str);
    #[cfg(feature = "HtmlCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMapElement", js_name = "areas")]
    #[doc = "Getter for the `areas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/areas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlMapElement`*"]
    pub fn areas(this: &HtmlMapElement) -> HtmlCollection;
}
