#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "PresentationAvailability",
        typescript_type = "PresentationAvailability"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationAvailability` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub type PresentationAvailability;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationAvailability",
        js_name = "value"
    )]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub fn value(this: &PresentationAvailability) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationAvailability",
        js_name = "onchange"
    )]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub fn onchange(this: &PresentationAvailability) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "PresentationAvailability",
        js_name = "onchange"
    )]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
    pub fn set_onchange(this: &PresentationAvailability, value: Option<&::js_sys::Function>);
}
