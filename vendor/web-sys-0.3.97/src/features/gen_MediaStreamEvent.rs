#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaStreamEvent",
        typescript_type = "MediaStreamEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEvent`*"]
    pub type MediaStreamEvent;
    #[cfg(feature = "MediaStream")]
    #[wasm_bindgen(method, getter, js_class = "MediaStreamEvent", js_name = "stream")]
    #[doc = "Getter for the `stream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEvent`*"]
    pub fn stream(this: &MediaStreamEvent) -> Option<MediaStream>;
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamEvent")]
    #[doc = "The `new MediaStreamEvent(..)` constructor, creating a new instance of `MediaStreamEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/MediaStreamEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEvent`*"]
    pub fn new(type_: &str) -> Result<MediaStreamEvent, JsValue>;
    #[cfg(feature = "MediaStreamEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamEvent")]
    #[doc = "The `new MediaStreamEvent(..)` constructor, creating a new instance of `MediaStreamEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/MediaStreamEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEvent`, `MediaStreamEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaStreamEventInit,
    ) -> Result<MediaStreamEvent, JsValue>;
}
