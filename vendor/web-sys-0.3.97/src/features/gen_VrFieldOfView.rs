#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRFieldOfView",
        typescript_type = "VRFieldOfView"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrFieldOfView` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub type VrFieldOfView;
    #[wasm_bindgen(method, getter, js_class = "VRFieldOfView", js_name = "upDegrees")]
    #[doc = "Getter for the `upDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/upDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn up_degrees(this: &VrFieldOfView) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VRFieldOfView", js_name = "rightDegrees")]
    #[doc = "Getter for the `rightDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/rightDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn right_degrees(this: &VrFieldOfView) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VRFieldOfView", js_name = "downDegrees")]
    #[doc = "Getter for the `downDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/downDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn down_degrees(this: &VrFieldOfView) -> f64;
    #[wasm_bindgen(method, getter, js_class = "VRFieldOfView", js_name = "leftDegrees")]
    #[doc = "Getter for the `leftDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/leftDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
    pub fn left_degrees(this: &VrFieldOfView) -> f64;
}
