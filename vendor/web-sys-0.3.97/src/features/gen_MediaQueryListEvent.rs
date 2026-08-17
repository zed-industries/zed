#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaQueryListEvent",
        typescript_type = "MediaQueryListEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryListEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub type MediaQueryListEvent;
    #[wasm_bindgen(method, getter, js_class = "MediaQueryListEvent", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub fn media(this: &MediaQueryListEvent) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaQueryListEvent", js_name = "matches")]
    #[doc = "Getter for the `matches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/matches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub fn matches(this: &MediaQueryListEvent) -> bool;
    #[wasm_bindgen(catch, constructor, js_class = "MediaQueryListEvent")]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
    pub fn new(type_: &str) -> Result<MediaQueryListEvent, JsValue>;
    #[cfg(feature = "MediaQueryListEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaQueryListEvent")]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`, `MediaQueryListEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaQueryListEventInit,
    ) -> Result<MediaQueryListEvent, JsValue>;
}
