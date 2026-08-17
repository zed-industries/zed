#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "GamepadPose",
        typescript_type = "GamepadPose"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadPose` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub type GamepadPose;
    #[wasm_bindgen(method, getter, js_class = "GamepadPose", js_name = "hasOrientation")]
    #[doc = "Getter for the `hasOrientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn has_orientation(this: &GamepadPose) -> bool;
    #[wasm_bindgen(method, getter, js_class = "GamepadPose", js_name = "hasPosition")]
    #[doc = "Getter for the `hasPosition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn has_position(this: &GamepadPose) -> bool;
    #[wasm_bindgen(catch, method, getter, js_class = "GamepadPose", js_name = "position")]
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn position(this: &GamepadPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "GamepadPose",
        js_name = "linearVelocity"
    )]
    #[doc = "Getter for the `linearVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn linear_velocity(this: &GamepadPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "GamepadPose",
        js_name = "linearAcceleration"
    )]
    #[doc = "Getter for the `linearAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn linear_acceleration(
        this: &GamepadPose,
    ) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "GamepadPose",
        js_name = "orientation"
    )]
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn orientation(this: &GamepadPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "GamepadPose",
        js_name = "angularVelocity"
    )]
    #[doc = "Getter for the `angularVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn angular_velocity(this: &GamepadPose) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "GamepadPose",
        js_name = "angularAcceleration"
    )]
    #[doc = "Getter for the `angularAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
    pub fn angular_acceleration(
        this: &GamepadPose,
    ) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue>;
}
