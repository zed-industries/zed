#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRPose",
        typescript_type = "VRPose"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrPose` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub type VrPose;
    #[wasm_bindgen(catch, method, getter, js_class = "VRPose", js_name = "position")]
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub fn position(this: &VrPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "VRPose", js_name = "linearVelocity")]
    #[doc = "Getter for the `linearVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub fn linear_velocity(this: &VrPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRPose",
        js_name = "linearAcceleration"
    )]
    #[doc = "Getter for the `linearAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub fn linear_acceleration(this: &VrPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(catch, method, getter, js_class = "VRPose", js_name = "orientation")]
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub fn orientation(this: &VrPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRPose",
        js_name = "angularVelocity"
    )]
    #[doc = "Getter for the `angularVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub fn angular_velocity(this: &VrPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRPose",
        js_name = "angularAcceleration"
    )]
    #[doc = "Getter for the `angularAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
    pub fn angular_acceleration(this: &VrPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
}
