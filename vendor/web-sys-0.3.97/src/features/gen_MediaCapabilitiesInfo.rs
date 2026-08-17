#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "MediaCapabilitiesInfo",
        typescript_type = "MediaCapabilitiesInfo"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaCapabilitiesInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub type MediaCapabilitiesInfo;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaCapabilitiesInfo",
        js_name = "supported"
    )]
    #[doc = "Getter for the `supported` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/supported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub fn supported(this: &MediaCapabilitiesInfo) -> bool;
    #[wasm_bindgen(method, getter, js_class = "MediaCapabilitiesInfo", js_name = "smooth")]
    #[doc = "Getter for the `smooth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/smooth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub fn smooth(this: &MediaCapabilitiesInfo) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "MediaCapabilitiesInfo",
        js_name = "powerEfficient"
    )]
    #[doc = "Getter for the `powerEfficient` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilitiesInfo/powerEfficient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilitiesInfo`*"]
    pub fn power_efficient(this: &MediaCapabilitiesInfo) -> bool;
}
