#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "MediaDevices",
        typescript_type = "MediaDevices"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaDevices` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub type MediaDevices;
    #[wasm_bindgen(method, getter, js_class = "MediaDevices", js_name = "ondevicechange")]
    #[doc = "Getter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn ondevicechange(this: &MediaDevices) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "MediaDevices", js_name = "ondevicechange")]
    #[doc = "Setter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn set_ondevicechange(this: &MediaDevices, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "MediaDevices", js_name = "enumerateDevices")]
    #[doc = "The `enumerateDevices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/enumerateDevices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn enumerate_devices(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "MediaDevices", js_name = "getDisplayMedia")]
    #[doc = "The `getDisplayMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn get_display_media(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "DisplayMediaStreamConstraints")]
    #[wasm_bindgen(catch, method, js_class = "MediaDevices", js_name = "getDisplayMedia")]
    #[doc = "The `getDisplayMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getDisplayMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`, `MediaDevices`*"]
    pub fn get_display_media_with_constraints(
        this: &MediaDevices,
        constraints: &DisplayMediaStreamConstraints,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaTrackSupportedConstraints")]
    #[wasm_bindgen(method, js_class = "MediaDevices", js_name = "getSupportedConstraints")]
    #[doc = "The `getSupportedConstraints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getSupportedConstraints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`, `MediaTrackSupportedConstraints`*"]
    pub fn get_supported_constraints(this: &MediaDevices) -> MediaTrackSupportedConstraints;
    #[wasm_bindgen(catch, method, js_class = "MediaDevices", js_name = "getUserMedia")]
    #[doc = "The `getUserMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`*"]
    pub fn get_user_media(this: &MediaDevices) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(feature = "MediaStreamConstraints")]
    #[wasm_bindgen(catch, method, js_class = "MediaDevices", js_name = "getUserMedia")]
    #[doc = "The `getUserMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaDevices`, `MediaStreamConstraints`*"]
    pub fn get_user_media_with_constraints(
        this: &MediaDevices,
        constraints: &MediaStreamConstraints,
    ) -> Result<::js_sys::Promise, JsValue>;
}
