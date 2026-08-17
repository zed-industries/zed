#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRStageParameters",
        typescript_type = "VRStageParameters"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrStageParameters` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
    pub type VrStageParameters;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "VRStageParameters",
        js_name = "sittingToStandingTransform"
    )]
    #[doc = "Getter for the `sittingToStandingTransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sittingToStandingTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
    pub fn sitting_to_standing_transform(
        this: &VrStageParameters,
    ) -> Result<::alloc::vec::Vec<f32>, JsValue>;
    #[wasm_bindgen(method, getter, js_class = "VRStageParameters", js_name = "sizeX")]
    #[doc = "Getter for the `sizeX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
    pub fn size_x(this: &VrStageParameters) -> f32;
    #[wasm_bindgen(method, getter, js_class = "VRStageParameters", js_name = "sizeZ")]
    #[doc = "Getter for the `sizeZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRStageParameters/sizeZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrStageParameters`*"]
    pub fn size_z(this: &VrStageParameters) -> f32;
}
