#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PresentationConnectionCloseEvent",
        typescript_type = "PresentationConnectionCloseEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionCloseEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
    pub type PresentationConnectionCloseEvent;
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnectionCloseEvent",
        js_name = "reason"
    )]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionClosedReason`*"]
    pub fn reason(this: &PresentationConnectionCloseEvent) -> PresentationConnectionClosedReason;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PresentationConnectionCloseEvent",
        js_name = "message"
    )]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
    pub fn message(this: &PresentationConnectionCloseEvent) -> ::alloc::string::String;
    #[cfg(feature = "PresentationConnectionCloseEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PresentationConnectionCloseEvent")]
    #[doc = "The `new PresentationConnectionCloseEvent(..)` constructor, creating a new instance of `PresentationConnectionCloseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/PresentationConnectionCloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionCloseEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &PresentationConnectionCloseEventInit,
    ) -> Result<PresentationConnectionCloseEvent, JsValue>;
}
