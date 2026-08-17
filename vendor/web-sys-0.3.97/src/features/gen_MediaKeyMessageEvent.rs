#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaKeyMessageEvent",
        typescript_type = "MediaKeyMessageEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyMessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
    pub type MediaKeyMessageEvent;
    #[cfg(feature = "MediaKeyMessageType")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaKeyMessageEvent",
        js_name = "messageType"
    )]
    #[doc = "Getter for the `messageType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/messageType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageType`*"]
    pub fn message_type(this: &MediaKeyMessageEvent) -> MediaKeyMessageType;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "MediaKeyMessageEvent",
        js_name = "message"
    )]
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
    pub fn message(this: &MediaKeyMessageEvent) -> Result<::js_sys::ArrayBuffer, JsValue>;
    #[cfg(feature = "MediaKeyMessageEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaKeyMessageEvent")]
    #[doc = "The `new MediaKeyMessageEvent(..)` constructor, creating a new instance of `MediaKeyMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/MediaKeyMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageEventInit`*"]
    pub fn new(
        type_: &str,
        event_init_dict: &MediaKeyMessageEventInit,
    ) -> Result<MediaKeyMessageEvent, JsValue>;
}
