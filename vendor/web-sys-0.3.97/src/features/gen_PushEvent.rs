#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "ExtendableEvent",
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PushEvent",
        typescript_type = "PushEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`*"]
    pub type PushEvent;
    #[cfg(feature = "PushMessageData")]
    #[wasm_bindgen(method, getter, js_class = "PushEvent", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`, `PushMessageData`*"]
    pub fn data(this: &PushEvent) -> Option<PushMessageData>;
    #[wasm_bindgen(catch, constructor, js_class = "PushEvent")]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`*"]
    pub fn new(type_: &str) -> Result<PushEvent, JsValue>;
    #[cfg(feature = "PushEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PushEvent")]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`, `PushEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PushEventInit,
    ) -> Result<PushEvent, JsValue>;
}
