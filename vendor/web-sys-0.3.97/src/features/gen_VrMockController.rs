#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRMockController",
        typescript_type = "VRMockController"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrMockController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub type VrMockController;
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newAxisMoveEvent")]
    #[doc = "The `newAxisMoveEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newAxisMoveEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_axis_move_event(this: &VrMockController, axis: u32, value: f64);
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newButtonEvent")]
    #[doc = "The `newButtonEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newButtonEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_button_event(this: &VrMockController, button: u32, pressed: bool);
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&mut [f32]>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&mut [f32]>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&mut [f32]>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&mut [f32]>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&mut [f32]>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&mut [f32]>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
    #[wasm_bindgen(method, js_class = "VRMockController", js_name = "newPoseMove")]
    #[doc = "The `newPoseMove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockController/newPoseMove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockController`*"]
    pub fn new_pose_move_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(
        this: &VrMockController,
        position: Option<&::js_sys::Float32Array>,
        linear_velocity: Option<&::js_sys::Float32Array>,
        linear_acceleration: Option<&::js_sys::Float32Array>,
        orientation: Option<&::js_sys::Float32Array>,
        angular_velocity: Option<&::js_sys::Float32Array>,
        angular_acceleration: Option<&::js_sys::Float32Array>,
    );
}
