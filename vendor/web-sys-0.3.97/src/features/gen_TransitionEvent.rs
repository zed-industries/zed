#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "TransitionEvent",
        typescript_type = "TransitionEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransitionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub type TransitionEvent;
    #[wasm_bindgen(method, getter, js_class = "TransitionEvent", js_name = "propertyName")]
    #[doc = "Getter for the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/propertyName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn property_name(this: &TransitionEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "TransitionEvent", js_name = "elapsedTime")]
    #[doc = "Getter for the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/elapsedTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn elapsed_time(this: &TransitionEvent) -> f32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "TransitionEvent",
        js_name = "pseudoElement"
    )]
    #[doc = "Getter for the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/pseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn pseudo_element(this: &TransitionEvent) -> ::alloc::string::String;
    #[wasm_bindgen(catch, constructor, js_class = "TransitionEvent")]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`*"]
    pub fn new(type_: &str) -> Result<TransitionEvent, JsValue>;
    #[cfg(feature = "TransitionEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "TransitionEvent")]
    #[doc = "The `new TransitionEvent(..)` constructor, creating a new instance of `TransitionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/TransitionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEvent`, `TransitionEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &TransitionEventInit,
    ) -> Result<TransitionEvent, JsValue>;
}
