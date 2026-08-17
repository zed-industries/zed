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
        js_name = "HTMLProgressElement",
        typescript_type = "HTMLProgressElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlProgressElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub type HtmlProgressElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLProgressElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn value(this: &HtmlProgressElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLProgressElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn set_value(this: &HtmlProgressElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLProgressElement", js_name = "max")]
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn max(this: &HtmlProgressElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLProgressElement", js_name = "max")]
    #[doc = "Setter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn set_max(this: &HtmlProgressElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLProgressElement", js_name = "position")]
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`*"]
    pub fn position(this: &HtmlProgressElement) -> f64;
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLProgressElement", js_name = "labels")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlProgressElement`, `NodeList`*"]
    pub fn labels(this: &HtmlProgressElement) -> NodeList;
}
