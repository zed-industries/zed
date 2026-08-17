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
        js_name = "HTMLSlotElement",
        typescript_type = "HTMLSlotElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSlotElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub type HtmlSlotElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLSlotElement", js_name = "name")]
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub fn name(this: &HtmlSlotElement) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLSlotElement", js_name = "name")]
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub fn set_name(this: &HtmlSlotElement, value: &str);
    #[wasm_bindgen(method, js_class = "HTMLSlotElement", js_name = "assignedNodes")]
    #[doc = "The `assignedNodes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
    pub fn assigned_nodes(this: &HtmlSlotElement) -> ::js_sys::Array;
    #[cfg(feature = "AssignedNodesOptions")]
    #[wasm_bindgen(method, js_class = "HTMLSlotElement", js_name = "assignedNodes")]
    #[doc = "The `assignedNodes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`, `HtmlSlotElement`*"]
    pub fn assigned_nodes_with_options(
        this: &HtmlSlotElement,
        options: &AssignedNodesOptions,
    ) -> ::js_sys::Array;
}
