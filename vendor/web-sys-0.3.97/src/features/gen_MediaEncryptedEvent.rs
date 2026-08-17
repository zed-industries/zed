#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "MediaEncryptedEvent",
        typescript_type = "MediaEncryptedEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaEncryptedEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub type MediaEncryptedEvent;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaEncryptedEvent",
        js_name = "initDataType"
    )]
    #[doc = "Getter for the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initDataType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub fn init_data_type(this: &MediaEncryptedEvent) -> ::alloc::string::String;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "MediaEncryptedEvent",
        js_name = "initData"
    )]
    #[doc = "Getter for the `initData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub fn init_data(this: &MediaEncryptedEvent) -> Result<Option<::js_sys::ArrayBuffer>, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "MediaEncryptedEvent")]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
    pub fn new(type_: &str) -> Result<MediaEncryptedEvent, JsValue>;
    #[cfg(feature = "MediaKeyNeededEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "MediaEncryptedEvent")]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`, `MediaKeyNeededEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &MediaKeyNeededEventInit,
    ) -> Result<MediaEncryptedEvent, JsValue>;
}
