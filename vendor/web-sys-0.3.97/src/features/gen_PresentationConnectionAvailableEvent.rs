#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PresentationConnectionAvailableEvent",
        typescript_type = "PresentationConnectionAvailableEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionAvailableEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`*"]
    pub type PresentationConnectionAvailableEvent;
    #[cfg(feature = "PresentationConnection")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnectionAvailableEvent",
        js_name = "connection"
    )]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEvent`*"]
    pub fn connection(this: &PresentationConnectionAvailableEvent) -> PresentationConnection;
    #[cfg(feature = "PresentationConnectionAvailableEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PresentationConnectionAvailableEvent")]
    #[doc = "The `new PresentationConnectionAvailableEvent(..)` constructor, creating a new instance of `PresentationConnectionAvailableEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/PresentationConnectionAvailableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`, `PresentationConnectionAvailableEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &PresentationConnectionAvailableEventInit,
    ) -> Result<PresentationConnectionAvailableEvent, JsValue>;
}
