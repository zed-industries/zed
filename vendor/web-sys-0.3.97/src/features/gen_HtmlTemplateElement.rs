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
        js_name = "HTMLTemplateElement",
        typescript_type = "HTMLTemplateElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTemplateElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTemplateElement`*"]
    pub type HtmlTemplateElement;
    #[cfg(feature = "DocumentFragment")]
    #[wasm_bindgen(method, getter, js_class = "HTMLTemplateElement", js_name = "content")]
    #[doc = "Getter for the `content` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/content)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlTemplateElement`*"]
    pub fn content(this: &HtmlTemplateElement) -> DocumentFragment;
}
