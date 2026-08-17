#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "ToggleEvent",
        typescript_type = "ToggleEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ToggleEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
    pub type ToggleEvent;
    #[wasm_bindgen(method, getter, js_class = "ToggleEvent", js_name = "oldState")]
    #[doc = "Getter for the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/oldState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
    pub fn old_state(this: &ToggleEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "ToggleEvent", js_name = "newState")]
    #[doc = "Getter for the `newState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/newState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
    pub fn new_state(this: &ToggleEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "ToggleEvent")]
    #[doc = "The `new ToggleEvent(..)` constructor, creating a new instance of `ToggleEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/ToggleEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
    pub fn new(type_: &str) -> Result<ToggleEvent, JsValue>;
    #[cfg(feature = "ToggleEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "ToggleEvent")]
    #[doc = "The `new ToggleEvent(..)` constructor, creating a new instance of `ToggleEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/ToggleEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`, `ToggleEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &ToggleEventInit,
    ) -> Result<ToggleEvent, JsValue>;
}
