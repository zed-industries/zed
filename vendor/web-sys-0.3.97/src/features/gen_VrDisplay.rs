#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "VRDisplay",
        typescript_type = "VRDisplay"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrDisplay` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub type VrDisplay;
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "isConnected")]
    #[doc = "Getter for the `isConnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isConnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn is_connected(this: &VrDisplay) -> bool;
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "isPresenting")]
    #[doc = "Getter for the `isPresenting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isPresenting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn is_presenting(this: &VrDisplay) -> bool;
    #[cfg(feature = "VrDisplayCapabilities")]
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "capabilities")]
    #[doc = "Getter for the `capabilities` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/capabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrDisplayCapabilities`*"]
    pub fn capabilities(this: &VrDisplay) -> VrDisplayCapabilities;
    #[cfg(feature = "VrStageParameters")]
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "stageParameters")]
    #[doc = "Getter for the `stageParameters` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/stageParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrStageParameters`*"]
    pub fn stage_parameters(this: &VrDisplay) -> Option<VrStageParameters>;
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "displayId")]
    #[doc = "Getter for the `displayId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn display_id(this: &VrDisplay) -> u32;
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "displayName")]
    #[doc = "Getter for the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn display_name(this: &VrDisplay) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "depthNear")]
    #[doc = "Getter for the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn depth_near(this: &VrDisplay) -> f64;
    #[wasm_bindgen(method, setter, js_class = "VRDisplay", js_name = "depthNear")]
    #[doc = "Setter for the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn set_depth_near(this: &VrDisplay, value: f64);
    #[wasm_bindgen(method, getter, js_class = "VRDisplay", js_name = "depthFar")]
    #[doc = "Getter for the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn depth_far(this: &VrDisplay) -> f64;
    #[wasm_bindgen(method, setter, js_class = "VRDisplay", js_name = "depthFar")]
    #[doc = "Setter for the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn set_depth_far(this: &VrDisplay, value: f64);
    #[wasm_bindgen(
        catch,
        method,
        js_class = "VRDisplay",
        js_name = "cancelAnimationFrame"
    )]
    #[doc = "The `cancelAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/cancelAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn cancel_animation_frame(this: &VrDisplay, handle: i32) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "VRDisplay", js_name = "exitPresent")]
    #[doc = "The `exitPresent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/exitPresent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn exit_present(this: &VrDisplay) -> Result<::js_sys::Promise, JsValue>;
    #[cfg(all(feature = "VrEye", feature = "VrEyeParameters",))]
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "getEyeParameters")]
    #[doc = "The `getEyeParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getEyeParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrEye`, `VrEyeParameters`*"]
    pub fn get_eye_parameters(this: &VrDisplay, which_eye: VrEye) -> VrEyeParameters;
    #[cfg(feature = "VrFrameData")]
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "getFrameData")]
    #[doc = "The `getFrameData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getFrameData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrFrameData`*"]
    pub fn get_frame_data(this: &VrDisplay, frame_data: &VrFrameData) -> bool;
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "getLayers")]
    #[doc = "The `getLayers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn get_layers(this: &VrDisplay) -> ::js_sys::Array;
    #[cfg(feature = "VrPose")]
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "getPose")]
    #[doc = "The `getPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrPose`*"]
    pub fn get_pose(this: &VrDisplay) -> VrPose;
    #[cfg(feature = "VrSubmitFrameResult")]
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "getSubmitFrameResult")]
    #[doc = "The `getSubmitFrameResult()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getSubmitFrameResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrSubmitFrameResult`*"]
    pub fn get_submit_frame_result(this: &VrDisplay, result: &VrSubmitFrameResult) -> bool;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "VRDisplay",
        js_name = "requestAnimationFrame"
    )]
    #[doc = "The `requestAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn request_animation_frame(
        this: &VrDisplay,
        callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "VRDisplay", js_name = "requestPresent")]
    #[doc = "The `requestPresent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestPresent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn request_present(
        this: &VrDisplay,
        layers: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "resetPose")]
    #[doc = "The `resetPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/resetPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn reset_pose(this: &VrDisplay);
    #[wasm_bindgen(method, js_class = "VRDisplay", js_name = "submitFrame")]
    #[doc = "The `submitFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/submitFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
    pub fn submit_frame(this: &VrDisplay);
}
