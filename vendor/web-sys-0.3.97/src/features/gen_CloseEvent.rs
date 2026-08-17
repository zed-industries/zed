#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "CloseEvent",
        typescript_type = "CloseEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub type CloseEvent;
    #[wasm_bindgen(method, getter, js_class = "CloseEvent", js_name = "wasClean")]
    #[doc = "Getter for the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn was_clean(this: &CloseEvent) -> bool;
    #[wasm_bindgen(method, getter, js_class = "CloseEvent", js_name = "code")]
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn code(this: &CloseEvent) -> u16;
    #[wasm_bindgen(method, getter, js_class = "CloseEvent", js_name = "reason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn reason(this: &CloseEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "CloseEvent")]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
    pub fn new(type_: &str) -> Result<CloseEvent, JsValue>;
    #[cfg(feature = "CloseEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "CloseEvent")]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`, `CloseEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &CloseEventInit,
    ) -> Result<CloseEvent, JsValue>;
}
