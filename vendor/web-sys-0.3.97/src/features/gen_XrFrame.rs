#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "XRFrame",
        typescript_type = "XRFrame"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrFrame;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[wasm_bindgen(method, getter, js_class = "XRFrame", js_name = "session")]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn session(this: &XrFrame) -> XrSession;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "XRFrame", js_name = "predictedDisplayTime")]
    #[doc = "Getter for the `predictedDisplayTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/predictedDisplayTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn predicted_display_time(this: &XrFrame) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrJointSpace")]
    #[wasm_bindgen(catch, method, js_class = "XRFrame", js_name = "fillJointRadii")]
    #[doc = "The `fillJointRadii()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillJointRadii)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrJointSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fill_joint_radii_with_f32_slice(
        this: &XrFrame,
        joint_spaces: &[XrJointSpace],
        radii: &mut [f32],
    ) -> Result<bool, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrJointSpace")]
    #[wasm_bindgen(catch, method, js_class = "XRFrame", js_name = "fillJointRadii")]
    #[doc = "The `fillJointRadii()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillJointRadii)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrJointSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fill_joint_radii_with_f32_array(
        this: &XrFrame,
        joint_spaces: &[XrJointSpace],
        radii: &::js_sys::Float32Array,
    ) -> Result<bool, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[wasm_bindgen(catch, method, js_class = "XRFrame", js_name = "fillPoses")]
    #[doc = "The `fillPoses()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillPoses)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fill_poses_with_f32_slice(
        this: &XrFrame,
        spaces: &[XrSpace],
        base_space: &XrSpace,
        transforms: &mut [f32],
    ) -> Result<bool, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[wasm_bindgen(catch, method, js_class = "XRFrame", js_name = "fillPoses")]
    #[doc = "The `fillPoses()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillPoses)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn fill_poses_with_f32_array(
        this: &XrFrame,
        spaces: &[XrSpace],
        base_space: &XrSpace,
        transforms: &::js_sys::Float32Array,
    ) -> Result<bool, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrJointPose", feature = "XrJointSpace", feature = "XrSpace",))]
    #[wasm_bindgen(method, js_class = "XRFrame", js_name = "getJointPose")]
    #[doc = "The `getJointPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getJointPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrJointPose`, `XrJointSpace`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_joint_pose(
        this: &XrFrame,
        joint: &XrJointSpace,
        base_space: &XrSpace,
    ) -> Option<XrJointPose>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrPose", feature = "XrSpace",))]
    #[wasm_bindgen(method, js_class = "XRFrame", js_name = "getPose")]
    #[doc = "The `getPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrPose`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_pose(this: &XrFrame, space: &XrSpace, base_space: &XrSpace) -> Option<XrPose>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrReferenceSpace", feature = "XrViewerPose",))]
    #[wasm_bindgen(method, js_class = "XRFrame", js_name = "getViewerPose")]
    #[doc = "The `getViewerPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getViewerPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrReferenceSpace`, `XrViewerPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_viewer_pose(
        this: &XrFrame,
        reference_space: &XrReferenceSpace,
    ) -> Option<XrViewerPose>;
}
