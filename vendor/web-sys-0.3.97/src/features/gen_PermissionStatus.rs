#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "PermissionStatus",
        typescript_type = "PermissionStatus"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PermissionStatus` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionStatus`*"]
    pub type PermissionStatus;
    #[cfg(feature = "PermissionState")]
    #[wasm_bindgen(method, getter, js_class = "PermissionStatus", js_name = "state")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionState`, `PermissionStatus`*"]
    pub fn state(this: &PermissionStatus) -> PermissionState;
    #[wasm_bindgen(method, getter, js_class = "PermissionStatus", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionStatus`*"]
    pub fn onchange(this: &PermissionStatus) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "PermissionStatus", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionStatus`*"]
    pub fn set_onchange(this: &PermissionStatus, value: Option<&::js_sys::Function>);
}
