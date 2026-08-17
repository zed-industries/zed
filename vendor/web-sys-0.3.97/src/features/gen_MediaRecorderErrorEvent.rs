#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaRecorderErrorEvent",
        typescript_type = "MediaRecorderErrorEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`*"]
    pub type MediaRecorderErrorEvent;
    #[cfg(feature = "DomException")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaRecorderErrorEvent",
        js_name = "error"
    )]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEvent`*"]
    pub fn error(this: &MediaRecorderErrorEvent) -> DomException;
    #[cfg(feature = "MediaRecorderErrorEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaRecorderErrorEvent")]
    #[doc = "The `new MediaRecorderErrorEvent(..)` constructor, creating a new instance of `MediaRecorderErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/MediaRecorderErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`, `MediaRecorderErrorEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &MediaRecorderErrorEventInit,
    ) -> Result<MediaRecorderErrorEvent, JsValue>;
}
