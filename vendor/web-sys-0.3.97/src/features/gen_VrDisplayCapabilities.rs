#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRDisplayCapabilities",
        typescript_type = "VRDisplayCapabilities"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrDisplayCapabilities` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    pub type VrDisplayCapabilities;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VRDisplayCapabilities",
        js_name = "hasPosition"
    )]
    #[doc = "Getter for the `hasPosition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    pub fn has_position(this: &VrDisplayCapabilities) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VRDisplayCapabilities",
        js_name = "hasOrientation"
    )]
    #[doc = "Getter for the `hasOrientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    pub fn has_orientation(this: &VrDisplayCapabilities) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VRDisplayCapabilities",
        js_name = "hasExternalDisplay"
    )]
    #[doc = "Getter for the `hasExternalDisplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/hasExternalDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    pub fn has_external_display(this: &VrDisplayCapabilities) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VRDisplayCapabilities",
        js_name = "canPresent"
    )]
    #[doc = "Getter for the `canPresent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/canPresent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    pub fn can_present(this: &VrDisplayCapabilities) -> bool;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VRDisplayCapabilities",
        js_name = "maxLayers"
    )]
    #[doc = "Getter for the `maxLayers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplayCapabilities/maxLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplayCapabilities`*"]
    pub fn max_layers(this: &VrDisplayCapabilities) -> u32;
}
