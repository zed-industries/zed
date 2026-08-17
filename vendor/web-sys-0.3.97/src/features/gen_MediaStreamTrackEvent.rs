#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaStreamTrackEvent",
        typescript_type = "MediaStreamTrackEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEvent`*"]
    pub type MediaStreamTrackEvent;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(method, getter, js_class = "MediaStreamTrackEvent", js_name = "track")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEvent`*"]
    pub fn track(this: &MediaStreamTrackEvent) -> MediaStreamTrack;
    #[cfg(feature = "MediaStreamTrackEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaStreamTrackEvent")]
    #[doc = "The `new MediaStreamTrackEvent(..)` constructor, creating a new instance of `MediaStreamTrackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/MediaStreamTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEvent`, `MediaStreamTrackEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &MediaStreamTrackEventInit,
    ) -> Result<MediaStreamTrackEvent, JsValue>;
}
