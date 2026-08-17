#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "BeforeUnloadEvent",
        typescript_type = "BeforeUnloadEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BeforeUnloadEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    pub type BeforeUnloadEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BeforeUnloadEvent",
        js_name = "returnValue"
    )]
    #[doc = "Getter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    pub fn return_value(this: &BeforeUnloadEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BeforeUnloadEvent",
        js_name = "returnValue"
    )]
    #[doc = "Setter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
    pub fn set_return_value(this: &BeforeUnloadEvent, value: &str);
}
