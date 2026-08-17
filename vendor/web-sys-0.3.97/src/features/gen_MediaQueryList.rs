#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaQueryList",
        typescript_type = "MediaQueryList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub type MediaQueryList;
    #[wasm_bindgen(method, getter, js_class = "MediaQueryList", js_name = "media")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub fn media(this: &MediaQueryList) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "MediaQueryList", js_name = "matches")]
    #[doc = "Getter for the `matches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/matches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub fn matches(this: &MediaQueryList) -> bool;
    #[wasm_bindgen(method, getter, js_class = "MediaQueryList", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub fn onchange(this: &MediaQueryList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaQueryList", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub fn set_onchange(this: &MediaQueryList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "MediaQueryList", js_name = "addListener")]
    #[doc = "The `addListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub fn add_listener_with_opt_callback(
        this: &MediaQueryList,
        listener: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(catch, method, js_class = "MediaQueryList", js_name = "addListener")]
    #[doc = "The `addListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `MediaQueryList`*"]
    pub fn add_listener_with_opt_event_listener(
        this: &MediaQueryList,
        listener: Option<&EventListener>,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaQueryList", js_name = "removeListener")]
    #[doc = "The `removeListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`*"]
    pub fn remove_listener_with_opt_callback(
        this: &MediaQueryList,
        listener: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;
    #[cfg(feature = "EventListener")]
    #[wasm_bindgen(catch, method, js_class = "MediaQueryList", js_name = "removeListener")]
    #[doc = "The `removeListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `MediaQueryList`*"]
    pub fn remove_listener_with_opt_event_listener(
        this: &MediaQueryList,
        listener: Option<&EventListener>,
    ) -> Result<(), JsValue>;
}
