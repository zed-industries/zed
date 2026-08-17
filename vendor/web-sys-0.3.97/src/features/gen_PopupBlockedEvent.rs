#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Event",
        extends = "::js_sys::Object",
        js_name = "PopupBlockedEvent",
        typescript_type = "PopupBlockedEvent"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopupBlockedEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub type PopupBlockedEvent;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "requestingWindow"
    )]
    #[doc = "Getter for the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/requestingWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`, `Window`*"]
    pub fn requesting_window(this: &PopupBlockedEvent) -> Option<Window>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowURI"
    )]
    #[doc = "Getter for the `popupWindowURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_uri(this: &PopupBlockedEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowName"
    )]
    #[doc = "Getter for the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_name(this: &PopupBlockedEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "PopupBlockedEvent",
        js_name = "popupWindowFeatures"
    )]
    #[doc = "Getter for the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn popup_window_features(this: &PopupBlockedEvent) -> Option<::alloc::string::String>;
    #[wasm_bindgen(catch, constructor, js_class = "PopupBlockedEvent")]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
    pub fn new(type_: &str) -> Result<PopupBlockedEvent, JsValue>;
    #[cfg(feature = "PopupBlockedEventInit")]
    #[wasm_bindgen(catch, constructor, js_class = "PopupBlockedEvent")]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`, `PopupBlockedEventInit`*"]
    pub fn new_with_event_init_dict(
        type_: &str,
        event_init_dict: &PopupBlockedEventInit,
    ) -> Result<PopupBlockedEvent, JsValue>;
}
