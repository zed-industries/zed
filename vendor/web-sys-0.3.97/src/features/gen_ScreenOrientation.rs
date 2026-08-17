#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "ScreenOrientation",
        typescript_type = "ScreenOrientation"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenOrientation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub type ScreenOrientation;
    #[cfg(feature = "OrientationType")]
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ScreenOrientation",
        js_name = "type"
    )]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OrientationType`, `ScreenOrientation`*"]
    pub fn type_(this: &ScreenOrientation) -> Result<OrientationType, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "ScreenOrientation",
        js_name = "angle"
    )]
    #[doc = "Getter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn angle(this: &ScreenOrientation) -> Result<u16, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "ScreenOrientation", js_name = "onchange")]
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn onchange(this: &ScreenOrientation) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "ScreenOrientation", js_name = "onchange")]
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn set_onchange(this: &ScreenOrientation, value: Option<&::js_sys::Function>);
    #[cfg(feature = "OrientationLockType")]
    #[wasm_bindgen(catch, method, js_class = "ScreenOrientation")]
    #[doc = "The `lock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/lock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OrientationLockType`, `ScreenOrientation`*"]
    pub fn lock(
        this: &ScreenOrientation,
        orientation: OrientationLockType,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "ScreenOrientation")]
    #[doc = "The `unlock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/unlock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenOrientation`*"]
    pub fn unlock(this: &ScreenOrientation) -> Result<(), JsValue>;
}
