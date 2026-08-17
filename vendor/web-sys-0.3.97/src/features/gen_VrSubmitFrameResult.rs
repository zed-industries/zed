#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "VRSubmitFrameResult",
        typescript_type = "VRSubmitFrameResult"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrSubmitFrameResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub type VrSubmitFrameResult;
    #[wasm_bindgen(method, getter, js_class = "VRSubmitFrameResult", js_name = "frameNum")]
    #[doc = "Getter for the `frameNum` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/frameNum)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub fn frame_num(this: &VrSubmitFrameResult) -> u32;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "VRSubmitFrameResult",
        js_name = "base64Image"
    )]
    #[doc = "Getter for the `base64Image` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/base64Image)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub fn base64_image(this: &VrSubmitFrameResult) -> Option<::alloc::string::String>;
    #[wasm_bindgen(catch, constructor, js_class = "VRSubmitFrameResult")]
    #[doc = "The `new VrSubmitFrameResult(..)` constructor, creating a new instance of `VrSubmitFrameResult`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRSubmitFrameResult/VRSubmitFrameResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrSubmitFrameResult`*"]
    pub fn new() -> Result<VrSubmitFrameResult, JsValue>;
}
