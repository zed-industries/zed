#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRFrameData",
        typescript_type = "VRFrameData"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrFrameData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub type VrFrameData;
    #[wasm_bindgen(method, getter, js_class = "VRFrameData", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn timestamp(this: &VrFrameData) -> f64;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRFrameData",
        js_name = "leftProjectionMatrix"
    )]
    #[doc = "Getter for the `leftProjectionMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftProjectionMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn left_projection_matrix(this: &VrFrameData) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRFrameData",
        js_name = "leftViewMatrix"
    )]
    #[doc = "Getter for the `leftViewMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftViewMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn left_view_matrix(this: &VrFrameData) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRFrameData",
        js_name = "rightProjectionMatrix"
    )]
    #[doc = "Getter for the `rightProjectionMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightProjectionMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn right_projection_matrix(this: &VrFrameData) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRFrameData",
        js_name = "rightViewMatrix"
    )]
    #[doc = "Getter for the `rightViewMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightViewMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn right_view_matrix(this: &VrFrameData) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[cfg(feature = "VrPose")]
    #[wasm_bindgen(method, getter, js_class = "VRFrameData", js_name = "pose")]
    #[doc = "Getter for the `pose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/pose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`, `VrPose`*"]
    pub fn pose(this: &VrFrameData) -> VrPose;
    #[wasm_bindgen(catch, constructor, js_class = "VRFrameData")]
    #[doc = "The `new VrFrameData(..)` constructor, creating a new instance of `VrFrameData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/VRFrameData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
    pub fn new() -> Result<VrFrameData, JsValue>;
}
