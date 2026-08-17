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
        js_name = "HTMLMeterElement",
        typescript_type = "HTMLMeterElement"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMeterElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub type HtmlMeterElement;
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "value")]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn value(this: &HtmlMeterElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMeterElement", js_name = "value")]
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_value(this: &HtmlMeterElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "min")]
    #[doc = "Getter for the `min` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn min(this: &HtmlMeterElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMeterElement", js_name = "min")]
    #[doc = "Setter for the `min` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_min(this: &HtmlMeterElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "max")]
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn max(this: &HtmlMeterElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMeterElement", js_name = "max")]
    #[doc = "Setter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_max(this: &HtmlMeterElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "low")]
    #[doc = "Getter for the `low` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn low(this: &HtmlMeterElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMeterElement", js_name = "low")]
    #[doc = "Setter for the `low` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_low(this: &HtmlMeterElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "high")]
    #[doc = "Getter for the `high` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn high(this: &HtmlMeterElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMeterElement", js_name = "high")]
    #[doc = "Setter for the `high` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_high(this: &HtmlMeterElement, value: f64);
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "optimum")]
    #[doc = "Getter for the `optimum` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn optimum(this: &HtmlMeterElement) -> f64;
    #[wasm_bindgen(method, setter, js_class = "HTMLMeterElement", js_name = "optimum")]
    #[doc = "Setter for the `optimum` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    pub fn set_optimum(this: &HtmlMeterElement, value: f64);
    #[cfg(feature = "NodeList")]
    #[wasm_bindgen(method, getter, js_class = "HTMLMeterElement", js_name = "labels")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMeterElement`, `NodeList`*"]
    pub fn labels(this: &HtmlMeterElement) -> NodeList;
}
